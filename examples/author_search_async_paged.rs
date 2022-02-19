use scholars::v1::definition::AuthorWithPapers;
use scholars::v1::endpoint::GetAuthorSearch;
use scholars::v1::pagination::{Page, Pages};
use scholars::v1::query_params::AuthorSearchParams;
use scholars::v1::utils::all_author_with_papers_fields;

use futures_util::TryStreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = reqwest::Client::new();
    let endpoint = GetAuthorSearch::new(query_params());
    let pages = Pages::Limit(10);

    // Collecting into a `Result<Collection<T>, E>` will
    // stop the iteration at the first `E` type returned.
    let papers =
        endpoint.paged_async(pages, &client).try_collect::<Vec<AuthorWithPapers>>().await?;

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
        Page::default(),
    )
}
