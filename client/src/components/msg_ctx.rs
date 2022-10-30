use crate::models::{
    me::{MeMe, Variables},
    Me,
};

use yew::prelude::*;
use yew_hooks::{use_async, use_mount};

use crate::hooks::lazy_function_result;

pub type UserContext = Option<MeMe>;

#[derive(Properties, Debug, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(UserContextProvider)]
pub fn user_context(props: &Props) -> Html {
    let ctx = use_state(|| None);

    let current_user = use_async(async move {
        let variables = Variables;
        let me = lazy_function_result::<Me>(variables).await;
        me
    });

    {
        let current_user = current_user.clone();
        use_mount(move || {
            current_user.run();
        });
    }

    {
        let user_ctx = ctx.clone();
        use_effect_with_deps(
            move |current_user| {
                if let Some(user_info) = &current_user.data {
                    if let Some(data) = user_info.clone() {
                        if let Some(user) = data.me {
                            user_ctx.set(Some(user));
                        }
                    }
                }
                || ()
            },
            current_user,
        )
    }

    html! {
        <ContextProvider<UserContext> context={(*ctx).clone()}>
            {props.children.clone()}
        </ContextProvider<UserContext>>
    }
}
