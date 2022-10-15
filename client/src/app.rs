use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::nav::Nav;
use crate::pages::{
    author_list::AuthorList, /*controller::Controller, controllers::Controllers,*/ home::Home,
    page_not_found::PageNotFound, post_list::PostList, users::Users,
};
use crate::pages::login::Login;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/posts")]
    Posts,
    #[at("/authors")]
    Authors,
    #[at("/users/:whatever")]
    Users { whatever: String },
    // #[at("/controller/:id/:email")]
    // Controller { id: String, email: String },
    // #[at("/users/:id/controllers")]
    // Controllers { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Nav />

            <main>
                <Switch<Route> render={Switch::render(switch)} />
            </main>
            <footer class="footer">
                <div class="content has-text-centered">
                    { "Powered by " }
                    <a href="https://yew.rs">{ "Yew" }</a>
                    { " using " }
                    <a href="https://bulma.io">{ "Bulma" }</a>
                    { " and images from " }
                    <a href="https://unsplash.com">{ "Unsplash" }</a>
                </div>
            </footer>
        </BrowserRouter>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Login => {
            html! { <Login /> }
        }
        Route::Posts => {
            html! { <PostList /> }
        }
        Route::Authors => {
            html! { <AuthorList /> }
        }
        // Route::Controllers { id } => {
        //     html! { <Controllers user_id={id.clone()} /> }
        // }
        Route::Users { whatever } => {
            html! { <Users whatever={whatever.clone()} /> }
        }
        // Route::Controller { id, email } => {
        //     html! { <Controller username={id.clone()} article_slug={email.clone()} /> }
        // }
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}
