use crate::models;
use std::collections::HashMap;
use serde::Serialize;
use serde;

#[derive(Debug, Serialize)]
pub struct TextCompletion {
    pub model: models::GPT3,
    pub prompt: String,
    
    #[serde(skip_serializing_if="Option::is_none")]
    pub suffix: Option<String>,
    
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_tokens: Option<i16>,
   
    #[serde(skip_serializing_if="Option::is_none")]
    pub temperature: Option<u16>,
   
    #[serde(skip_serializing_if="Option::is_none")]
    pub top_p: Option<u16>,
    
    #[serde(skip_serializing_if="Option::is_none")]
    pub n: Option<i16>,
    
    #[serde(skip_serializing_if="Option::is_none")]
    pub stream: Option<bool>,
    
    #[serde(skip_serializing_if="Option::is_none")]
    pub logprobs: Option<i16>,
    
    #[serde(skip_serializing_if="Option::is_none")]
    pub echo: Option<bool>,
    
    #[serde(skip_serializing_if="Option::is_none")]
    pub stop: Option<String>,
    
    #[serde(skip_serializing_if="Option::is_none")]
    pub presence_penalty: Option<u16>,
    
    #[serde(skip_serializing_if="Option::is_none")]

    pub frequency_penalty: Option<u16>,
    
    #[serde(skip_serializing_if="Option::is_none")]
    pub best_of: Option<u16>,
    
    #[serde(skip_serializing_if="Option::is_none")]
    pub logit_bias: Option<HashMap<String, i32>>,
    
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<String>,
}

impl TextCompletion {
    pub fn from_default(model: models::GPT3, prompt: String) -> TextCompletion {
        TextCompletion {
           model: model,
           prompt: prompt,
           suffix: None,
           max_tokens: Some(10),
           temperature: Some(1),
           top_p: Some(1),
           n: Some(1),
           stream: Some(false),
           logprobs: None,
           echo: None,
           stop: None,
           presence_penalty: None,
           frequency_penalty: None,
           best_of: None,
           logit_bias: None,
           user: None,
        }
    }
}
