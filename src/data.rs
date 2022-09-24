
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Model {
    #[default]
    GptTurbo,
    GptTurbo0301,
}

impl Model {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Model::GptTurbo => "gpt-3.5-turbo",
            Model::GptTurbo0301 => "gpt-3.5-turbo-0301",