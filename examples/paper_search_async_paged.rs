use scholars::v1::definition::BasePaper;
use scholars::v1::endpoint::GetPaperSearch;
use scholars::v1::pagination::{Page, Pages};
use scholars::v1::query_params::PaperSearchParams;
use scholars::v1::utils::all_base_paper_fields;

use futures_util::TryStreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = reqwest::Client::new();
    let endpoint = GetPaperSearch::new(query_params());
    let pages = Pages::Limit(9_950);

    // Collecting into a `Result<Collection<T>, E>` will
    // stop the iteration at the first `E` type returned.
    let papers = endpoint.paged_async(pages, &client).try_collect::<Vec<BasePaper>>().await?;

    println!(
        "results:\n{}\nnumber of results: {}",
        serde_json::to_string_pretty(&papers).unwrap(),
        papers.len()
    );

    Ok(())
}

fn query_params() -> PaperSearchParams {
    PaperSearchParams::new(
        "covid".to_string(),
        Some(all_base_paper_fields()),
        Page::new(9_940, 7).unwrap(),
    )
}
