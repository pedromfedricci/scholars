use scholars::v1::definition::BasePaper;
use scholars::v1::endpoint::GetPaperSearch;
use scholars::v1::pagination::Page;
use scholars::v1::query_params::PaperSearchParams;
use scholars::v1::utils::all_base_paper_fields;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = reqwest::blocking::Client::new();
    let paper_search = GetPaperSearch::new(query_params());

    // A blocking query execution through the default reqwest client implementation.
    let papers: BasePaper = paper_search.query(&client)?;
    println!("{}", serde_json::to_string_pretty(&papers).unwrap());

    Ok(())
}

fn query_params() -> PaperSearchParams {
    PaperSearchParams::new("covid".to_string(), Some(all_base_paper_fields()), Page::default())
}
