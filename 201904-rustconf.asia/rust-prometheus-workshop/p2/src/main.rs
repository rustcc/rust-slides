use actix_web::{web, App, HttpServer, Responder};
use evmap::{ReadHandle, WriteHandle};
use std::sync::{Arc, Mutex};

struct KvMap {
    pub read_handle: ReadHandle<String, String>,
    pub write_handle: Arc<Mutex<WriteHandle<String, String>>>,
}

fn api_get(map: web::Data<KvMap>, param: web::Path<String>) -> impl Responder {
    let key = &*param;
    map.read_handle
        .get_and(key, |vals| format!("Key {} = {}", key, vals[0]))
        .unwrap_or_else(|| format!("Key {} does not exist", key))
}

fn api_set(map: web::Data<KvMap>, param: web::Path<(String, String)>) -> impl Responder {
    let mut write_handle = map.write_handle.lock().unwrap();
    let (key, value) = (&param.0, &param.1);

    write_handle.update(key.to_owned(), value.to_owned());
    write_handle.refresh();
    format!("Put key {} = {}", key, value)
}

fn main() -> std::io::Result<()> {
    let (map_r, map_w_st) = evmap::new::<String, String>();
    let map_w = Arc::new(Mutex::new(map_w_st));

    HttpServer::new(move || {
        App::new()
            .data(KvMap {
                read_handle: map_r.clone(),
                write_handle: map_w.clone(),
            })
            .service(web::resource("/get/{key}").to(api_get))
            .service(web::resource("/set/{key}/{value}").to(api_set))
    })
    .bind("0.0.0.0:8088")?
    .run()
}
