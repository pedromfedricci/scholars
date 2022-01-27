use scholars::v1::definition::FullPaper;
use scholars::v1::endpoint::GetPaper;
use scholars::v1::query_params::PaperParams;
use scholars::v1::utils::all_full_paper_fields;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = reqwest::blocking::Client::new();
    let endpoint = GetPaper::new(query_params(), paper_id());

    // A blocking query execution through the default reqwest client implementation.
    let paper: FullPaper = endpoint.query(&client)?;
    println!("{}", serde_json::to_string_pretty(&paper).unwrap());

    Ok(())
}

fn query_params() -> PaperParams {
    PaperParams::new(Some(all_full_paper_fields()))
}

fn paper_id() -> String {
    "649def34f8be52c8b66281af98ae884c09aef38b".to_owned()
}
