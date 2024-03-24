use actix_web::{ App, HttpServer};
mod controller;
mod routes;
mod services;
mod models;
pub mod dtos;
pub mod middlewares;
pub mod db_connection;
#[derive(Debug,Clone)]
pub struct  AppDb{
    pub pool:sqlx::PgPool
}
#[actix_web::main]
async fn main() {
    let pool = db_connection::connect().await;
    let db=AppDb{
        pool
    };
   
    let _=HttpServer::new(move || {
        App::new()
        .service(routes::app_routes::app_routes(
            db.clone()
        ))
    })
    .bind("127.0.0.1:8081")
    .unwrap()
    .run()
    .await
    ;
}
