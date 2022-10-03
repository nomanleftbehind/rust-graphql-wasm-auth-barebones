use graphql_client::GraphQLQuery;
use serde_json::{json, Value};
use yew::prelude::*;

// use crate::components::nodes::{page_not_found, random_wish_node};
use crate::{
    pages::page_not_found::PageNotFound,
    util::{
        common::{fetch_gql_data, FetchState},
        console_log::console_log,
        constant::CFG,
    },
};

// NaiveDateTime is a custom scalar serialized as string.
// It has to be defined in the scope where the struct under derive is located, next to the query struct will work.
type NaiveDateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/controller.graphql"
)]
struct ControllerData;

async fn query_str(id: String, email: String) -> String {
    let build_query = ControllerData::build_query(controller_data::Variables { email, id });
    let query = json!(build_query);

    query.to_string()
}

pub enum Msg {
    SetState(FetchState<Value>),
    GetData,
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub username: String,
    pub article_slug: String,
}

pub struct Controller {
    data: FetchState<Value>,
}

impl Component for Controller {
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
            FetchState::Success(controller_data) => view_controller(controller_data),
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
                    match fetch_gql_data(&query_str(props.username, props.article_slug).await).await
                    {
                        Ok(data) => Msg::SetState(FetchState::Success(data)),
                        Err(err) => Msg::SetState(FetchState::Failed(err)),
                    }
                });

                ctx.link().send_message(Msg::SetState(FetchState::Fetching));

                false
            }
        }
    }
}

fn view_controller(controller_data: &Value) -> Html {
    if controller_data.is_null() {
        html! { <PageNotFound /> }
    } else {
        // let wish_val = &controller_data["randomWish"];
        // let random_wish = random_wish_node(wish_val);

        let controller = &controller_data["getControllerById"];
        let manufacturer = controller["manufacturer"].as_str().unwrap();
        let document = gloo_utils::document();
        document.set_title(&format!(
            "{} - {}",
            CFG.get("site.title").unwrap(),
            manufacturer,
        ));

        // let article_topics_vec = controller["createdBy2"].as_array().unwrap();
        // let article_topics = article_topics_vec.iter().map(|topic| {
        //     html! {
        //         <a class="s-badge s-badge__sm ml4 mb2"
        //             href={ topic["uri"].as_str().unwrap().to_string() } target="_blank">
        //             { topic["name"].as_str().unwrap() }
        //         </a>
        //     }
        // });

        let model = controller["model"].as_str().unwrap();
        let content_html_section = gloo_utils::document().create_element("section").unwrap();
        content_html_section.set_class_name("fs-body2 mt24");
        content_html_section.set_inner_html(model);
        let content_html_node = Html::VRef(content_html_section.into());

        console_log!("content_html_node: {:?}", content_html_node);

        html! {
            <>
                // { random_wish }
                <controller class="s-card mx24 my12">
                    <h2 class="mb6">
                        <a class="s-tag mr6"
                            href={ controller["model"].as_str().unwrap().to_string() }
                            target="_blank">
                            { controller["createdBy2"]["firstName"].as_str().unwrap() }
                        </a>
                        <a href={ controller["model"].as_str().unwrap().to_string() } target="_blank">
                            { manufacturer }
                        </a>
                    </h2>
                    <p class="fs-caption my6">
                        { controller["updatedAt"].as_str().unwrap() }
                        { " by " }
                        <a href={ format!("/{}", controller["createdBy2"]["lastName"].as_str().unwrap()) }
                            target="_blank">
                            { controller["createdBy2"]["firstName"].as_str().unwrap() }
                            { "@" }
                            { controller["createdBy2"]["lastName"].as_str().unwrap() }
                        </a>
                    </p>
                    // <p class="my6">
                    //     <b class="mr2">{ "Topics:" }</b>
                    //     { for article_topics }
                    // </p>
                    <p class="my6 p4 bg-bronze-lighter">
                        { "💥" }
                        <b class="fc-danger">{ "内容涉及著作权，均归属作者本人。" }</b>
                        { "若非作者注明，默认欢迎转载：请注明出处，及相关链接。" }
                    </p>
                    <p class="fs-body1 my6 p6 bg-gold-lighter">
                        <b class="mr2">{ "Summary:" }</b>
                        { controller["model"].as_str().unwrap() }
                    </p>
                    <link href="/css/night-owl.min.css" rel="stylesheet" />
                    { content_html_node }
                    // <script src="/js/hl.js?132689068675031052"></script>
                    <img class="mt12" src="/imgs/rust-shijian.png" alt={ "Rust 生态与实践" } />
                </controller>
            </>
        }
    }
}
