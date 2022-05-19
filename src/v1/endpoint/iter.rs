use std::marker::PhantomData;

use crate::endpoint::{Endpoint, EndpointResult};
use crate::v1::definition::{Batch, Batched, SearchBatch};
use crate::v1::pagination::{Page, Paged, Results};

#[cfg(feature = "async")]
pub(in crate::v1) use r#async::{BatchEndpointAsyncIter, SearchBatchEndpointAsyncIter};

#[cfg(feature = "blocking")]
pub(in crate::v1) use blocking::{BatchEndpointIter, SearchBatchEndpointIter};

#[derive(Debug)]
struct InnerEndpointIter<'c, T, E, C, B> {
    endpoint: E,
    client: &'c C,
    batch: B,
    results: Results,
    count: u64,
    // `batch` holds elements of type `T`.
    _marker: PhantomData<T>,
}

impl<T, E, C, B: Batched<T>> InnerEndpointIter<'_, T, E, C, B> {
    #[inline]
    fn update_current_page(&mut self, batch: B) {
        self.count = self.count.saturating_add(batch.len() as u64);
        self.batch = batch;
    }
}

impl<T, E: Paged, C, B: Batched<T>> InnerEndpointIter<'_, T, E, C, B> {
    #[inline]
    fn requested_limit(&mut self) -> Option<()> {
        if let Results::Limit(requested) = self.results {
            // If reached requested limit, stop iterating.
            if self.count >= requested {
                return None;
            }

            if let Some(next) = self.batch.get_next() {
                let mut limit = self.endpoint.get_limit();
                // Decrease next batch size if it over extends
                // the requested number of results.
                if next.saturating_add(limit) >= requested {
                    limit = requested.saturating_sub(next);
                    // If reached API enforced limit, stop iterating.
                    self.endpoint.set_limit(limit).ok()?;
                }
            }
        }
        // Results limit was not specified, keep iterating.
        Some(())
    }

    #[inline]
    fn next_page(&mut self) -> Option<()> {
        // Check if requested limit has been reached.
        self.requested_limit()?;
        if let Some(next) = self.batch.get_next() {
            // If reached API enforced limit, stop iterating.
            self.endpoint.next_page(next).ok()?;
            Some(())
        // No next value, all results were returned, stop iterating.
        } else {
            None
        }
    }
}

impl<'c, T, E: Paged, C, B: Batched<T>> InnerEndpointIter<'c, T, E, C, B> {
    fn new(endpoint: E, mut batch: B, results: Results, client: &'c C) -> Self {
        batch.set_next(Some(endpoint.get_offset()));
        Self { endpoint, results, client, batch, _marker: PhantomData, count: 0 }
    }
}

#[cfg(feature = "blocking")]
mod blocking {
    use super::*;
    use crate::{client::Client, query::Query};

    impl<T, E, C, B> Iterator for InnerEndpointIter<'_, T, E, C, B>
    where
        E: Endpoint + Paged + Query<B, E, C>,
        C: Client,
        B: Batched<T>,
    {
        type Item = EndpointResult<T, E, C>;

        // The iterator will keep yielding errors if the endpoint returns
        // them indefinitely, so it is up to the caller to treat it as they
        // see fit, like short-circuiting it by collecting into a Result.
        fn next(&mut self) -> Option<Self::Item> {
            if self.batch.as_ref().is_empty() {
                // Check requested results limit and then move to the next page.
                self.next_page()?;
                // Query the endpoint.
                match self.endpoint.query(self.client) {
                    Err(err) => return Some(Err(err)),
                    // Update current page results and control data.
                    Ok(batch) => self.update_current_page(batch),
                };
                // Reverse the results to `pop` in FIFO order.
                self.batch.as_mut().reverse();
            }
            // Else, return the next value from current page.
            self.batch.as_mut().pop().map(Ok)
        }
    }

    pub(in crate::v1) struct BatchEndpointIter<'c, T, E, C>(
        InnerEndpointIter<'c, T, E, C, Batch<T>>,
    );

    impl<'c, T, E: Paged, C> BatchEndpointIter<'c, T, E, C> {
        pub(in crate::v1) fn new(endpoint: E, results: Results, client: &'c C) -> Self {
            let batch = Batch::default();
            BatchEndpointIter(InnerEndpointIter::new(endpoint, batch, results, client))
        }
    }

    pub(in crate::v1) struct SearchBatchEndpointIter<'c, T, E, C>(
        InnerEndpointIter<'c, T, E, C, SearchBatch<T>>,
    );

    impl<'c, T, E: Paged, C> SearchBatchEndpointIter<'c, T, E, C> {
        pub(in crate::v1) fn new(endpoint: E, results: Results, client: &'c C) -> Self {
            let batch = SearchBatch::default();
            SearchBatchEndpointIter(InnerEndpointIter::new(endpoint, batch, results, client))
        }
    }

    impl<T, E, C> SearchBatchEndpointIter<'_, T, E, C> {
        pub(in crate::v1) fn total(&self) -> u64 {
            self.0.batch.total()
        }
    }

    impl<T, E, C> Iterator for BatchEndpointIter<'_, T, E, C>
    where
        E: Endpoint + Paged + Query<Batch<T>, E, C>,
        C: Client,
    {
        type Item = EndpointResult<T, E, C>;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }

    impl<T, E, C> Iterator for SearchBatchEndpointIter<'_, T, E, C>
    where
        E: Endpoint + Paged + Query<SearchBatch<T>, E, C>,
        C: Client,
    {
        type Item = EndpointResult<T, E, C>;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let count = self.0.count;
            let limit = self.0.batch.total().min(Page::RANGE_LIMIT);
            let remainder = limit.saturating_sub(count) as usize;
            (remainder, Some(remainder))
        }
    }
}

#[cfg(feature = "async")]
mod r#async {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    use futures_core::{ready, Stream};
    use pin_project::pin_project;

    use super::*;
    use crate::{client::AsyncClient, query::AsyncQuery};

    impl<T, E, C, B> InnerEndpointIter<'_, T, E, C, B>
    where
        E: Endpoint + Paged + AsyncQuery<B, E, C> + Sync,
        C: AsyncClient + Sync,
        B: Batched<T>,
    {
        // The iterator will keep yielding errors if the endpoint returns
        // them indefinitely, so it is up to the caller to treat it as they
        // see fit, like short-circuiting it by collecting into a Result.
        async fn next_async(&mut self) -> Option<EndpointResult<T, E, C>> {
            if self.batch.as_ref().is_empty() {
                // Check requested results limit and move to the next page.
                self.next_page()?;
                // Query the endpoint.
                match self.endpoint.query_async(self.client).await {
                    Err(err) => return Some(Err(err)),
                    // Update current page results and control data.
                    Ok(batch) => self.update_current_page(batch),
                };
                // Reverse the results to `pop` in FIFO order.
                self.batch.as_mut().reverse();
            }
            // Else, return the next value from current page.
            self.batch.as_mut().pop().map(Ok)
        }
    }

    type PinnedBoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;
    type FutureOutput<'c, T, E, C, B> =
        Option<(EndpointResult<T, E, C>, InnerEndpointIter<'c, T, E, C, B>)>;

    #[pin_project(project = StreamStateProj, project_replace = StreamStateProjReplace)]
    enum StreamState<'c, T, E: Endpoint, C: AsyncClient, B> {
        Inner { inner: InnerEndpointIter<'c, T, E, C, B> },
        Future { future: PinnedBoxFuture<'c, FutureOutput<'c, T, E, C, B>> },
        Empty,
    }

    impl<'c, T, E: Endpoint, C: AsyncClient, B> StreamState<'c, T, E, C, B> {
        fn project_future(
            self: Pin<&mut Self>,
        ) -> Option<&mut PinnedBoxFuture<'c, FutureOutput<'c, T, E, C, B>>> {
            match self.project() {
                StreamStateProj::Future { future } => Some(future),
                _ => None,
            }
        }

        fn take_value(self: Pin<&mut Self>) -> Option<InnerEndpointIter<'c, T, E, C, B>> {
            match &*self {
                StreamState::Inner { .. } => match self.project_replace(StreamState::Empty) {
                    StreamStateProjReplace::Inner { inner } => Some(inner),
                    _ => unreachable!(),
                },
                _ => None,
            }
        }
    }

    #[pin_project]
    struct EndpointStream<'c, T, E: Endpoint, C: AsyncClient, B> {
        #[pin]
        state: StreamState<'c, T, E, C, B>,
    }

    impl<'c, T, E: Endpoint + Paged, C: AsyncClient, B: Batched<T>> EndpointStream<'c, T, E, C, B> {
        fn new(endpoint: E, batch: B, results: Results, client: &'c C) -> Self {
            let inner = InnerEndpointIter::new(endpoint, batch, results, client);
            EndpointStream { state: StreamState::Inner { inner } }
        }
    }

    impl<'c, T: 'c, E: 'c, C, B: 'c> Stream for EndpointStream<'c, T, E, C, B>
    where
        E: Endpoint + Paged + AsyncQuery<B, E, C> + Sync,
        C: AsyncClient + Sync,
        B: Batched<T>,
    {
        type Item = EndpointResult<T, E, C>;

        fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            let mut this = self.project();

            if let Some(mut state) = this.state.as_mut().take_value() {
                this.state.set(StreamState::Future {
                    future: Box::pin(
                        async move { state.next_async().await.map(|item| (item, state)) },
                    ),
                });
            }

            let step = match this.state.as_mut().project_future() {
                Some(fut) => ready!(fut.as_mut().poll(cx)),
                None => panic!("Stream must not be polled after it returned `Poll::Ready(None)`"),
            };

            if let Some((item, next_state)) = step {
                this.state.set(StreamState::Inner { inner: next_state });
                Poll::Ready(Some(item))
            } else {
                this.state.set(StreamState::Empty);
                Poll::Ready(None)
            }
        }
    }

    pub(in crate::v1) struct BatchEndpointAsyncIter<'c, T, E: Endpoint, C: AsyncClient>(
        EndpointStream<'c, T, E, C, Batch<T>>,
    );

    impl<'c, T, E: Endpoint + Paged, C: AsyncClient> BatchEndpointAsyncIter<'c, T, E, C> {
        pub(in crate::v1) fn new(endpoint: E, results: Results, client: &'c C) -> Self {
            let batch = Batch::default();
            BatchEndpointAsyncIter(EndpointStream::new(endpoint, batch, results, client))
        }
    }

    impl<'c, T: 'c, E: 'c, C> Stream for BatchEndpointAsyncIter<'c, T, E, C>
    where
        E: Endpoint + Paged + AsyncQuery<Batch<T>, E, C> + Sync,
        C: AsyncClient + Sync,
    {
        type Item = EndpointResult<T, E, C>;

        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            Pin::new(&mut self.0).poll_next(cx)
        }
    }

    pub(in crate::v1) struct SearchBatchEndpointAsyncIter<'c, T, E: Endpoint, C: AsyncClient>(
        EndpointStream<'c, T, E, C, SearchBatch<T>>,
    );

    impl<'c, T, E: Endpoint + Paged, C: AsyncClient> SearchBatchEndpointAsyncIter<'c, T, E, C> {
        pub(in crate::v1) fn new(endpoint: E, results: Results, client: &'c C) -> Self {
            let batch = SearchBatch::default();
            SearchBatchEndpointAsyncIter(EndpointStream::new(endpoint, batch, results, client))
        }
    }

    impl<T, E: Endpoint, C: AsyncClient> SearchBatchEndpointAsyncIter<'_, T, E, C> {
        pub(in crate::v1) fn total(&self) -> u64 {
            match self.0.state {
                StreamState::Inner { ref inner } => inner.batch.total(),
                _ => 0,
            }
        }
    }

    impl<'c, T: 'c, E: 'c, C> Stream for SearchBatchEndpointAsyncIter<'c, T, E, C>
    where
        E: Endpoint + Paged + AsyncQuery<SearchBatch<T>, E, C> + Sync,
        C: AsyncClient + Sync,
    {
        type Item = EndpointResult<T, E, C>;

        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            Pin::new(&mut self.0).poll_next(cx)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            if let StreamState::Inner { ref inner } = self.0.state {
                let count = inner.count;
                let limit = inner.batch.total().min(Page::RANGE_LIMIT);
                let remainder = limit.saturating_sub(count) as usize;
                (remainder, Some(remainder))
            } else {
                (0, None)
            }
        }
    }
}
