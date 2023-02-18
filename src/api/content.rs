use serde::{Deserialize, Serialize};

use crate::models::CompletionResponse;

pub trait Content {
    fn generate(&self, prompt: &str, max_tokens: usize) -> Result<CompletionResponse, String>;
}

impl Content for crate::OpenAI {
    fn generate(&self, prompt: &str, max_tokens: usize) -> Result<CompletionResponse, String> {
        let model = "content-filter-alpha-001";
        self.generate::<CompletionResponse>(model, prompt, max_tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_generate() {
        let openai = crate::OpenAI::new("test_key".to_owned());
        let prompt = "This is a test prompt.";
        let max_tokens = 5;

        let result = openai.generate::<CompletionResponse>("content-filter-alpha-001", prompt, max_tokens);
        assert!(result.is_ok());
    }
}
