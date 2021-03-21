use serde::{Serialize, Deserialize};
use reqwest::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderParam {
    pub provider: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Provider {
    pub name: String,
    pub oauth: bool
}

pub struct ProviderInner {
    pub name: String,
    pub oauth: bool,
    pub addr: String,
    pub port: u16,
}

impl ProviderInner {
    pub fn ext(self) -> Provider {
        Provider{
            name: self.name,
            oauth: self.oauth
        }
    }
    pub fn url(self, path: &'static str) -> Url {
        Url::parse(&format!("http://{}:{}{}", self.addr, self.port, path)).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub oauth_provider: String,
}
