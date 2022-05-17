use scholars::v1::definition::AuthorWithPapers;
use scholars::v1::endpoint::GetPaperAuthors;
use scholars::v1::pagination::{Page, Results};
use scholars::v1::query_params::PaperAuthorsParams;
use scholars::v1::utils::author_with_papers_fields_with;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = reqwest::blocking::Client::new();
    let endpoint = GetPaperAuthors::new(query_params(), paper_id());
    let results = Results::Limit(68);

    // Collecting into a `Result<Collection<T>, E>` will
    // stop the iteration at the first `E` type returned.
    let authors = endpoint.paged(results, &client).collect::<Result<Vec<AuthorWithPapers>, _>>()?;

    println!(
        "results:\n{}\nnumber of results: {}",
        serde_json::to_string_pretty(&authors).unwrap(),
        authors.len()
    );

    // This can possibily yield errors indefinitely if
    // the API endpoint keeps returning them.
    //
    // let res: Result<Vec<AuthorWithPapers>, anyhow::Error>;
    // for res in endpoint.paged(results, &client) {
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
