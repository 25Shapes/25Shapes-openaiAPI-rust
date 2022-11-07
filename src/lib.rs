mod data;
use std::io::{Error, ErrorKind};

use data::CompletionRequest;

pub async fn async_completions(
    key: String,
    model: data::Model,
    messages: Vec<data::Message>,
) -> Result<data::CompletionResponse, Error> {
    let model_name = model.as_str().to_string();
    let request_body = CompletionRequest {
        model: model_name,
        messages,
    };
    let request_body_json = serde_json::json!(request_body).to_owned().to_string();
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", key))
        .header("Content-Type", "application/json")
        .body(request_body_json)
        .send();

    match res.await {
        Ok(r) => match r.text().await {
            Ok(body_str) => {
                println!("{}", body_str);
                match serde_json::from_str(&body_str) {
                    Ok(response) => {
                        return Ok(response);
                    }
        