use url::Url;

pub fn is_http_scheme(url: &str) -> Option<String> {
    match Url::parse(url) {
        Ok(url) => {
            Some(url.scheme().to_string())
        }
        Err(_) => {
            None
        }
    }
}