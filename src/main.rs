use core::time;
use notify_rust::{Notification, Urgency};
use serde::Deserialize;
use std::collections::HashSet;

const SO_URL: &str = "https://api.stackexchange.com/2.3/questions";
// the tag is hardcoded for now to `rust`
const TAG: &str = "rust";

#[derive(Deserialize, Debug)]
struct Root {
    items: Vec<Question>,
    quota_max: u32,
    quota_remaining: u32,
}

#[derive(Deserialize, Debug)]
struct Question {
    title: String,
    link: String,
    question_id: u32,
}

fn query_args() -> Vec<(&'static str, &'static str)> {
    vec![
        ("page", "1"),
        ("pagesize", "10"),
        ("order", "desc"),
        ("sort", "creation"),
        ("site", "stackoverflow"),
        ("tagged", TAG),
    ]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut question_ids = HashSet::new();
    let client = reqwest::blocking::Client::builder().build()?;
    let mut query_args = query_args();
    let auth_key;
    if let Ok(env_var) = std::env::var("SO_NOTIFY_AUTH_KEY") {
        // This probably deserve a question on SO: while I solved
        // the issue with env_var lifetime (&str is required in query_args),
        // the solution looks suboptimal
        auth_key = env_var;
        query_args.push(("key", &auth_key));
    };

    loop {
        let resp = client.get(SO_URL).query(&query_args).send();
        let text_resp;

        match resp {
            Ok(value) => text_resp = value.text()?,
            Err(e) if e.is_timeout() => {
                println!("Request timed out, retrying...");
                continue;
            }
            Err(e) => return Err(Box::new(e)),
        }

        let questions = serde_json::from_str::<Root>(&text_resp).expect(&text_resp);

        println!(
            "Quota max: {}, quota remaining: {}",
            questions.quota_max, questions.quota_remaining
        );

        if !question_ids.is_empty() {
            for q in &questions.items {
                if !question_ids.contains(&q.question_id) {
                    println!("{}\n{}\n{}\n", q.title, q.link, q.question_id);
                    desktop_notification(&q.title, &q.link);
                }
            }
        }

        question_ids.clear();
        for q in questions.items {
            question_ids.insert(q.question_id);
        }

        std::thread::sleep(time::Duration::from_millis(60_000));
    }
}

fn desktop_notification(summary: &str, link: &str) {
    let handle = Notification::new()
        .summary(summary)
        .body(link)
        .icon("hint")
        .urgency(Urgency::Critical)
        .show()
        .expect("Notification must be sent");

    // won't show the next notification until the previous one is clicked on
    handle.wait_for_action(|_action| ());
}
