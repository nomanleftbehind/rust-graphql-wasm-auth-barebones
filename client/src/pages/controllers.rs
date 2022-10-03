use graphql_client::GraphQLQuery;
use serde_json::{json, Value};
use yew::prelude::*;

use crate::util::{
    common::{fetch_gql_data, FetchState},
    constant::CFG,
};

type NaiveDateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/controllers_by_creator.graphql"
)]
struct ControllersByCreatorData;

async fn query_str(user_id: String) -> String {
    let build_query =
        ControllersByCreatorData::build_query(controllers_by_creator_data::Variables { user_id });
    let query = json!(build_query);

    query.to_string()
}

pub enum Msg {
    SetState(FetchState<Value>),
    GetData,
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub user_id: String,
}

pub struct Controllers {
    data: FetchState<Value>,
}

impl Component for Controllers {
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
            FetchState::Success(controllers_data) => view_controllers(controllers_data),
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
                    match fetch_gql_data(&query_str(props.user_id).await).await {
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

fn view_controllers(controllers_data: &Value) -> Html {
    let document = gloo_utils::document();
    document.set_title(&format!(
        "{} - {}",
        CFG.get("site.title").unwrap(),
        "Articles"
    ));

    let controllers_vec = controllers_data["getControllersbyAuthor"]
        .as_array()
        .unwrap();
    let controllers = controllers_vec.iter().enumerate().map(|(i, controller)| {
        html! {
            <tr>
                <td> { i + 1 } </td>
                <td> { controller["manufacturer"].as_str().unwrap() } </td>
                <td> { controller["model"].as_str().unwrap() } </td>
                <td> { controller["serialNumber"].as_str().unwrap() } </td>
                <td> { controller["function"].as_str().unwrap() } </td>
                <td> { controller["createdAt"].as_str().unwrap() } </td>
                <td> { controller["updatedAt"].as_str().unwrap() } </td>
                <td> { controller["createdBy2"]["email"].as_str().unwrap() } </td>
            </tr>
        }
    });

    html! {
        <table class="table-test">
            <thead>
                <tr>
                    <th> { "Index" } </th>
                    <th> { "Manufacturer" } </th>
                    <th> { "Model" } </th>
                    <th> { "Serial Number" } </th>
                    <th> { "Function" } </th>
                    <th> { "Created At" } </th>
                    <th> { "Updated At" } </th>
                    <th> { "Created By" } </th>
                </tr>
            </thead>
            <tbody>
                { for controllers }
            </tbody>
        </table>
    }
}
