use serde_json::Number;

#[derive(Serialize, Deserialize, Debug)]
pub struct SignInResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivitiesResponse {
    pub activities: Vec<ActivityResponse>,
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ActivityResponse {
    pub id: String,
    pub name: String,
    pub color: String,
    pub integration: String,
    pub device_side: Option<Number>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorListResponse {
    pub errors: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityRequest {
    pub name: String,
    pub color: String,
    pub integration: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditActivityRequest {
    pub name: Option<String>,
    pub color: Option<String>,
}
