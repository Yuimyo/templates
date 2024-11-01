use async_graphql::Object;
use sqlx::{Acquire, SqlitePool};

use super::Mutation;

#[Object]
impl Mutation {
    async fn set_value<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        name: String,
        value: String,
    ) -> async_graphql::Result<String> {
        let pool = ctx.data::<SqlitePool>()?;
        let mut tx = pool.begin().await.unwrap();
        let conn = tx.acquire().await.unwrap();

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO memo (name, value)
            VALUES (?, ?)
        "#,
        )
        .bind(&name)
        .bind(&value)
        .execute(&mut *conn)
        .await
        .unwrap();

        tx.commit().await.unwrap();

        Ok(value)
    }
}
