mod api;
mod models;

use std::collections::HashMap;

pub use models::{CompletionResponse, Choice, Logprobs, GenerateRequest, GenerateResponse, EmbeddingValue, Embedding, EmbeddingsResponse, Model, ModelsResponse, TrainingResponse, TrainingStatusResponse};

use reqwest::Client;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json;
pub struct OpenAI {
    api_key: String,
    client: Client,
    base_url: String,
}

impl OpenAI {
pub fn new(api_key: String) -> OpenAI {
    OpenAI {
        api_key: api_key,
        client: Client::new(),
        base_url: "https://api.openai.com/v1/".to_owned(),
    }
}

    pub fn set_base_url(&mut self, base_url: &str) {
        self.base_url = base_url.to_owned();
    }

    pub fn set_api_key(&mut self, api_key: &str) {
        self.api_key = api_key.to_owned();
    }

    fn build_url(&self, endpoint: &str) -> String {
        format!("{}{}", self.base_url, endpoint)
    }

pub async fn generate<T>(&self, model: &str, prompt: &str, max_tokens: usize) -> Result<T, String>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    let endpoint = format!("engines/{}/completions", model);
    let url = self.build_url(&endpoint);

    #[derive(Serialize)]
    struct Data<'a> {
        prompt: &'a str,
        max_tokens: usize,
        engine: &'a str,
    }

    let data = Data {
        prompt,
        max_tokens,
        engine: model,
    };

    let body = match serde_json::to_string(&data) {
    Ok(body) => body,
    Err(e) => return Err(e.to_string()),
};

let response = self.client.post(&url)
    .header("Authorization", format!("Bearer {}", self.api_key))
    .header("Content-Type", "application/json")
    .body(body)
    .send()
    .await
    .expect("Request failed");


    let status = response.status();
    let body = response.text().await.map_err(|err| err.to_string())?;

 let response_body: T = serde_json::from_str(&body)
    .unwrap_or_else(|err| panic!("Error parsing response body: {}", err));

    if status.is_success() {
        Ok(response_body)
    } else {
        Err(format!("Request failed with status code: {}, response body: {:?}", status, response_body))
    }
}

   pub async fn list_models(&self) -> Result<ModelsResponse, String> {
        let endpoint = "models";
        let url = self.build_url(&endpoint);

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await;

        let response = match response {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };

        let status = response.status();
        let body = response.text().await.map_err(|err| err.to_string())?;

        let response_body: ModelsResponse = serde_json::from_str(&body).map_err(|err| err.to_string())?;

        if status.is_success() {
            Ok(response_body)
        } else {
            Err(format!("Request failed with status code: {}, response body: {:?}", status, response_body))
        }
    }
     pub async fn get_embeddings(&self, model: &str, texts: &[&str]) -> Result<EmbeddingsResponse, String> {
        let endpoint = format!("engines/{}/embeddings", model);
        let url = self.build_url(&endpoint);

        #[derive(Serialize)]
        struct Data<'a> {
            documents: &'a [&'a str],
        }

        let data = Data { documents: texts };

        let body = serde_json::to_string(&data).unwrap();

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await;

        let response = match response {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };

        let status = response.status();
        let body = response.text().await.map_err(|err| err.to_string())?;

        let response_body: EmbeddingsResponse = serde_json::from_str(&body).map_err(|err| err.to_string())?;

        if status.is_success() {
            Ok(response_body)
        } else {
            Err(format!("Request failed with status code: {}, response body: {:?}", status, response_body))
        }
    }
   pub async fn get_training_logs(&self, model: &str, training_id: &str) -> Result<String, String> {
        let endpoint = format!("engines/{}/training/{}/logs", model, training_id);
        let url = self.build_url(&endpoint);

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await;

        let response = match response {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };

        let status = response.status();
        let body = response.text().await.map_err(|err| err.to_string())?;

        if status.is_success() {
            Ok(body)
        } else {
            Err(format!("Request failed with status code: {}, response body: {:?}", status, body))
        }
    }
        pub async fn check_training_status(&self, model: &str, training_id: &str) -> Result<TrainingStatusResponse, String> {
        let endpoint = format!("engines/{}/training/{}", model, training_id);
        let url = self.build_url(&endpoint);

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await;

        let response = match response {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };

        let status = response.status();
        let body = response.text().await.map_err(|err| err.to_string())?;

        let response_body: TrainingStatusResponse = serde_json::from_str(&body).map_err(|err| err.to_string())?;

        if status.is_success() {
            Ok(response_body)
        } else {
            Err(format!("Request failed with status code: {}, response body: {:?}", status, response_body))
        }
    }
    pub async fn create_training(
        &self,
        model: &str,
        training_data: &[&str],
        validation_data: &[&str],
        name: &str,
        model_id: Option<&str>,
        options: Option<HashMap<&str, &str>>,
    ) -> Result<TrainingResponse, String> {
        let endpoint = format!("engines/{}/train", model);
        let url = self.build_url(&endpoint);

        #[derive(Serialize)]
        struct TrainingData<'a> {
            text: &'a [&'a str],
        }

        #[derive(Serialize)]
        struct Data<'a> {
            data: TrainingData<'a>,
            name: &'a str,
            model: Option<&'a str>,
            validation_data: Option<TrainingData<'a>>,
            #[serde(flatten)]
            options: Option<HashMap<&'a str, &'a str>>,
        }

        let data = Data {
            data: TrainingData { text: training_data },
            name,
            model: model_id,
            validation_data: Some(TrainingData { text: validation_data }),
            options,
        };

        let body = serde_json::to_string(&data).unwrap();

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await;

        let response = match response {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };

        let status = response.status();
        let body = response.text().await.map_err(|err| err.to_string())?;

        let response_body: TrainingResponse = serde_json::from_str(&body).map_err(|err| err.to_string())?;

        if status.is_success() {
            Ok(response_body)
        } else {
            Err(format!("Request failed with status code: {}, response body: {:?}", status, response_body))
        }
    }
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
