
use openai_api::{OpenAI, CompletionResponse};
use clap::{Parser};
use std::env;
#[derive(Parser)]
#[clap(name = "openai-cli", version = "0.1", author, about = "A CLI for the OpenAI API")]
struct Cli {
    /// Your OpenAI API key
    #[clap(short, long)]
    api_key: Option<String>,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    /// Generates text using the OpenAI API
    Generate {
        /// The name of the GPT model to use
        #[clap(short, long)]
        model: String,

        /// The prompt to generate text from
        #[clap(short, long)]
        prompt: String,

        /// The maximum number of tokens to generate
        #[clap(short, long)]
        max_tokens: usize,
    },
}
#[tokio::main]
async fn main() {
    loop {
        let cli = Cli::parse();
        let api_key = cli.api_key.unwrap_or_else(|| env::var("OPENAI_API_KEY").unwrap());
        match cli.subcmd {
            SubCommand::Generate { model, prompt, max_tokens } => {
                let openai = OpenAI::new(api_key);
                let response = openai.generate::<CompletionResponse>(&model, &prompt, max_tokens)
                      .await
                      .unwrap_or_else(|err| panic!("Error generating text: {}", err));
                println!("{:#?}", response.choices);
                println!("Generating text with model '{}', prompt '{}', and max tokens '{}'", model, prompt, max_tokens);
            }
        }
    }
}