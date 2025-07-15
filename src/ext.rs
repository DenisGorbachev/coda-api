use crate::Client;

impl Client {
    pub const BASE_URL: &'static str = "https://coda.io/apis/v1";

    pub fn new_with_key(api_key: &str) -> reqwest::Result<Client> {
        let authorization_header = format!("Bearer {}", api_key);

        let mut headers = reqwest::header::HeaderMap::with_capacity(1);
        headers.insert(
            reqwest::header::AUTHORIZATION,
            authorization_header
                .parse()
                .expect("API key should be valid"),
        );

        let client_with_custom_defaults = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()?;

        let client = Self::new_with_client(Self::BASE_URL, client_with_custom_defaults);

        Ok(client)
    }
}
