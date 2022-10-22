use crate::components::{msg_ctx::UserContextProvider, nav::Nav};
use crate::pages::{
    home::Home, login::Login, page_not_found::PageNotFound, post_list::PostList, users::Users,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/posts")]
    Posts,
    #[at("/users/:whatever")]
    Users { whatever: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <UserContextProvider>
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
        </UserContextProvider>
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
        Route::Users { whatever } => {
            html! { <Users whatever={whatever.clone()} /> }
        }
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}
