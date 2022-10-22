use crate::util::{common::build_request, console_log::console_log};
use graphql_client::{GraphQLQuery, Response};
use serde_json::json;

#[derive(Clone, Debug)]
pub struct QueryResponse<T> {
    pub data: Option<T>,
    pub error: Option<String>,
    pub loading: bool,
}

pub async fn lazy_function<Q>(variables: Q::Variables) -> QueryResponse<Q::ResponseData>
where
    Q: GraphQLQuery,
    Q::Variables: 'static,
    Q::ResponseData: Clone + 'static,
{
    let request_body = Q::build_query(variables);
    let request_json = &json!(request_body);
    let request = build_request(request_json).await;
    match request {
        Ok(response) => {
            console_log!("mutation: {:?}", &response);

            let json = response.json::<Response<Q::ResponseData>>().await;

            match json {
                Ok(response) => QueryResponse {
                    data: response.data,
                    error: None,
                    loading: false,
                },
                Err(error) => QueryResponse {
                    data: None,
                    error: Some(error.to_string()),
                    loading: false,
                },
            }
        }
        Err(error) => QueryResponse {
            data: None,
            error: Some(error.to_string()),
            loading: false,
        },
    }
}
