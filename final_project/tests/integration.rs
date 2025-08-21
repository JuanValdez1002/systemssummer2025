use std::time::Duration;
use final_project::check_website;

#[test]
fn test_check_website_success() {
    let status = check_website("https://www.rust-lang.org", Duration::from_secs(5), 1);
    assert!(status.status.is_ok());
}

#[test]
fn test_check_website_failure() {
    let status = check_website("https://notarealwebsite.rust", Duration::from_secs(2), 1);
    assert!(status.status.is_err());
}

#[test]
fn test_performance_many_requests() {
    let urls = vec!["https://www.rust-lang.org"; 50];
    let timeout = Duration::from_secs(2);
    let max_retries = 1;
    let (tx, rx) = std::sync::mpsc::channel();
    for url in urls {
        let tx = tx.clone();
        let url = url.to_string();
        std::thread::spawn(move || {
            let status = check_website(&url, timeout, max_retries);
            tx.send(status).unwrap();
        });
    }
    drop(tx);
    let results: Vec<_> = rx.into_iter().collect();
    assert_eq!(results.len(), 50);
}

#[test]
fn test_timeout() {
    let status = check_website("http://10.255.255.1", Duration::from_millis(100), 1);
    assert!(status.status.is_err());
}
