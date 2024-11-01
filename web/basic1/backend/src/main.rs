mod graphql;

use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use graphql::{Mutation, Query};
use rocket::{response::content, routes, State};
use rocket_cors::AllowedOrigins;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tokio::fs;

pub type SampleSchema = Schema<Query, Mutation, EmptySubscription>;

#[rocket::get("/")]
fn graphiql() -> content::RawHtml<String> {
    content::RawHtml(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[rocket::get("/graphql?<query..>")]
async fn graphql_query(schema: &State<SampleSchema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema.inner()).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(schema: &State<SampleSchema>, request: GraphQLRequest) -> GraphQLResponse {
    request.execute(schema.inner()).await
}

async fn load_pool() -> anyhow::Result<SqlitePool> {
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await?;

    let schema_query = fs::read_to_string("./schema.sql").await?;
    let mut conn = pool.acquire().await?;
    sqlx::query(&schema_query).execute(&mut *conn).await?;

    Ok(pool)
}

#[rocket::launch]
async fn rocket() -> _ {
    let pool = load_pool().await.unwrap();

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(pool)
        .finish();

    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::some_regex(&["http://localhost:[0-9]+"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::build()
        .manage(schema)
        .mount("/", routes![graphql_query, graphql_request, graphiql])
        .attach(cors)
}
