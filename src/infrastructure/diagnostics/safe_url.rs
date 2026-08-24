use std::fmt::{Display, Formatter};

use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafeUrl(String);

impl SafeUrl {
    pub fn from_url(url: &Url) -> Self {
        let scheme = url.scheme();
        let host = url.host_str().unwrap_or_default();
        let path = url.path();

        match url.port() {
            Some(port) => Self(format!("{scheme}://{host}:{port}{path}")),
            None => Self(format!("{scheme}://{host}{path}")),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for SafeUrl {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}
