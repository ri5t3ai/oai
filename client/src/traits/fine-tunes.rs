use crate::models::TrainingResponse;

pub trait FineTunes {
    fn create_training(
        &self,
        training_data: &[&str],
        validation_data: &[&str],
        name: &str,
        model_id: Option<&str>,
        options: Option<std::collections::HashMap<&str, &str>>,
    ) -> Result<TrainingResponse, String>;
}

impl FineTunes for crate::OpenAI {
    fn create_training(
        &self,
        training_data: &[&str],
        validation_data: &[&str],
        name: &str,
        model_id: Option<&str>,
        options: Option<std::collections::HashMap<&str, &str>>,
    ) -> Result<TrainingResponse, String> {
        let model = "davinci-codex";
        self.create_training(model, training_data, validation_data, name, model_id, options)
    }
}