use openai_client::{parameters::TextCompletion, session::Session, models::GPT3};
use async_std::task;
use serde_json;
use serde_json::{ Value, Error };
use std::process;

fn main() {
    let prompt = "hello chat gpt! you are awesome!".to_owned(); 
    let mut params = TextCompletion::from_default(GPT3::TextDavinci003, prompt);
    params.max_tokens = Some(200);
    params.stream = Some(false);

    let _key = "xxx".to_owned();
    let sess= Session::new(_key).unwrap();

    task::block_on(async {
        let res: Result<String, String> = sess.completions(params).await;

        let json_string = match res {
            Ok(r) => r,
            Err(er) => {
                eprintln!("{}", er);
                process::exit(1);
            }
        };

        let res:Result<Value, Error> = serde_json::from_str(&json_string);

        match res {
            Ok(json) => {
                dbg!(json);
                process::exit(1);
            },
            Err(er) => {
                eprintln!("{}", er);
                process::exit(1);
            }
        }
    });
}
