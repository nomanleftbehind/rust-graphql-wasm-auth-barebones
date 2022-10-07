use yew::{function_component, html, prelude::*};

use crate::hooks::use_query::use_query;
use crate::hooks::{all_users, AllUsers};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct UserProps {
    pub whatever: String,
}

#[function_component(Users)]
pub fn user_list(UserProps { whatever }: &UserProps) -> Html {
    let variables = all_users::Variables {
        whatever: whatever.to_string(),
    };
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
        .into_iter().enumerate()
        .map(|(i, user)| {
            html! {
                <tr>
                    <td> { i + 1 } </td>
                    <td> { user.id } </td>
                    <td> { user.email } </td>
                    <td> { user.post_signature.unwrap_or("".to_string()) /*use unwrap_or because this is optional field*/} </td>
                    <td> { user.password_hash } </td>
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
                        <th> { "Post Signature" } </th>
                        <th> { "Password Hash" } </th>
                    </tr>
                </thead>
                <tbody>
                    { for users }
                </tbody>
            </table>
        </>
    }
}
