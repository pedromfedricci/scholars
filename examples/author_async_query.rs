use scholars::v1::definition::AuthorWithPapers;
use scholars::v1::endpoint::GetAuthor;
use scholars::v1::query_params::AuthorParams;
use scholars::v1::utils::all_author_with_papers_fields;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = reqwest::Client::new();
    let endpoint = GetAuthor::new(query_params(), author_id());

    // An async query execution through the default reqwest client implementation.
    let author: AuthorWithPapers = endpoint.query_async(&client).await?;
    println!("{}", serde_json::to_string_pretty(&author).unwrap());

    Ok(())
}

fn query_params() -> AuthorParams {
    AuthorParams::new(Some(all_author_with_papers_fields()))
}

fn author_id() -> String {
    "1741101".to_owned()
}
