use actix_web::{App, HttpServer};
pub mod services;
pub mod error_handler;
pub mod db;
pub mod schema;
pub mod models;


mod api;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    
    config::init_config();
    db::init();
    let port = config::get_server_port();
    println!("Starting server on port {port}");
    HttpServer::new(|| {
        App::new()
        .configure(api::init_routes)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

