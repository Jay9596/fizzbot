use reqwest;
use serde::{Deserialize, Serialize};

const SERVER: &str = "https://api.noopschallenge.com";
const ROUTE: &str = "/fizzbot";

#[derive(Debug, Deserialize)]
struct Rule {
    number: u8,
    response: String,
}

#[derive(Deserialize)]
struct ExampleResponse {
    answer: String,
}

#[derive(Deserialize)]
struct Response {
    message: String,
    #[serde(default)]
    rules: Vec<Rule>,
    #[serde(default)]
    numbers: Vec<i32>,
    nextQuestion: Option<String>,
    exampleResponse: Option<ExampleResponse>,
    result: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct Answer {
    answer: String,
}

impl Answer {
    pub fn new(ans: &str) -> Self {
        Answer {
            answer: ans.to_owned(),
        }
    }
}

fn start() {
    println!("  -- Interview Start -- ");

    let res = listen(ROUTE);

    if res.nextQuestion.is_some() {
        let next = res.nextQuestion.unwrap();
        let ques = listen(&next);
        let res = give_answer(&next, "RUST");
        if res.result == Some("correct".to_owned()) && res.nextQuestion.is_some() {
            solve(res);
        }
    }

    println!(" -- Interview End --  ");
}

fn solve(res: Response) {
    let next = res.nextQuestion.unwrap();
    let ques = listen(&next);
    let ans = try_solve(&ques.numbers, &ques.rules);
    let resp = give_answer(&next, &ans);
    if resp.result == Some("correct".to_owned()) && resp.nextQuestion.is_some() {
        solve(resp);
    } else if resp.result == Some("interview complete".to_owned()) {
        println!("THANK YOU! For your time!");
    } else {
        println!(
            "ERROR: Please try again\nMessage:{}\nRules:{:?}\nNumbers:{:?}\nExample response:{}",
            res.message,
            res.rules,
            res.numbers,
            res.exampleResponse.unwrap().answer
        );
    }
}

fn listen(speaker: &str) -> Response {
    reqwest::get(format!("{}{}", SERVER, speaker).as_str())
        .expect("Server Error")
        .json::<Response>()
        .expect("JSON parsing error")
}

fn give_answer(route: &str, ans: &str) -> Response {
    let ans = Answer::new(ans);
    reqwest::Client::new()
        .post(format!("{}{}", SERVER, route).as_str())
        .json(&ans)
        .send()
        .expect("Sending error")
        .json::<Response>()
        .expect("JSON parsing error in response")
}

fn try_solve(nums: &[i32], rules: &[Rule]) -> String {
    let check_rule = |n: i32| {
        let mut result = String::new();
        for r in rules.iter() {
            if n % (r.number as i32) == 0 {
                result.push_str(&r.response)
            }
        }
        if result.is_empty() {
            n.to_string()
        } else {
            result
        }
    };

    nums.iter()
        .map(|&n| check_rule(n))
        .collect::<Vec<String>>()
        .join(" ")
        .trim()
        .to_string()
}

fn main() {
    start()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_solve_fizzbuzzbazz() {
        assert_eq!(
            try_solve(
                &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 105],
                &[
                    Rule {
                        number: 3,
                        response: "Fizz".to_owned()
                    },
                    Rule {
                        number: 5,
                        response: "Buzz".to_owned()
                    },
                    Rule {
                        number: 7,
                        response: "Bazz".to_owned()
                    }
                ]
            ),
            "1 2 Fizz 4 Buzz Fizz Bazz 8 Fizz Buzz 11 Fizz 13 Bazz FizzBuzz FizzBuzzBazz"
        );
    }

    #[test]
    fn sample_solve_fizbuzz() {
        assert_eq!(
            try_solve(
                &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
                &[
                    Rule {
                        number: 3,
                        response: "Fizz".to_owned()
                    },
                    Rule {
                        number: 5,
                        response: "Buzz".to_owned()
                    }
                ]
            ),
            "1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz"
        );
    }
}
