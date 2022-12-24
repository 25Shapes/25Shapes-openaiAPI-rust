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
                    Err(e) => return Err(Error::new(ErrorKind::Other, e.to_string())),
                };
            }
            Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
        },
        Err(e) => return Err(Error::new(ErrorKind::Other, e.to_string())),
    }
}

pub async fn async_transcribe(
    key: String,
    file: Vec<u8>,
    file_name: String,
) -> Result<data::TranscriptionResponse, Error> {
    let file_part = reqwest::multipart::Part::bytes(file).file_name(file_name);
    let client = reqwest::Client::new();
    let form = reqwest::multipart::Form::new()
        .part("file", file_part)
        .text("model", "whisper-1");
    let res = client
        .post("https://api.openai.com/v1/audio/transcriptions")
        .multipart(form)
        .header("Authorization", format!("Bearer {}", key))
        .header("Content-Type", "multipart/form-data")
        .send();
    match res.await {
        Ok(r) => match r.text().await {
            Ok(body_str) => {
                println!("{}", body_str);
                mat