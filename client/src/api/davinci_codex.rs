use serde::{Deserialize, Serialize};

use crate::models::CompletionResponse;

pub trait DavinciCodex {
    fn generate(&self, prompt: &str, max_tokens: usize) -> Result<CompletionResponse, String>;
}

impl DavinciCodex for crate::OpenAI {
    fn generate(&self, prompt: &str, max_tokens: usize) -> Result<CompletionResponse, String> {
        let model = "davinci-codex";
        self.generate::<CompletionResponse>(model, prompt, max_tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_davinci_codex_generate() {
        let openai = crate::OpenAI::new("test_key".to_owned());
        let prompt = "Q: How do you define a function in Rust?\nA:";
        let max_tokens = 5;

        let result = openai.generate::<CompletionResponse>("davinci-codex", prompt, max_tokens);
        assert!(result.is_ok());
    }
}
