use core::time;
use std::env::VarError;

const SO_URL: &str = "https://api.stackexchange.com/2.3/questions";
// the tag is hardcoded for now to `rust`
const TAG: &str = "rust";

pub fn get_text_response() -> Result<String, Box<dyn std::error::Error>> {
    loop {
        let resp = reqwest_client()
            .expect("Can't build 'reqwest' client")
            .send();

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

fn reqwest_client() -> Result<reqwest::blocking::RequestBuilder, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let auth_key = std::env::var("SO_NOTIFY_AUTH_KEY");
    let query_args = query_args(&auth_key);

    Ok(client.get(SO_URL).query(&query_args))
}

fn query_args(auth_key: &Result<String, VarError>) -> Vec<(&'static str, &str)> {
    let mut query_args = vec![
        ("page", "1"),
        ("pagesize", "10"),
        ("order", "desc"),
        ("sort", "creation"),
        ("site", "stackoverflow"),
        ("tagged", TAG),
    ];

    if let Ok(ref auth_key_ref) = auth_key {
        query_args.push(("key", auth_key_ref));
    };

    query_args
}
