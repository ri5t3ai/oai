use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GenerateRequest<'a> {
    pub prompt: &'a str,
    pub max_tokens: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub text: String,
    pub index: u32,
    pub logprobs: Option<Logprobs>,
    pub finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Logprobs {
    pub tokens: Vec<String>,
    pub token_logprobs: Vec<f64>,
    pub top_logprobs: Option<Vec<Vec<f64>>>,
    pub text_offset: u32,
}

#[derive(Debug, Deserialize)]
pub struct EmbeddingValue {
    pub vector: Vec<f32>,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct Embedding {
    pub embedding: Vec<f32>,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct EmbeddingsResponse {
    pub embeddings: Vec<Embedding>,
}

#[derive(Debug, Deserialize)]
pub struct Model {
    pub id: String,
    pub object: String,
    pub owner: Option<String>,
    pub created: i64,
    pub fine_tunable: bool,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ModelsResponse {
    pub data: Vec<Model>,
    pub object: String,
    pub model: Option<String>,
    pub id: Option<String>,
    pub created: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct TrainingResponse {
    pub id: String,
    pub model: String,
    pub created: i64,
    pub status: String,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TrainingStatusResponse {
    pub status: String,
    pub created: i64,
    pub loss: Option<f64>,
    pub message: Option<String>,
}
