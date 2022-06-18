use reqwest::{Client, header::HeaderMap};
use anyhow::Result;

static DEFAULT_BASE_API: &str = "https://www.yuque.com/api/v2";
static DEFAULT_USER_AGENT: &str = "Yuque-Rust-Client";

#[derive(Debug)]
pub struct Yuque {
    pub client: Client,
}

impl Yuque {
    pub fn new(token: String) -> Result<Yuque> {
        let mut headers = HeaderMap::new();
        headers.insert("X-Auth-Token", token.parse().unwrap());
        let client = Client::builder()
            .user_agent(DEFAULT_USER_AGENT)
            .default_headers(headers)
            .build()?;
        Ok(Yuque { client })
    }

    pub fn build_api(endpoint: &str, space: Option<&str>) -> String {
        if let Some(space) = space {
            format!("https://{}.yuque.com/api/v2{}", space, endpoint)
        } else {
            format!("{}{}", DEFAULT_BASE_API, endpoint)
        }
    }
}
