mod data;
use std::io::{Error, ErrorKind};

use data::CompletionRequest;

pub async fn async_completions(
    key: String,
    model: data::Model,
    messages: Vec<data::Message>,
) -> Result