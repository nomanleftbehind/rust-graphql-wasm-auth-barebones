use crate::{
    hooks::lazy_function,
    models::{
        login_user::{LoginUserInput, Variables},
        LoginUser,
    },
    util::console_log::console_log,
};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

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
            spawn_local(async move {
                let vars = Variables {
                    login_user_input: LoginUserInput {
                        email: new_email.to_string(),
                        password: new_password.to_string(),
                    },
                };
                lazy_function::<LoginUser>(vars).await;
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
