use scholars::v1::definition::BasePaper;
use scholars::v1::endpoint::GetPaperSearch;
use scholars::v1::pagination::Page;
use scholars::v1::query_params::PaperSearchParams;
use scholars::v1::utils::all_base_paper_fields;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = reqwest::Client::new();
    let endpoint = GetPaperSearch::new(query_params());

    // An async query execution through the default reqwest client implementation.
    let papers: BasePaper = endpoint.query_async(&client).await?;
    println!("{}", serde_json::to_string_pretty(&papers).unwrap());

    Ok(())
}

fn query_params() -> PaperSearchParams {
    PaperSearchParams::new("covid".to_string(), Some(all_base_paper_fields()), Page::default())
}
