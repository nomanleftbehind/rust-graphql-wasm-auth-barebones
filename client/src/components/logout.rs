use crate::hooks::{logout_user::Variables, LogoutUser};
use crate::Route;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, Callback};
use yew_router::hooks::use_history;
use yew_router::prelude::History;

use crate::hooks::lazy_function;

#[function_component(Logout)]
pub fn logout() -> Html {
    let history = use_history().unwrap();

    let onclick = {
        Callback::once(move |_| {
            spawn_local(async move {
                let response = lazy_function::<LogoutUser>(Variables).await;

                if response.data.is_some() {
                    history.push(Route::Login)
                }
            })
        })
    };

    html! {
        <button {onclick}> { "Logout" }</button>
    }
}
