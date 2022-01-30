use scholars::v1::definition::Reference;
use scholars::v1::endpoint::GetPaperReferences;
use scholars::v1::pagination::{Page, Pages};
use scholars::v1::query_params::PaperReferencesParams;
use scholars::v1::utils::paper_fields_with;

use futures_util::TryStreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = reqwest::Client::new();
    let endpoint = GetPaperReferences::new(query_params(), paper_id());
    let pages = Pages::Limit(98);

    // Collecting into a `Result<Collection<T>, E>` will
    // stop the iteration at the first `E` type returned.
    let references = endpoint.paged_async(pages, &client).try_collect::<Vec<Reference>>().await?;

    println!(
        "results:\n{}\nnumber of results: {}",
        serde_json::to_string_pretty(&references).unwrap(),
        references.len()
    );

    // This can possibily yield errors indefinitely if
    // the API endpoint keeps returning them.
    //
    // let res: Result<Vec<Reference>, anyhow::Error>;
    // for res in endpoint.paged(pages, &client) {
    //     println!("{:#?}", res)
    // }

    Ok(())
}

fn query_params() -> PaperReferencesParams {
    PaperReferencesParams::new(Some(paper_fields_with(std::iter::empty())), Page::default())
}

fn paper_id() -> String {
    "649def34f8be52c8b66281af98ae884c09aef38b".to_owned()
}
