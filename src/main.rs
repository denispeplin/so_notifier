use core::time;
use notify_rust::{Notification, Urgency};
use serde::Deserialize;

use questions::Question;

const SO_URL: &str = "https://api.stackexchange.com/2.3/questions";
// the tag is hardcoded for now to `rust`
const TAG: &str = "rust";

#[derive(Deserialize, Debug)]
struct Root {
    items: Vec<Question>,
    quota_max: u32,
    quota_remaining: u32,
}

fn client() -> Result<reqwest::blocking::RequestBuilder, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder().build()?;

    let mut query_args = vec![
        ("page", "1"),
        ("pagesize", "10"),
        ("order", "desc"),
        ("sort", "creation"),
        ("site", "stackoverflow"),
        ("tagged", TAG),
    ];

    let auth_key;
    if let Ok(env_var) = std::env::var("SO_NOTIFY_AUTH_KEY") {
        // This probably deserve a question on SO: while I solved
        // the issue with env_var lifetime (&str is required in query_args),
        // the solution looks suboptimal
        auth_key = env_var;
        query_args.push(("key", &auth_key));
    };

    Ok(client.get(SO_URL).query(&query_args))
}

fn get_text_response() -> Result<String, Box<dyn std::error::Error>> {
    loop {
        let resp = client().expect("Can't build 'reqwest' client").send();

        match resp {
            Ok(value) => return Ok(value.text().expect("Can't get text response")),
            Err(e) if e.is_timeout() => {
                println!("Request timed out, retrying...");
                std::thread::sleep(time::Duration::from_millis(10_000));
                continue;
            }
            Err(e) => return Err(Box::new(e)),
        }
    }
}

fn decode_questions(text_resp: String) -> Root {
    serde_json::from_str::<Root>(&text_resp).expect(&text_resp)
}

pub mod questions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // it's a trick so on the first run none of
    // the questions would be considered new
    let mut latest_question_id = u32::MAX;

    loop {
        let text_resp = get_text_response()?;
        let root = decode_questions(text_resp);
        let questions = root.items;

        println!(
            "Quota max: {}, quota remaining: {}",
            root.quota_max, root.quota_remaining
        );

        let new_questions = questions::list_new(&questions, latest_question_id);

        latest_question_id = questions::latest_id(&questions);

        for q in new_questions {
            println!("{}\n{}\n{}\n", q.title, q.link, q.id);
            desktop_notification(&q.title, &q.link);
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
