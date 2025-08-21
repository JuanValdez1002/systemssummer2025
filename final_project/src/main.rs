use serde::Serialize;
use chrono::{DateTime, Utc};
use std::time::{Duration, Instant};
use std::sync::mpsc;
use std::thread;

#[derive(Debug, Serialize)]
struct WebsiteStatus {
    url: String,
    status: Result<u16, String>,
    response_time_ms: u128,
    timestamp: DateTime<Utc>,
}

pub fn check_website(url: &str, timeout: Duration, max_retries: u32) -> WebsiteStatus {
    let start = Instant::now();
    let mut last_err = None;
    for _ in 0..max_retries {
        let agent = ureq::AgentBuilder::new()
            .timeout(timeout)
            .build();
        let resp = agent.get(url).call();
        let elapsed = start.elapsed();
        let timestamp = Utc::now();
        match resp {
            Ok(response) => {
                return WebsiteStatus {
                    url: url.to_string(),
                    status: Ok(response.status()),
                    response_time_ms: elapsed.as_millis(),
                    timestamp,
                };
            }
            Err(e) => {
                last_err = Some(e.to_string());
            }
        }
    }
    WebsiteStatus {
        url: url.to_string(),
        status: Err(last_err.unwrap_or_else(|| "Unknown error".to_string())),
        response_time_ms: start.elapsed().as_millis(),
        timestamp: Utc::now(),
    }
}

fn main() {
        let urls = vec![
            "https://www.rust-lang.org",
            "https://www.google.com",
            "https://www.github.com",
            "https://www.thiswebsitedoesnotexist12345.com",
            "https://www.wikipedia.org",
            "https://www.stackoverflow.com",
            "https://www.microsoft.com",
            "https://www.apple.com",
            "https://www.reddit.com",
            "https://www.bbc.com",
        ];
        let num_workers = 10;
        let timeout = Duration::from_secs(5);
        let max_retries = 2;

        let (tx, rx) = mpsc::channel();

        let chunk_size = (urls.len() + num_workers - 1) / num_workers;
        for chunk in urls.chunks(chunk_size) {
            let tx = tx.clone();
            let chunk = chunk.to_vec();
            thread::spawn(move || {
                for url in chunk {
                    let status = check_website(&url, timeout, max_retries);
                    tx.send(status).unwrap();
                }
            });
        }

        drop(tx);

        for status in rx {
            println!("{:#?}", status);
        }
    }