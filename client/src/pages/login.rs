use crate::hooks::{login_user, LoginUser};
use crate::util::common::GraphQLResponse;
use crate::util::{common::build_request, console_log::console_log};
use graphql_client::GraphQLQuery;
use serde_json::json;
use web_sys::HtmlInputElement;
use serde::Deserialize;

use yew::prelude::*;
// use yew_hooks::prelude::*;
// use yew_router::prelude::*;

// use crate::components::list_errors::ListErrors;
// use crate::hooks::use_user_context;
// use crate::routes::AppRoute;
// use crate::services::auth::*;
// use crate::types::{LoginInfo, LoginInfoWrapper};

// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Clone, Debug, Default)]
// #[serde(rename_all = "camelCase")]
// pub struct LoginInfo {
//     pub email: String,
//     pub password: String,
// }

// #[derive(Serialize, Deserialize, Clone, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct LoginInfoWrapper {
//     pub user: LoginInfo,
// }

#[derive(Debug, Deserialize)]
struct MaybeWorks {
    login: String,
}

/// Login page
#[function_component(Login)]
pub fn login() -> Html {
    console_log!("log {:?}", &LoginUser);
    let new_email = use_state(|| "doma@emissions.com".to_string());
    let new_password = use_state(|| "everythinghastostartsomewhere".to_string());

    let onsubmit = {
        let new_email = new_email.clone();
        let new_password = new_password.clone();

        Callback::from(move |e: FocusEvent| {
            e.prevent_default(); /* Prevent event propagation */
            let new_email = new_email.clone();
            let new_password = new_password.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let variables = login_user::Variables {
                    login: login_user::LoginUser {
                        email: new_email.to_string(),
                        password: new_password.to_string(),
                    },
                };
                let request_body = LoginUser::build_query(variables);
                let request_json = &json!(request_body);
                let request = build_request(request_json).await;
                console_log!("request: {:?}", &request);
                if let Ok(response) = request {
                    // response.headers().append(reqwest::header::ACCESS_CONTROL_ALLOW_CREDENTIALS, http::header::HeaderValue::from_bytes(b"true").unwrap());
                    console_log!("response headers: {:?}", response.headers());
                    console_log!("response: {:?}", &response);
                    
                    let json = response.json::<GraphQLResponse<MaybeWorks>>().await;
                    console_log!("json: {:?}", &json);
                    match json {
                        Ok(_responser) => {
                            console_log!("responser: {:?}", &_responser);
                            
                            ()},
                            Err(_error) => {
                                console_log!("error: {:?}", &_error);
                            ()},
                    }
                }
            })
        })
    };

    let oninput_email = {
        let new_email = new_email.clone();
        console_log!("new email: {:?}", &new_email);
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*new_email).clone();
            info = input.value();
            new_email.set(info)
        })
    };
    let oninput_password = {
        let new_password = new_password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*new_password).clone();
            info = input.value();
            new_password.set(info)
        })
    };

    html! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">{ "Sign In" }</h1>
                        // <p class="text-xs-center">
                        //     <Link<AppRoute> to={AppRoute::Register}>
                        //         { "Need an account?" }
                        //     </Link<AppRoute>>
                        // </p>
                        <form {onsubmit}>
                            <fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="email"
                                        placeholder="Email"
                                        value={(*new_email).clone()}
                                        oninput={oninput_email}
                                        />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="password"
                                        placeholder="Password"
                                        value={(*new_password).clone()}
                                        oninput={oninput_password}
                                        />
                                </fieldset>
                                <button
                                    class="btn btn-lg btn-primary pull-xs-right"
                                    type="submit"
                                    disabled=false>
                                    { "Sign in" }
                                </button>
                            </fieldset>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
