use crate::{Limiter, RawClient};

pub struct Client {
    pub raw: RawClient,
    pub limiter: Limiter,
}

impl Client {
    pub fn new_with_key(api_key: &str) -> reqwest::Result<Self> {
        let raw = RawClient::new_with_key(api_key)?;
        let limiter = Limiter::default();

        Ok(Self {
            raw,
            limiter,
        })
    }
}
