use crate::{
    components::msg_ctx::UserContext,
    hooks::{all_users, use_query, AllUsers},
    util::console_log::console_log,
};
use yew::{function_component, html, prelude::*};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct UserProps {
    pub whatever: String,
}

#[function_component(Users)]
pub fn users(UserProps { whatever }: &UserProps) -> Html {
    let user = use_context::<UserContext>().expect("no ctx found");
    console_log!("current user: {:?}", user);

    let variables = all_users::Variables {
        whatever: whatever.to_string(),
    };
    // let asjh = AllUsers::build_query(variables;)
    let get_all_users = use_query::<AllUsers>(variables);

    if get_all_users.data.is_none() {
        return html! {
            <>
                <h1>{"Query Failed!"}</h1>
            </>
        };
    }
    let users = get_all_users
        .data
        .unwrap()
        .all_users
        .into_iter()
        .enumerate()
        .map(|(i, user)| {
            html! {
                <tr>
                    <td> { i + 1 } </td>
                    <td> { user.id } </td>
                    <td> { user.email } </td>
                    <td> { user.password } </td>
                    <td> { user.first_name } </td>
                    <td> { user.last_name } </td>
                </tr>
            }
        });

    html! {
        <>
            <h1>{ "all users" }</h1>
            <table class="table-test">
                <thead>
                    <tr>
                        <th> { "Index" } </th>
                        <th> { "Id" } </th>
                        <th> { "Email" } </th>
                        <th> { "Password" } </th>
                        <th> { "First Name" } </th>
                        <th> { "Last Name" } </th>
                    </tr>
                </thead>
                <tbody>
                    { for users }
                </tbody>
            </table>
        </>
    }
}
