use reqwest::header::HeaderMap;

pub struct SeurityHeaders {
    pub csp: Option<String>,
    pub hsts: Option<String>,
    pub x_frame_options: Option<String>,
}

pub fn extract_security_headers(headers: &HeaderMap) -> SecurityHeaders {
    let csp = headers
        .get("Content-security-policy")
        .and_then(|v| v.to_str().ok())
        .map(String::from);

    let hst = headers
        .get("strict-transport-security")
        .and_then(|v| v.to_str().ok())
        .map(String::from);

    let x_frame_options = headers
        .get("x_frame-options")
        .and_then(|v| v.to_str().ok())
        .map(String::from);

    SecurityHeaders {
        csp,
        hsts,
        x_frame_options,
    }
}
