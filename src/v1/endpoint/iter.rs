use crate::v1::definition::Batch;
use crate::v1::endpoint::{Endpoint, EndpointResult};
use crate::v1::pagination::{Paged, Pages};

#[derive(Debug)]
pub(in crate::v1) struct EndpointIter<'c, T, E, C> {
    endpoint: E,
    client: &'c C,
    current_page: Vec<T>,
    pages: Pages,
    next: Option<u64>,
    count: u64,
}

impl<T, E, C> EndpointIter<'_, T, E, C> {
    #[inline]
    fn update_current_page(&mut self, batch: Batch<T>) {
        self.count = self.count.saturating_add(batch.data.len() as u64);
        self.next = batch.next;
        self.current_page = batch.data;
    }
}

impl<T, E: Paged, C> EndpointIter<'_, T, E, C> {
    #[inline]
    fn requested(&mut self) -> Option<()> {
        if let Pages::Limit(requested) = self.pages {
            if self.count >= requested {
                return None;
            }

            if let Some(next) = self.next {
                let mut limit = self.endpoint.get_limit();
                if next.saturating_add(limit) >= requested {
                    limit = requested.saturating_sub(next);
                    self.endpoint.set_limit(limit).ok()?;
                }
            }
        }
        // Else, no results limit were specified,
        // so we don't have to do any boundaries checks.
        Some(())
    }

    #[inline]
    fn next_page(&mut self) -> Option<()> {
        // Check if reached requested results limit.
        self.requested()?;
        // Else, try to move to next page.
        if let Some(next) = self.next {
            self.endpoint.next_page(next).ok()?;
            Some(())
        } else {
            None
        }
    }
}

impl<'c, T, E: Paged, C> EndpointIter<'c, T, E, C> {
    pub(in crate::v1) fn new(endpoint: E, pages: Pages, client: &'c C) -> Self {
        let next = Some(endpoint.get_offset());
        Self { endpoint, pages, client, current_page: vec![], next, count: 0 }
    }
}

#[cfg(feature = "blocking")]
mod blocking {
    use super::*;
    use crate::{client::Client, query::Query};

    impl<T, E, C> Iterator for EndpointIter<'_, T, E, C>
    where
        E: Endpoint + Paged + Query<Batch<T>, E, C>,
        C: Client,
    {
        type Item = EndpointResult<T, E, C>;

        // The iterator will keep yielding errors if the endpoint returns
        // them indefinitely, so it is up to the caller to treat it as they
        // see fit, like short-circuiting it by collecting into a Result.
        fn next(&mut self) -> Option<Self::Item> {
            if self.current_page.is_empty() {
                // Check requested results limit and then move to the next page.
                self.next_page()?;
                // Query the endpoint.
                match self.endpoint.query(self.client) {
                    Err(err) => return Some(Err(err)),
                    Ok(batch @ Batch { .. }) => {
                        // Update current page results and control data.
                        self.update_current_page(batch);
                    }
                };
                // Reverse the results to `pop` in FIFO order.
                self.current_page.reverse();
            }
            // Else, return the next value from current page.
            self.current_page.pop().map(Ok)
        }
    }
}

#[cfg(feature = "async")]
mod r#async {
    use super::*;
    use crate::{client::AsyncClient, query::AsyncQuery};

    impl<T, E, C> EndpointIter<'_, T, E, C>
    where
        E: Endpoint + Paged + AsyncQuery<Batch<T>, E, C> + Sync,
        C: AsyncClient + Sync,
    {
        // The iterator will keep yielding errors if the endpoint returns
        // them indefinitely, so it is up to the caller to treat it as they
        // see fit, like short-circuiting it by collecting into a Result.
        pub(in crate::v1) async fn next_async(&mut self) -> Option<EndpointResult<T, E, C>> {
            if self.current_page.is_empty() {
                // Check requested results limit and move to the next page.
                self.next_page()?;
                // Query the endpoint.
                match self.endpoint.query_async(self.client).await {
                    Err(err) => return Some(Err(err)),
                    Ok(batch @ Batch { .. }) => {
                        // Update current page results and control data.
                        self.update_current_page(batch);
                    }
                };
                // Reverse the results to `pop` in FIFO order.
                self.current_page.reverse();
            }
            // Else, return the next value from current page.
            self.current_page.pop().map(Ok)
        }
    }
}
