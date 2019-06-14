use super::talk::{Interviewer, Response, Rule, ROUTE};
use std::time::Instant;

pub struct Interviewee {
    language: String,
}

impl Interviewee {
    pub fn new(language: &str) -> Self {
        Self {
            language: language.to_string(),
        }
    }

    pub fn start_interview(&self) {
        let now = Instant::now();
        println!("  -- Interview Start -- ");

        let res = Interviewer::listen(ROUTE);
        println!("Dear Applicant,\n{}", res.message);

        if res.nextQuestion.is_some() {
            let next = res.nextQuestion.unwrap();
            let ques = Interviewer::listen(&next);
            println!("{}\n", ques.message);
            println!("ANSWER: {}\n", &self.language);
            let res = Interviewer::give_answer(&next, &self.language);
            if res.result == Some("correct".to_owned()) && res.nextQuestion.is_some() {
                println!("RESPONSE: {}", res.message);
                Self::solve_question(res);
            }
        }

        println!(
            " -- Interview End --  \nTime taken: {:#?} seconds",
            now.elapsed().as_secs()
        );
    }

    fn solve_question(res: Response) {
        println!("\t--------------------------------------\t");
        let next = res.nextQuestion.unwrap();
        let ques = Interviewer::listen(&next);
        println!(
            "QUESTION:\n{}\nNumbers:{:?}\nRules:{:?}\n",
            ques.message, ques.numbers, ques.rules
        );
        let ans = Self::try_solve(&ques.numbers, &ques.rules);
        println!("ANSWER: {}\n", ans);
        let resp = Interviewer::give_answer(&next, &ans);
        if resp.result == Some("correct".to_owned()) && resp.nextQuestion.is_some() {
            println!(
                "RESPONSE: {}\nAnswer is: {}",
                resp.message,
                resp.result.clone().unwrap()
            );
            Self::solve_question(resp);
        } else if resp.result == Some("interview complete".to_owned()) {
            println!("RESPONSE: {}\n{}", resp.message, resp.result.unwrap());
            println!("\nANSWER: THANK YOU! For your time!\n");
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
}

#[cfg(test)]
mod tests {
    use super::{super::talk::Rule, Interviewee};

    #[test]
    fn sample_solve_fizzbuzzbazz() {
        assert_eq!(
            Interviewee::try_solve(
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
            Interviewee::try_solve(
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
