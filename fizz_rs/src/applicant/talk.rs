#![allow(non_snake_case)]
use reqwest;
use serde::{Deserialize, Serialize};

pub const SERVER: &str = "https://api.noopschallenge.com";
pub const ROUTE: &str = "/fizzbot";

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub number: u8,
    pub response: String,
}

#[derive(Deserialize)]
pub struct ExampleResponse {
    pub answer: String,
}

#[derive(Deserialize)]
pub struct Response {
    pub message: String,
    #[serde(default)]
    pub rules: Vec<Rule>,
    #[serde(default)]
    pub numbers: Vec<i32>,
    pub nextQuestion: Option<String>,
    pub exampleResponse: Option<ExampleResponse>,
    pub result: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Answer {
    answer: String,
}

impl Answer {
    pub fn new(ans: &str) -> Self {
        Answer {
            answer: ans.to_owned(),
        }
    }
}

pub struct Interviewer;

impl Interviewer {
    pub fn listen(speaker: &str) -> Response {
        reqwest::get(format!("{}{}", SERVER, speaker).as_str())
            .expect("Server Error")
            .json::<Response>()
            .expect("JSON parsing error")
    }

    pub fn give_answer(route: &str, ans: &str) -> Response {
        let ans = Answer::new(ans);
        reqwest::Client::new()
            .post(format!("{}{}", SERVER, route).as_str())
            .json(&ans)
            .send()
            .expect("Sending error")
            .json::<Response>()
            .expect("JSON parsing error in response")
    }
}
