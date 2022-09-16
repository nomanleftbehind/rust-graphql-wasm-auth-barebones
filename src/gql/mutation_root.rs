use crate::authentication::register;
use crate::gql::context::ContextExt;
use crate::gql::user::User;
use async_graphql::*;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn register(&self, ctx: &Context<'_>, email: String, password: String) -> Result<User> {
        let pool = ctx.db_pool();

        let mut transaction = pool.begin().await?;
        let result = register(&mut transaction, email, password).await?;
        transaction.commit().await?;
        Ok(result)
    }
}
