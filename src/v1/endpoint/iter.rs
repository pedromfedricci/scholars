use std::marker::PhantomData;

use crate::v1::definition::{Batch, Batched, SearchBatch};
use crate::v1::endpoint::{Endpoint, EndpointResult};
use crate::v1::pagination::{Paged, Pages};

#[derive(Debug)]
struct EndpointIter<'c, T, E, C, B> {
    endpoint: E,
    client: &'c C,
    batch: B,
    pages: Pages,
    count: u64,
    _marker: PhantomData<T>,
}

pub(in crate::v1) struct BatchEndpontIter<'c, T, E, C>(EndpointIter<'c, T, E, C, Batch<T>>);

impl<'c, T, E: Paged, C> BatchEndpontIter<'c, T, E, C> {
    pub(in crate::v1) fn new(endpoint: E, pages: Pages, client: &'c C) -> Self {
        let batch = Batch::default();
        BatchEndpontIter(EndpointIter::new(endpoint, batch, pages, client))
    }
}

pub(in crate::v1) struct SearchBatchEndpontIter<'c, T, E, C>(
    EndpointIter<'c, T, E, C, SearchBatch<T>>,
);

impl<'c, T, E: Paged, C> SearchBatchEndpontIter<'c, T, E, C> {
    pub(in crate::v1) fn new(endpoint: E, pages: Pages, client: &'c C) -> Self {
        let batch = SearchBatch::default();
        SearchBatchEndpontIter(EndpointIter::new(endpoint, batch, pages, client))
    }
}

impl<T, E, C, B: Batched<T> + Default> EndpointIter<'_, T, E, C, B> {
    #[inline]
    fn update_current_page(&mut self, batch: B) {
        self.count = self.count.saturating_add(batch.len() as u64);
        self.batch = batch;
    }
}

impl<T, E: Paged, C, B: Batched<T>> EndpointIter<'_, T, E, C, B> {
    #[inline]
    fn requested_limit(&mut self) -> Option<()> {
        if let Pages::Limit(requested) = self.pages {
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

impl<'c, T, E: Paged, C, B: Batched<T>> EndpointIter<'c, T, E, C, B> {
    fn new(endpoint: E, mut batch: B, pages: Pages, client: &'c C) -> Self {
        batch.set_next(Some(endpoint.get_offset()));
        Self { endpoint, pages, client, batch, _marker: PhantomData, count: 0 }
    }
}

#[cfg(feature = "blocking")]
mod blocking {
    use super::*;
    use crate::{client::Client, query::Query, v1::pagination::Page};

    impl<T, E, C, B> Iterator for EndpointIter<'_, T, E, C, B>
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
                    Ok(batch) => {
                        // Update current page results and control data.
                        self.update_current_page(batch);
                    }
                };
                // Reverse the results to `pop` in FIFO order.
                self.batch.as_mut().reverse();
            }
            // Else, return the next value from current page.
            self.batch.as_mut().pop().map(Ok)
        }
    }

    impl<T, E, C> Iterator for BatchEndpontIter<'_, T, E, C>
    where
        E: Endpoint + Paged + Query<Batch<T>, E, C>,
        C: Client,
    {
        type Item = EndpointResult<T, E, C>;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }

    impl<T, E, C> Iterator for SearchBatchEndpontIter<'_, T, E, C>
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

    impl<T, E, C> SearchBatchEndpontIter<'_, T, E, C> {
        pub(in crate::v1) fn total(&self) -> u64 {
            self.0.batch.total()
        }
    }
}

#[cfg(feature = "async")]
mod r#async {
    use super::*;
    use crate::{client::AsyncClient, query::AsyncQuery};

    impl<T, E, C, B: Batched<T>> EndpointIter<'_, T, E, C, B>
    where
        E: Endpoint + Paged + AsyncQuery<B, E, C> + Sync,
        C: AsyncClient + Sync,
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
                    Ok(batch) => {
                        // Update current page results and control data.
                        self.update_current_page(batch);
                    }
                };
                // Reverse the results to `pop` in FIFO order.
                self.batch.as_mut().reverse();
            }
            // Else, return the next value from current page.
            self.batch.as_mut().pop().map(Ok)
        }
    }

    impl<T, E, C> BatchEndpontIter<'_, T, E, C>
    where
        E: Endpoint + Paged + AsyncQuery<Batch<T>, E, C> + Sync,
        C: AsyncClient + Sync,
    {
        async fn next_async(&mut self) -> Option<EndpointResult<T, E, C>> {
            self.0.next_async().await
        }
    }

    impl<'c, T: 'c, E: 'c, C> BatchEndpontIter<'c, T, E, C>
    where
        E: Endpoint + Paged + AsyncQuery<Batch<T>, E, C> + Sync,
        C: AsyncClient + Sync,
    {
        pub(in crate::v1) fn into_async_iter(
            self,
        ) -> impl futures_util::Stream<Item = EndpointResult<T, E, C>> + 'c {
            futures_util::stream::unfold(self, |mut iter| async move {
                iter.next_async().await.map(|item| (item, iter))
            })
        }
    }

    impl<T, E, C> SearchBatchEndpontIter<'_, T, E, C>
    where
        E: Endpoint + Paged + AsyncQuery<SearchBatch<T>, E, C> + Sync,
        C: AsyncClient + Sync,
    {
        async fn next_async(&mut self) -> Option<EndpointResult<T, E, C>> {
            self.0.next_async().await
        }
    }

    impl<'c, T: 'c, E: 'c, C> SearchBatchEndpontIter<'c, T, E, C>
    where
        E: Endpoint + Paged + AsyncQuery<SearchBatch<T>, E, C> + Sync,
        C: AsyncClient + Sync,
    {
        pub(in crate::v1) fn into_async_iter(
            self,
        ) -> impl futures_util::Stream<Item = EndpointResult<T, E, C>> + 'c {
            futures_util::stream::unfold(self, |mut iter| async move {
                iter.next_async().await.map(|item| (item, iter))
            })
        }
    }
}
