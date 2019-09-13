use super::handlers::AnalyzerError;
use failure::{Error, ResultExt};
use reqwest::Response;
use std::collections::HashMap;
use crate::data::{InsultResponse, InsultRequest};

const BASE_URL: &str = "https://evilinsult.com/generate_insult.php?type=json";

pub fn get_insult() -> Result<InsultResponse, Error> {
    let url = format!("{}{}", BASE_URL, "&lang=en");
    let result = get(&url)?;
    serde_json::from_str(&result).map_err(|e| format_err!("could not parse json, reason: {}", e))
}

pub fn get_insult_by_languange(language: &str) -> Result<InsultResponse, Error> {
    let url = format!("{}{}{}", BASE_URL, "&lang=", language);
    let result = get(&url)?;
    serde_json::from_str(&result).map_err(|e| format_err!("could not parse json, reason: {}", e))
}

pub fn create_insult(insult: &InsultRequest) -> Result<InsultResponse, Error> {
    let mut body: HashMap<&str, &str> = HashMap::new();
    body.insert("name", &insult.name);
    body.insert("color", &insult.color);
    body.insert("integration", &insult.integration);
    let path = format!("{}/activities", BASE_URL);
    let result = post(&path, &body)?;
    serde_json::from_str(&result).map_err(|e| format_err!("could not parse json, reason: {}", e))
}

fn post(path: &str, body: &HashMap<&str, &str>) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let res = client
        .post(path)
        .json(&body)
        .send()
        .context("error during post request")?;
    parse_result(res)
}

fn get(path: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let res = client
        .get(path)
        .send()
        .context("error during get request")?;

    parse_result(res)
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