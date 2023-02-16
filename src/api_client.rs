use core::time;

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
