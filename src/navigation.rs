use std::net::{IpAddr, SocketAddr};

use url::Url;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationRequest {
    pub raw_input: String,
    pub normalized_uri: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NavigationError {
    EmptyInput,
    InvalidUri(String),
}

pub fn normalize_user_input(input: &str) -> Result<NavigationRequest, NavigationError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(NavigationError::EmptyInput);
    }

    let normalized_uri = if has_supported_scheme(trimmed) {
        trimmed.to_string()
    } else if let Some(ip_uri) = normalize_ip_host(trimmed) {
        ip_uri
    } else if looks_like_host(trimmed) {
        format!("https://{trimmed}")
    } else {
        return Err(NavigationError::InvalidUri(trimmed.to_string()));
    };

    Url::parse(&normalized_uri).map_err(|_| NavigationError::InvalidUri(trimmed.to_string()))?;

    Ok(NavigationRequest {
        raw_input: trimmed.to_string(),
        normalized_uri,
    })
}

fn has_supported_scheme(input: &str) -> bool {
    input.starts_with("http://")
        || input.starts_with("https://")
        || input.starts_with("file://")
        || input.starts_with("about:")
}

fn looks_like_host(input: &str) -> bool {
    input.contains('.') || input.starts_with("localhost")
}

fn normalize_ip_host(input: &str) -> Option<String> {
    if let Ok(socket_addr) = input.parse::<SocketAddr>() {
        return Some(format!("https://{socket_addr}"));
    }

    if let Ok(ip_addr) = input.parse::<IpAddr>() {
        let host = match ip_addr {
            IpAddr::V4(addr) => addr.to_string(),
            IpAddr::V6(addr) => format!("[{addr}]"),
        };
        return Some(format!("https://{host}"));
    }

    None
}
