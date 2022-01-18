macro_rules! api_base_url_v1 {
    () => {
        concat!(api_base_url!(), "v1/")
    };
}

macro_rules! paper_search_endpoint {
    () => {
        concat!(api_base_url_v1!(), "paper/search")
    };
}

macro_rules! paper_endpoint {
    () => {
        concat!(api_base_url_v1!(), "paper/{paper_id}")
    };
}

macro_rules! paper_authors_endpoint {
    () => {
        concat!(paper_endpoint!(), "/authors")
    };
}

macro_rules! paper_references_endpoint {
    () => {
        concat!(paper_endpoint!(), "/references")
    };
}

macro_rules! paper_citations_endpoint {
    () => {
        concat!(paper_endpoint!(), "/citations")
    };
}

macro_rules! author_search_endpoint {
    () => {
        concat!(api_base_url_v1!(), "author/search")
    };
}

macro_rules! author_endpoint {
    () => {
        concat!(api_base_url_v1!(), "author/{author_id}")
    };
}

macro_rules! author_papers_endpoint {
    () => {
        concat!(author_endpoint!(), "/papers")
    };
}

pub(super) fn paper_search_endpoint() -> String {
    paper_search_endpoint!().to_string()
}

pub(super) fn paper_endpoint(paper_id: &str) -> String {
    format!(paper_endpoint!(), paper_id = paper_id)
}

pub(super) fn paper_authors_endpoint(paper_id: &str) -> String {
    format!(paper_authors_endpoint!(), paper_id = paper_id)
}

pub(super) fn paper_references_endpoint(paper_id: &str) -> String {
    format!(paper_references_endpoint!(), paper_id = paper_id)
}

pub(super) fn paper_citations_endpoint(paper_id: &str) -> String {
    format!(paper_citations_endpoint!(), paper_id = paper_id)
}

pub(super) fn author_search_endpoint() -> String {
    author_search_endpoint!().to_string()
}

pub(super) fn author_endpoint(author_id: &str) -> String {
    format!(author_endpoint!(), author_id = author_id)
}

pub(super) fn author_papers_endpoint(author_id: &str) -> String {
    format!(author_papers_endpoint!(), author_id = author_id)
}
