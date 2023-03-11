use client::OpenAI;
use log::info;

use dotenv::dotenv;
use serde_json::Value;
#[tokio::main]
async fn main() {
    dotenv().ok();

    fern::Dispatch::new()
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set.");
    let openai = OpenAI::new(api_key.to_owned());

    let result: Value = openai.list_models().await.unwrap();
    info!("List models example result: {:#?}", result);
}
