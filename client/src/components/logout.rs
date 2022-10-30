use crate::{
    models::{logout_user::Variables, LogoutUser},
    Route,
};
use yew::{classes, function_component, html, Callback};
use yew_hooks::use_async;
use yew_router::prelude::Link;

use crate::hooks::lazy_function_result;

#[function_component(Logout)]
pub fn logout() -> Html {
    let state = use_async(async move {
        let res = lazy_function_result::<LogoutUser>(Variables).await;
        res
    });

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.run();
        })
    };

    html! {
        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
            <button class={ "button is-rounded is-small" } {onclick}> { "Logout" }</button>
        </Link<Route>>
    }
}
