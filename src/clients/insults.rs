use failure::{Error, ResultExt};
use reqwest::Response;
use std::collections::HashMap;
use crate::domain::models::{InsultResponse, InsultRequest};

const BASE_URL: &str = "https://evilinsult.com/generate_insult.php?type=json";

pub struct InsultClient {
    http_client: reqwest::Client
}

pub fn new(http_client: reqwest::Client) -> InsultClient {
    InsultClient{
        http_client: http_client
    }
}

impl InsultClient {
    pub fn get_insult(&self) -> Result<InsultResponse, Error> {
        let url = format!("{}{}", BASE_URL, "&lang=en");

        let res = self.http_client
            .get(&url)
            .send()
            .context("error during get request")?;

        let result = parse_result(res)?;

        serde_json::from_str(&result).map_err(|e| format_err!("could not parse json, reason: {}", e))
    }

    pub fn get_insult_by_languange(&self, language: &str) -> Result<InsultResponse, Error> {
        let url = format!("{}{}{}", BASE_URL, "&lang=", language);

        let res = self.http_client
            .get(&url)
            .send()
            .context("error during get request")?;

        let result = parse_result(res)?;

        serde_json::from_str(&result).map_err(|e| format_err!("could not parse json, reason: {}", e))
    }

    pub fn create_insult(&self, insult: &InsultRequest) -> Result<InsultResponse, Error> {
        let mut body: HashMap<&str, &str> = HashMap::new();

        body.insert("name", &insult.name);
        body.insert("color", &insult.color);
        body.insert("integration", &insult.integration);

        let path = format!("{}/activities", BASE_URL);

        let res = self.http_client
            .post(&path)
            .json(&body)
            .send()
            .context("error during post request")?;

        let result = parse_result(res)?;

        serde_json::from_str(&result).map_err(|e| format_err!("could not parse json, reason: {}", e))
    }
}


fn parse_result(mut res: Response) -> Result<String, Error> {
    let mut buf: Vec<u8> = vec![];
    if res.status().is_success() {
        res.copy_to(&mut buf)
            .context("could not copy response into buffer")?;
    } else {
        return Err(format_err!("request error: {}", res.status()));
    }
    let result = std::str::from_utf8(&buf)?;
    Ok(result.to_string())
}