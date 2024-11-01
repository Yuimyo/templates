use async_graphql::Object;
use sqlx::SqlitePool;

use super::Query;

#[Object]
impl Query {
    async fn hoge(&self) -> i32 {
        42
    }

    async fn get_value<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        name: String,
    ) -> async_graphql::Result<Option<String>> {
        let pool = ctx.data::<SqlitePool>()?;
        let result = sqlx::query_as(
            r#"
                SELECT value FROM memo WHERE name = ?
            "#,
        )
        .bind(name)
        .fetch_one(pool)
        .await;

        match result {
            Ok((value,)) => Ok(Some(value)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
