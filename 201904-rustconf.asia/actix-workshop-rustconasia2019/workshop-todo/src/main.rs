#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;

use std::{env, io};
use actix_web::{http, web, App, HttpServer, HttpRequest};

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

mod api;
mod task;
mod db;
mod schema;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

embed_migrations!("./migrations");

fn main() -> () {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager)
        .expect("Failed to create pool");
    
    let app = move || {
         App::new()
            .data(pool.clone())
            .service(
                web::resource("/").route(
                    web::get().to(api::index)
                )
            )
            .service(
                web::resource("/todo").route(
                        web::post().to(api::create)
                    )
                )
     };

    debug!("Starting server");
    HttpServer::new(app).bind("localhost:8080").unwrap().run();
}
