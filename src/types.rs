use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudFlareResponse {
    #[serde(rename = "Status")]
    status: i64,
    #[serde(rename = "TC")]
    tc: bool,
    #[serde(rename = "RD")]
    rd: bool,
    #[serde(rename = "RA")]
    ra: bool,
    #[serde(rename = "AD")]
    ad: bool,
    #[serde(rename = "CD")]
    cd: bool,
    #[serde(rename = "Question")]
    question: Vec<Question>,
    #[serde(rename = "Answer")]
    answer: Vec<Answer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    name: String,
    #[serde(rename = "type")]
    answer_type: i64,
    #[serde(rename = "TTL")]
    ttl: i64,
    data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    name: String,
    #[serde(rename = "type")]
    question_type: i64,
}
