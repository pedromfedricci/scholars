use scholars::v1::definition::Citation;
use scholars::v1::endpoint::GetPaperCitations;
use scholars::v1::pagination::{Page, Results};
use scholars::v1::query_params::PaperCitationsParams;
use scholars::v1::utils::paper_fields_with;

use futures_util::TryStreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = reqwest::Client::new();
    let endpoint = GetPaperCitations::new(query_params(), paper_id());
    let results = Results::Limit(98);

    // Collecting into a `Result<Collection<T>, E>` will
    // stop the iteration at the first `E` type returned.
    let citations = endpoint.paged_async(results, &client).try_collect::<Vec<Citation>>().await?;

    println!(
        "results:\n{}\nnumber of results: {}",
        serde_json::to_string_pretty(&citations).unwrap(),
        citations.len()
    );

    // This can possibily yield errors indefinitely if
    // the API endpoint keeps returning them.
    //
    // let res: Result<Vec<Citation>, anyhow::Error>;
    // for res in endpoint.paged(results, &client) {
    //     println!("{:#?}", res)
    // }

    Ok(())
}

fn query_params() -> PaperCitationsParams {
    PaperCitationsParams::new(Some(paper_fields_with(std::iter::empty())), Page::default())
}

fn paper_id() -> String {
    "649def34f8be52c8b66281af98ae884c09aef38b".to_owned()
}
