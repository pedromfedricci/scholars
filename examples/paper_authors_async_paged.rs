use scholars::v1::definition::AuthorWithPapers;
use scholars::v1::endpoint::GetPaperAuthors;
use scholars::v1::pagination::{Page, Pages};
use scholars::v1::query_params::PaperAuthorsParams;
use scholars::v1::utils::author_with_papers_fields_with;

use futures_util::TryStreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = reqwest::Client::new();
    let endpoint = GetPaperAuthors::new(query_params(), paper_id());
    let pages = Pages::Limit(68);

    // Collecting into a `Result<Collection<T>, E>` will
    // stop the iteration at the first `E` type returned.
    let authors =
        endpoint.paged_async(pages, &client).try_collect::<Vec<AuthorWithPapers>>().await?;

    println!(
        "results:\n{}\nnumber of results: {}",
        serde_json::to_string_pretty(&authors).unwrap(),
        authors.len()
    );

    // This can possibily yield errors indefinitely if
    // the API endpoint keeps returning them.
    //
    // let res: Result<Vec<AuthorWithPapers>, anyhow::Error>;
    // for res in endpoint.paged(pages, &client) {
    //     println!("{:#?}", res)
    // }

    Ok(())
}

fn query_params() -> PaperAuthorsParams {
    PaperAuthorsParams::new(
        Some(author_with_papers_fields_with(std::iter::empty())),
        Page::default(),
    )
}

fn paper_id() -> String {
    "649def34f8be52c8b66281af98ae884c09aef38b".to_owned()
}
