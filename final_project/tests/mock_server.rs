use httpmock::MockServer;
use httpmock::Method::GET;
use final_project::check_website;
use std::time::Duration;

#[test]
fn test_mock_server_success() {
    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(GET).path("/");
        then.status(200);
    });
    let url = &server.url("/");
    let status = check_website(url, Duration::from_secs(2), 1);
    assert_eq!(status.status, Ok(200));
}

#[test]
fn test_mock_server_failure() {
    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(GET).path("/fail");
        then.status(500);
    });
    let url = &server.url("/fail");
    let status = check_website(url, Duration::from_secs(2), 1);
    assert!(status.status.is_err());
    assert!(status.status.as_ref().err().unwrap().contains("500"));
}
