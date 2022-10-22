use crate::hooks::{
    lazy_function,
    me::{MeMe, Variables},
    Me,
};
use crate::util::console_log::console_log;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Message {
    pub inner: Option<MeMe>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
}

#[derive(Properties, Debug, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(MessageProvider)]
pub fn message_provider(props: &Props) -> Html {
    let ctx = use_state(|| Message { inner: None });

    let effect_ctx = ctx.clone();

    use_effect_with_deps(
        move |asd| {
            console_log!("asd: {:?}", &asd);

            spawn_local(async move {
                let variables = Variables;

                let me = lazy_function::<Me>(variables).await;
                console_log!("vars: {:?}", &me);

                match me.data {
                    Some(x) => {
                        console_log!("{:?}", &x.me);
                        effect_ctx.set(Message { inner: x.me })
                    }
                    None => (),
                };
            });
            || ()
        },
        (),
    );

    html! {
        <ContextProvider<Message> context={(*ctx).clone()}>
            {props.children.clone()}
        </ContextProvider<Message>>
    }
}
