use crate::Client;

#[test]
fn new_with_key() {
    let client = Client::new_with_key("YOUR_API_KEY").unwrap();
    assert_eq!(client.baseurl(), "https://coda.io/apis/v1");
}
