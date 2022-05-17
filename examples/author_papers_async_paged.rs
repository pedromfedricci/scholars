use scholars::v1::definition::PaperWithLinks;
use scholars::v1::endpoint::GetAuthorPapers;
use scholars::v1::pagination::{Page, Results};
use scholars::v1::query_params::AuthorPapersParams;
use scholars::v1::utils::all_paper_with_links_fields;

use futures_util::TryStreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = reqwest::Client::new();
    let endpoint = GetAuthorPapers::new(query_params(), author_id());
    let results = Results::Limit(68);

    // Collecting into a `Result<Collection<T>, E>` will
    // stop the iteration at the first `E` type returned.
    let papers =
        endpoint.paged_async(results, &client).try_collect::<Vec<PaperWithLinks>>().await?;

    println!(
        "results:\n{}\nnumber of results: {}",
        serde_json::to_string_pretty(&papers).unwrap(),
        papers.len()
    );

    // This can possibily return the same error indefinitely if
    // the endpoint keeps returning it.
    //
    // let res: Result<Vec<AuthorWithPapers>, anyhow::Error>;
    // for res in endpoint.paged(results, &client) {
    //     println!("{:#?}", res)
    // }

    Ok(())
}

fn query_params() -> AuthorPapersParams {
    AuthorPapersParams::new(Some(all_paper_with_links_fields()), Page::default())
}

fn author_id() -> String {
    "1741101".to_owned()
}
