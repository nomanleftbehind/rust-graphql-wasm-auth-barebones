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
        .json(request_json)
        // Compiles but shows as error
        // Implement this when stabilized: https://doc.rust-lang.org/cargo/reference/unstable.html#per-package-targets
        .fetch_credentials_include()
        .send()
        .await;

    response
}

// pub async fn fetch_gql_data(query: &str) -> Result<Value, FetchError> {
//     let mut req_opts = RequestInit::new();
//     req_opts.method("POST");
//     req_opts.body(Some(&JsValue::from_str(query)));
//     req_opts.mode(RequestMode::Cors);

//     let request = Request::new_with_str_and_init(&gql_uri().await, &req_opts)?;

//     let window = gloo_utils::window();
//     let resp_value =
//         JsFuture::from(window.fetch_with_request(&request)).await?;
//     let resp: Response = resp_value.dyn_into().unwrap();
//     let resp_text = JsFuture::from(resp.text()?).await?;

//     let data_str = resp_text.as_string().unwrap();
//     let data_value: Value = from_str(&data_str).unwrap();

//     Ok(data_value["data"].clone())
// }