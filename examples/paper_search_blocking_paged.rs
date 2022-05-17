use scholars::v1::definition::BasePaper;
use scholars::v1::endpoint::GetPaperSearch;
use scholars::v1::pagination::{Page, Results};
use scholars::v1::query_params::PaperSearchParams;
use scholars::v1::utils::all_base_paper_fields;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = reqwest::blocking::Client::new();
    let endpoint = GetPaperSearch::new(query_params());
    let results = Results::Limit(9_950);

    // Collecting into a `Result<Collection<T>, E>` will
    // stop the iteration at the first `E` type returned.
    let papers = endpoint.paged(results, &client).collect::<Result<Vec<BasePaper>, _>>()?;

    println!(
        "results:\n{}\nnumber of results: {}",
        serde_json::to_string_pretty(&papers).unwrap(),
        papers.len()
    );

    // This can possibily return the same error indefinitely if
    // the endpoint keeps returning it.
    //
    // let res: Result<Vec<BasePaper>, anyhow::Error>;
    // for res in endpoint.paged(results, &client) {
    //     println!("{:#?}", res)
    // }

    Ok(())
}

fn query_params() -> PaperSearchParams {
    PaperSearchParams::new(
        "covid".to_string(),
        Some(all_base_paper_fields()),
        Page::new(9_940, 7).unwrap(),
    )
}
