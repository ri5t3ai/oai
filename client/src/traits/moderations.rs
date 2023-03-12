use crate::models::ModerationResponse;

pub trait TextModeration {
    fn moderate_text(&self, text: &str) -> Result<ModerationResponse, String>;
}

impl TextModeration for crate::OpenAI {
    fn moderate_text(&self, text: &str) -> Result<ModerationResponse, String> {
        let model = "text-moderation-latest";
        self.generate::<ModerationResponse>(model, text, 1)
    }
}
