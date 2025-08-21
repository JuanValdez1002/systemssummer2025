use serde::Serialize;
use chrono::{DateTime, Utc};
use std::time::{Duration, Instant};

#[derive(Debug, Serialize)]
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time_ms: u128,
    pub timestamp: DateTime<Utc>,
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
