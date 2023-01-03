use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudFlareResponse {
    #[serde(rename = "Status")]
    pub status: i64,
    #[serde(rename = "TC")]
    pub tc: bool,
    #[serde(rename = "RD")]
    pub rd: bool,
    #[serde(rename = "RA")]
    pub ra: bool,
    #[serde(rename = "AD")]
    pub ad: bool,
    #[serde(rename = "CD")]
    pub cd: bool,
    #[serde(rename = "Question")]
    pub question: Vec<Question>,
    #[serde(rename = "Answer")]
    pub answer: Vec<Answer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    pub name: String,
    #[serde(rename = "type")]
    pub answer_type: i64,
    #[serde(rename = "TTL")]
    pub ttl: i64,
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub name: String,
    #[serde(rename = "type")]
    pub question_type: i64,
}
