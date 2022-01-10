#[macro_export]
macro_rules! api_base_url_v1 {
    () => {
        concat!(api_base_url!(), "v1/")
    };
}

#[macro_export]
macro_rules! paper_search_endpoint {
    () => {
        concat!(api_base_url_v1!(), "paper/search")
    };
}

#[macro_export]
macro_rules! paper_endpoint {
    () => {
        concat!(api_base_url_v1!(), "paper/{paper_id}")
    };
}

#[macro_export]
macro_rules! paper_authors_endpoint {
    () => {
        concat!(paper_endpoint!(), "/authors")
    };
}

#[macro_export]
macro_rules! paper_references_endpoint {
    () => {
        concat!(paper_endpoint!(), "/references")
    };
}

#[macro_export]
macro_rules! paper_citations_endpoint {
    () => {
        concat!(paper_endpoint!(), "/citations")
    };
}

#[macro_export]
macro_rules! author_endpoint {
    () => {
        concat!(api_base_url_v1!(), "author/{author_id}")
    };
}

#[macro_export]
macro_rules! author_papers_endpoint {
    () => {
        concat!(author_endpoint!(), "/papers")
    };
}
