use crate::util::{
    common::{fetch_gql_data, FetchState},
    constant::CFG,
};
use graphql_client::GraphQLQuery;
use serde_json::{json, Value};
use std::fmt::Debug;
use yew::prelude::*;
use yew::{html, Component, Context, Html};

// NaiveDateTime is a custom scalar serialized as string.
// It has to be defined in the scope where the struct under derive is located, next to the query struct will work.
type UUID = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/all_users.graphql",
    response_derives = "Debug"
)]
struct AllUsers;

async fn query_str(whatever: String) -> String {
    let build_query = AllUsers::build_query(all_users::Variables { whatever });
    let query = json!(build_query);

    query.to_string()
}

pub enum Msg {
    SetState(FetchState<Value>),
    GetData,
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub whatever: String,
}

pub struct Users {
    data: FetchState<Value>,
}

impl Component for Users {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            data: FetchState::NotFetching,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.data {
            FetchState::NotFetching => html! { "NotFetching" },
            FetchState::Fetching => html! { "Fetching" },
            FetchState::Success(users_data) => view_users(users_data),
            FetchState::Failed(err) => html! { err },
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(Msg::GetData);
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetState(fetch_state) => {
                self.data = fetch_state;

                true
            }
            Msg::GetData => {
                let props = ctx.props().clone();
                ctx.link().send_future(async {
                    match fetch_gql_data(&query_str(props.whatever).await).await {
                        Ok(data) => Msg::SetState(FetchState::Success(data)),
                        Err(err) => Msg::SetState(FetchState::Failed(err)),
                    }
                });

                ctx.link().send_message(Msg::SetState(FetchState::Fetching));

                false
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        ctx.link().send_message(Msg::GetData);

        false
    }
}

fn view_users(users_data: &Value) -> Html {
    let document = gloo_utils::document();
    document.set_title(&format!(
        "{} - {}",
        CFG.get("site.title").unwrap(),
        "Articles"
    ));

    let users_vec = users_data["allUsers"].as_array().unwrap();
    let users = users_vec.iter().enumerate().map(|(i, user)| {
        html! {
            <tr>
                <td> { i + 1 } </td>
                <td> { user["id"].as_str().unwrap() } </td>
                <td> { user["email"].as_str().unwrap() } </td>
                <td> { user["postSignature"].as_str().unwrap_or("") /*use unwrap_or because this is optiona field*/} </td>
                <td> { user["passwordHash"].as_str().unwrap() } </td>
                // <td> { user["posts"]["body"].as_str().unwrap() } </td>
            </tr>
        }
    });

    html! {
        <>
            <h1>{ "all users" }</h1>
            <table class="table-test">
                <thead>
                    <tr>
                        <th> { "Index" } </th>
                        <th> { "Id" } </th>
                        <th> { "Email" } </th>
                        <th> { "Post Signature" } </th>
                        <th> { "Password Hash" } </th>
                    </tr>
                </thead>
                <tbody>
                    { for users }
                </tbody>
            </table>
        </>
    }
}
