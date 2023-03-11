use client::{
    EmbeddingsResponse, GenerateResponse, ModelsResponse, OpenAI, TrainingResponse,
    TrainingStatusResponse,
};
use log::{debug, error, info, trace, warn};

use dotenv::dotenv;
use serde_json::Value;
#[tokio::main]
async fn main() {
 dotenv().ok();

    fern::Dispatch::new().chain(std::io::stdout()).apply().unwrap();

    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set.");
    let openai = OpenAI::new(api_key.to_owned());

    let result: Value = openai.list_models().await.unwrap();
    info!("List models example result: {:#?}", result);
    // Generate example
    // let model = "davinci";
    // let prompt = "Hello, world!";
    // let max_tokens = 5;
    // let result: GenerateResponse = openai.generate(model, prompt, max_tokens).await.unwrap();
    // println!("Generate example result: {:#?}", result);

    // List models example


    // // Get embeddings example
    // let model = "davinci";
    // let texts = &["Hello", "world"];
    // let result: EmbeddingsResponse = openai.get_embeddings(model, texts).await.unwrap();
    // println!("Get embeddings example result: {:?}", result);

    // // Get training logs example
    // let model = "davinci";
    // let training_id = "YOUR_TRAINING_ID";
    // let result: String = openai.get_training_logs(model, training_id).await.unwrap();
    // println!("Get training logs example result: {:?}", result);

    // // Check training status example
    // let model = "davinci";
    // let training_id = "YOUR_TRAINING_ID";
    // let result: TrainingStatusResponse = openai
    //     .check_training_status(model, training_id)
    //     .await
    //     .unwrap();
    // println!("Check training status example result: {:?}", result);

    // // Create training example
    // let model = "davinci";
    // let training_data = &["Example training data"];
    // let validation_data = &["Example validation data"];
    // let name = "Example training";
    // let result: TrainingResponse = openai
    //     .create_training(model, training_data, validation_data, name, None, None)
    //     .await
    //     .unwrap();
    // println!("Create training example result: {:?}", result);
}
// fn setup_logger() -> Result<(), fern::InitError> {
//     fern::Dispatch::new()
//         .format(|out, message, record| {
//             out.finish(format_args!(
//                 "{} [{}][{}] {}",
//                 chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
//                 record.target(),
//                 record.level(),
//                 serde_json::to_string_pretty(message).unwrap()
//             ))
//         })
//         .level(log::LevelFilter::Debug)
//         .chain(std::io::stdout())
//         .chain(fern::log_file("output.log")?)
//         .apply()?;
//     Ok(())
// }
