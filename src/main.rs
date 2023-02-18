use openai_api::{OpenAI, CompletionResponse};


#[tokio::main]
async fn main() {
    let api_key = "YOUR_API_KEY_HERE";
    let openai = OpenAI::new(api_key.to_owned());

    let prompt = "Q: What is the capital of France?\nA:";
    let model = "davinci";
    let max_tokens = 5;
    let result: Result<CompletionResponse, String> = openai.generate(model, prompt, max_tokens).await;

    match result {
        Ok(response) => {
            for choice in response.choices {
                println!("{}", choice.text);
            }
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}