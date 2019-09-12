use super::data::{
    ActivitiesResponse, ActivityRequest, ActivityResponse, EditActivityRequest, ErrorListResponse,
    SignInResponse,
};
use super::handlers::AnalyzerError;
use failure::{Error, ResultExt};
use reqwest::Response;
use std::collections::HashMap;
use crate::data::InsultResponse;

const BASE_URL: &str = "https://evilinsult.com/generate_insult.php?lang=en&type=json";

pub fn get_insult() -> Result<InsultResponse, Error> {
    let url = BASE_URL;//format!("{}/activities", BASE_URL);
    let result = get(&url)?;
    serde_json::from_str(&result).map_err(|e| format_err!("could not parse json, reason: {}", e))
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