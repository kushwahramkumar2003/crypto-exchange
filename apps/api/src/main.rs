use std::sync::{Arc, Mutex};
use db::Db::Db;
use poem::{EndpointExt, Route, Server, get, handler, listener::TcpListener, web::Path};

use crate::routes::auth::auth_route;
pub mod routes;
pub mod controller;
pub mod dto;
pub mod utils;
pub mod middlewares;

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let db = Arc::new(Mutex::new(Db::new()));
    let app = Route::new()
    .at("/hello/:name", get(hello))
    .nest("/auth", auth_route())
    .data(db);

    Server::new(TcpListener::bind("0.0.0.0:8080"))
      .run(app)
      .await
}