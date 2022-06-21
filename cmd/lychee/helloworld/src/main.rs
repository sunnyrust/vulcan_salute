use actix_files::Files;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use tera::Tera;
use vulcan_salute::web::*;
mod controllers;
use controllers::index::Index;
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello vulcan salute!\nLive long and prosper! \nPeace and long life")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let url = "0.0.0.0:9728";
    println!("Listening on: {}, open browser and visit have a try!", url);
    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        App::new()
        .data(tera)
        .wrap(middleware::Logger::default()) // enable logger
        .service(Files::new("/static", "static/"))
        .service(web::resource("/").route(web::get().to(Index::index)))
        .service(web::resource("/hello").route(web::get().to(hello)))
        .service(web::scope("").wrap(web_error::error_handlers()))
    })
    .bind(url)?
    .run()
    .await
}

