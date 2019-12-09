use actix_web::{web, App, HttpServer, Responder};

fn index() -> impl Responder {
    "It works!"
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::resource("/").to(index)))
        .bind("0.0.0.0:8088")?
        .run()
}
