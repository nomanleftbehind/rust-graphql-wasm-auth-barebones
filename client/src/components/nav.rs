use crate::{
    components::{logout::Logout, msg_ctx::UserContext},
    util::console_log::console_log,
};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Nav)]
pub fn nav() -> Html {
    let user = use_context::<UserContext>().expect("no ctx found");

    let user_clone = user.clone();
    {
        use_effect_with_deps(
            move |seed| {
                console_log!("current user: {:?}", seed);

                || {}
            },
            user_clone,
        );
    }

    // let user_cloned = user.clone();
    // use_effect_with_deps(
    //     move |_| {
    //         let user_cloned2 = user_cloned.clone();
    //         // ...
    //         || ()
    //     },
    //     user_cloned.clone(), // dependents
    // );

    let navbar_active = use_state_eq(|| false);

    let toggle_navbar = {
        let navbar_active = navbar_active.clone();

        Callback::from(move |_| {
            navbar_active.set(!*navbar_active);
        })
    };

    let active_class = if !*navbar_active { "is-active" } else { "" };

    // let user = use_context::<UserContext>().expect("no ctx found");

    html! {
        <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <h1 class="navbar-item is-size-3">{ "Yew Blog" }</h1>

                <button class={classes!("navbar-burger", "burger", active_class)}
                    aria-label="menu" aria-expanded="false"
                    onclick={toggle_navbar}
                >
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </button>
            </div>
            <div class={classes!("navbar-menu", active_class)}>
                <div class="navbar-start">
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                        { "Home" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Posts}>
                        { "Posts" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Users {whatever: "kasld".to_string()}}>
                        { "Users" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Login}>
                        { "Login" }
                    </Link<Route>>

                    if let Some(u) = user {
                        <div class="navbar-item has-dropdown is-hoverable">
                            <div class="navbar-link">
                                { u.email }
                            </div>
                            <div class="navbar-dropdown">
                                <div classes={classes!("navbar-item")}>
                                    <Logout/>
                                </div>
                            </div>
                        </div>
                    }
                </div>
            </div>
        </nav>
    }
}
