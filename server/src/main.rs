mod models;
mod todos;
mod recipies;

use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use warp::{http::Method, Filter};


use todos::{
    // handlers,
    filters,
};


/// Provides a RESTful web server managing some Todos.
///
/// API will be:
///
/// - `GET /todos`: return a JSON list of Todos.
/// - `POST /todos`: create a new Todo.
/// - `PUT /todos/:id`: update a specific Todo.
/// - `DELETE /todos/:id`: delete a specific Todo.
/// 
/// - `GET /recipe`: return a JSON list of Todos.
/// - `POST /recipe`: create a new Todo.
/// - `PUT /recipe/:id`: update a specific Todo.
/// - `DELETE /recipe/:id`: delete a specific Todo.
/// 
#[tokio::main]
async fn main() {
    dotenv().ok();

    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "todos=info");
    }

    let db_url = env::var("DATABASE_URL").expect("Failed to find 'DATABASE_URL'");

    pretty_env_logger::init();

    // Postgres
    let pool = PgPool::new(&db_url)
        .await
        .expect("Failed to connect to pool");

    let api = filters::todos(pool.clone()).with(warp::log("todos"))
        .or(recipies::filters::recipies(pool.clone()).with(warp::log("recipies")));

    let cors = warp::cors().allow_methods(&[Method::GET, Method::POST, Method::DELETE]);

    // View access logs by setting `RUST_LOG=todos`.
    let routes = api.with(cors);
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
