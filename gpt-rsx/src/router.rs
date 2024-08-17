///ROUTER FOR THE QUERIES
/// --------------------
/// Could use an AI Gateway etc
/// This whole thing could be a Worker actually
/// 

 pub mod data;
 pub mod llm_processing;

use warp::Filter;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use uuid::Uuid;

async fn store_input(pool: SqlitePool, input: InputData) -> Result<impl warp::Reply, warp::Rejection> {
    let id = Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        INSERT INTO input (id, input_text)
        VALUES (?1, ?2)
        "#,
    )
    .bind(&id)
    .bind(&input.input_text)
    .execute(&pool)
    .await
    .map_err(|_| warp::reject::custom("Failed to store input"))?;

    Ok(warp::reply::json(&id))
}

async fn store_output(pool: SqlitePool, output: OutputData) -> Result<impl warp::Reply, warp::Rejection> {
    sqlx::query(
        r#"
        INSERT INTO output (id, output_text)
        VALUES (?1, ?2)
        "#,
    )
    .bind(&output.id)
    .bind(&output.output_text)
    .execute(&pool)
    .await
    .map_err(|_| warp::reject::custom("Failed to store output"))?;

    Ok(warp::reply::json(&output.id))
}

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite://gpt_pipeline.db").await.unwrap();
    initialize_database(&pool).await.unwrap();

    let pool_filter = warp::any().map(move || pool.clone());

    let store_input_route = warp::post()
        .and(warp::path("store_input"))
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(store_input);

    let store_output_route = warp::post()
        .and(warp::path("store_output"))
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(store_output);

    let routes = store_input_route.or(store_output_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}