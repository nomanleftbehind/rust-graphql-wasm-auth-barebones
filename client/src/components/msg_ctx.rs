use crate::{
    hooks::{
        lazy_function,
        me::{MeMe, Variables},
        Me,
    },
    util::console_log::console_log,
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

pub type UserContext = Option<MeMe>;

#[derive(Properties, Debug, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(UserContextProvider)]
pub fn user_context(props: &Props) -> Html {
    let ctx = use_state(|| None);

    let effect_ctx = ctx.clone();

    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let variables = Variables;
                let me = lazy_function::<Me>(variables).await;

                match me.data {
                    Some(res) => match res.me {
                        Some(user) => {
                            console_log!("{:?}", &user);
                            effect_ctx.set(Some(user))
                        }
                        None => (),
                    },
                    None => (),
                };
            });
            || ()
        },
        (),
    );

    html! {
        <ContextProvider<UserContext> context={(*ctx).clone()}>
            {props.children.clone()}
        </ContextProvider<UserContext>>
    }
}
