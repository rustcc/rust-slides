use actix_web::{
    web,
    web::{HttpRequest, HttpResponse}
};

use crate::task::{NewTask, Task};
use crate::db;
use super::PgPool;

pub fn index(req: HttpRequest) -> HttpResponse {
     HttpResponse::Ok()
        .content_type("text/plain")
        .body("Hello world!")
}


pub fn create(item: web::Json<NewTask>, pool: web::Data<PgPool>,) -> HttpResponse  {
    let desc = item.description.clone();
    db::create_task(desc, &pool);
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("create sucessful!"))
}
