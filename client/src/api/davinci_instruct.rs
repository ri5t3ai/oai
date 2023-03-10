use serde::{Deserialize, Serialize};

use crate::models::CompletionResponse;

pub trait DavinciInstruct {
    fn generate(&self, prompt: &str, max_tokens: usize) -> Result<CompletionResponse, String>;
}

impl DavinciInstruct for crate::OpenAI {
    fn generate(&self, prompt: &str, max_tokens: usize) -> Result<CompletionResponse, String> {
        let model = "davinci-instruct-beta";
        self.generate::<CompletionResponse>(model, prompt, max_tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_davinci_instruct_generate() {
        let openai = crate::OpenAI::new("test_key".to_owned());
        let prompt = "How do you create a list in Python?";
        let max_tokens = 5;

        let result = openai.generate::<CompletionResponse>("davinci-instruct-beta", prompt, max_tokens);
        assert!(result.is_ok());
    }
}
