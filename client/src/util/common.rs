use crate::util::constant::CFG;
use reqwest::{Error, Response};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: T,
}

pub async fn build_request(request_json: &Value) -> Result<Response, Error> {
    let addr = CFG.get("gql.addr").unwrap();
    let port = CFG.get("gql.port").unwrap();
    let path = CFG.get("gql.path").unwrap();

    let url = format!("http://{}:{}/{}", addr, port, path);
    let response = reqwest::Client::new()
        .post(url)
        .header(reqwest::header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
        .json(request_json)
        .send()
        .await;

    response
}
