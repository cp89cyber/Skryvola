#[path = "../src/navigation.rs"]
mod navigation;

use navigation::{normalize_user_input, NavigationError};

#[test]
fn trims_whitespace() {
    let request = normalize_user_input("  example.com  ").expect("expected valid request");
    assert_eq!(request.raw_input, "example.com");
    assert_eq!(request.normalized_uri, "https://example.com");
}

#[test]
fn accepts_http_url() {
    let request = normalize_user_input("http://example.com").expect("expected valid request");
    assert_eq!(request.normalized_uri, "http://example.com");
}

#[test]
fn accepts_https_url() {
    let request = normalize_user_input("https://example.com").expect("expected valid request");
    assert_eq!(request.normalized_uri, "https://example.com");
}

#[test]
fn accepts_about_blank() {
    let request = normalize_user_input("about:blank").expect("expected valid request");
    assert_eq!(request.normalized_uri, "about:blank");
}

#[test]
fn prefixes_bare_domains() {
    let request = normalize_user_input("example.com/path").expect("expected valid request");
    assert_eq!(request.normalized_uri, "https://example.com/path");
}

#[test]
fn prefixes_localhost() {
    let request = normalize_user_input("localhost:3000").expect("expected valid request");
    assert_eq!(request.normalized_uri, "https://localhost:3000");
}

#[test]
fn rejects_empty_input() {
    let error = normalize_user_input("   ").expect_err("expected empty input error");
    assert_eq!(error, NavigationError::EmptyInput);
}

#[test]
fn rejects_malformed_input_with_spaces() {
    let error = normalize_user_input("hello world").expect_err("expected invalid uri error");
    assert_eq!(error, NavigationError::InvalidUri("hello world".to_string()));
}

#[test]
fn rejects_plain_words_without_host_shape() {
    let error = normalize_user_input("hello").expect_err("expected invalid uri error");
    assert_eq!(error, NavigationError::InvalidUri("hello".to_string()));
}
