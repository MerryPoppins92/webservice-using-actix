// module imports
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

// configure route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

// Configure handler
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello les frerots")
}

// INstantiate and run the HTTP server
#[actix_rt::main]
async fn main() -> io::Result<()> {
    //  construct app and configure routes
    let app = move || App::new().configure(general_routes);
    // start HTTP server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}