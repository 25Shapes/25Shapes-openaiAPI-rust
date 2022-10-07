mod data;
use std::io::{Error, ErrorKind};

use data::CompletionRequest;

pub async fn async_completions(
    key: String,
    model: data::Model,
    messages: Vec<data::Message>,
) -> Result<data::CompletionResponse, Error> {
    let model_name = model.as_str().to_string();