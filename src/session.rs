use surf::{Config, Request, Url, http::Method::Post};
use crate::parameters;

#[derive(Debug)]
pub struct Session {
    pub key: String,
    pub client: surf::Client,
}

impl Session {
    pub fn new(apikey: String) -> Result<Self, String> {
        let c = Config::new()
            .set_timeout(Some(std::time::Duration::from_secs(5)))
            .try_into();

        match c {
            Ok(c) => {
                let s = Session { key: apikey, client: c };
                Ok(s)               
            },
            Err(er) => Err(er.to_string()),
        } 
    }

    pub async fn completions(&self, params: parameters::TextCompletion) -> Result<String, String> {
        let url = Url::parse("https://api.openai.com/v1/completions");
        let url = match url {
            Ok(u) => u,
            Err(er) => return Err(er.to_string()),
        };

        let mut req = Request::new(Post, url);
        let auth_value = format!("Bearer {}", self.key);
        req.set_header("content-type", "application/json");
        req.set_header("Authorization", auth_value);

        let convjson= serde_json::to_string(&params);
        let body = match convjson{
            Ok(body) => body,
            Err(er) => return Err(er.to_string()),
        };
        // dbg!(&body);
        req.body_string(body);

        let mut res = match self.client.send(req).await {
            Ok(r) => r,
            Err(er) => return Err(er.to_string()),
        };
        
        let json_string = res.body_string().await;

        match json_string {
            Ok(res) => Ok(res),
            Err(er) => Err(er.to_string()),
        }
    }
}
