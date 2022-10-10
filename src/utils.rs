pub fn build_reqwest_client() -> Result<reqwest::Client, reqwest::Error> {
    let mut headers = reqwest::header::HeaderMap::new();

    headers.insert(
        "Authorization",
        format!("Basic {}", dotenv::var("WAKATIME_API_KEY").unwrap())
            .parse()
            .unwrap(),
    );

    reqwest::Client::builder().default_headers(headers).build()
}

pub fn seconds_to_minutes(seconds: f64) -> i64 {
    (seconds as i64) / 60
}
