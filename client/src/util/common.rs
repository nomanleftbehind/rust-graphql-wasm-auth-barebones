use crate::util::constant::CFG;
use reqwest::{Error, Response};
use serde_json::Value;

pub async fn build_request(request_json: &Value) -> Result<Response, Error> {
    let addr = CFG.get("gql.addr").unwrap();
    let port = CFG.get("gql.port").unwrap();
    let path = CFG.get("gql.path").unwrap();

    let url = format!("http://{}:{}/{}", addr, port, path);
    let response = reqwest::Client::new()
        .post(url)
        .json(request_json)
        // Compiles but shows as error
        // Implement this when stabilized: https://doc.rust-lang.org/cargo/reference/unstable.html#per-package-targets
        .fetch_credentials_include()
        .send()
        .await;

    response
}
