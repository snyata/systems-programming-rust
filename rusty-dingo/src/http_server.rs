/// HTTP_SERVER
/// Routes Traffic and receives calls from the scanner and keylogger
/// Can be integrated into other modules
/// 

use warp::Filter;
use sqlx::{Error, SqlitePool};
use serde::Serialize;
use std::convert::Infallible;
use scanner::{ ScanResult, get_scan_results, scan_cidr };


/// START BASIC HTTP SERVER ON LOCALHOST PORT 3030 
/// PASS DB POOL

pub async fn run_server(db_pool: SqlitePool) {
    let db_pool_filter = warp::any().map(move || db_pool.clone());

    // results route
    let results_route = warp::path!("api" / "v1" / "results")
        .and(warp::get())
        .and(db_pool_filter.clone())
        .and_then(scan_cidr)
        .await?
        .and_then(get_scan_results)
        .await?;


    let hello_route = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let routes = results_route.or(hello_route);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
