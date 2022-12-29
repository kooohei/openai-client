use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum GPT3 {
    #[serde(rename(serialize="text-davinci-003"))]
    TextDavinci003,
    #[serde(rename(serialize="text-curie-001"))]
    TextCurie001,
    #[serde(rename(serialize="text-babbage-001"))]
    TextBabbage001,
    #[serde(rename(serialize="text-ada-001"))]
    TextAda001,
}

pub enum CODEX {
    CodeDavinci002,
    CodeCushman001,
}

pub enum ContentFilter {
    ContentFilterAlpha,
}
