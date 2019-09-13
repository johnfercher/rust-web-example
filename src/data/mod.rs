use serde_json::Number;

#[derive(Serialize, Deserialize, Debug)]
pub struct InsultResponse {
    pub number: String,
    pub language: String,
    pub insult: String,
    pub created: String,
    pub shown: String,
    pub createdby: String,
    pub active: String,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InsultRequest {
    pub name: String,
    pub color: String,
    pub integration: String,
}