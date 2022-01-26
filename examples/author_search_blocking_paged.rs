use scholars::v1::definition::AuthorWithPapers;
use scholars::v1::endpoint::GetAuthorSearch;
use scholars::v1::pagination::{Page, Pages};
use scholars::v1::query_params::AuthorSearchParams;
use scholars::v1::utils::all_author_with_papers_fields;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = reqwest::blocking::Client::new();
    let endpoint = GetAuthorSearch::new(query_params());
    let pages = Pages::Limit(9_950);

    // Collecting into a `Result<Collection<T>, E>` will
    // stop the iteration at the first `E` type returned.
    let papers = endpoint.paged(pages, &client).collect::<Result<Vec<AuthorWithPapers>, _>>()?;

    println!(
        "results:\n{}\nnumber of results: {}",
        serde_json::to_string_pretty(&papers).unwrap(),
        papers.len()
    );

    Ok(())
}

fn query_params() -> AuthorSearchParams {
    AuthorSearchParams::new(
        "adam smith".to_string(),
        Some(all_author_with_papers_fields()),
        Page::new(450, 24).unwrap(),
    )
}
