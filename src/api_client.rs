use core::time;
use std::env::VarError;

use super::logger::log;

const SO_URL: &str = "https://api.stackexchange.com/2.3/questions";

pub fn get_text_response() -> Result<String, Box<dyn std::error::Error>> {
    loop {
        let resp = reqwest_client()
            .expect("Can't build 'reqwest' client")
            .send();

        match resp {
            Ok(value) => return Ok(value.text().expect("Can't get text response")),
            Err(e) => {
                log(format!("{e}\n"));
                log("Request failed, retrying...");

                std::thread::sleep(time::Duration::from_millis(60_000));
            }
        }
    }
}

fn reqwest_client() -> Result<reqwest::blocking::RequestBuilder, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let question_tag = &get_question_tag();
    let auth_key = std::env::var("SO_NOTIFY_AUTH_KEY");
    let query_args = query_args(question_tag, &auth_key);

    Ok(client.get(SO_URL).query(&query_args))
}

fn query_args<'a>(
    question_tag: &'a str,
    auth_key: &'a Result<String, VarError>,
) -> Vec<(&'static str, &'a str)> {
    let mut query_args = vec![
        ("page", "1"),
        ("pagesize", "10"),
        ("order", "desc"),
        ("sort", "creation"),
        ("site", "stackoverflow"),
        ("tagged", question_tag),
    ];

    if let Ok(ref auth_key_ref) = auth_key {
        query_args.push(("key", auth_key_ref));
    };

    query_args
}

fn get_question_tag() -> String {
    let mut args = std::env::args();

    // the name of the program has to be skipped
    args.next();
    args.next().expect("SO tag must be sent in arguments")
}
