use scholars::v1::definition::AuthorWithPapers;
use scholars::v1::endpoint::GetAuthor;
use scholars::v1::query_params::AuthorParams;
use scholars::v1::utils::all_author_with_papers_fields;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = reqwest::blocking::Client::new();
    let endpoint = GetAuthor::new(query_params(), author_id());

    // A blocking query execution through the default reqwest client implementation.
    let author: AuthorWithPapers = endpoint.query(&client)?;
    println!("{}", serde_json::to_string_pretty(&author).unwrap());

    Ok(())
}

fn query_params() -> AuthorParams {
    AuthorParams::new(Some(all_author_with_papers_fields()))
}

fn author_id() -> String {
    "1741101".to_owned()
}
