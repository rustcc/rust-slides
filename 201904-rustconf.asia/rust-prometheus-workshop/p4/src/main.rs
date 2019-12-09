mod metrics;

use actix_web::{web, App, HttpServer, Responder};
use evmap::{ReadHandle, WriteHandle};
use std::sync::{Arc, Mutex};

use self::metrics::*;

struct KvMap {
    pub read_handle: ReadHandle<String, String>,
    pub write_handle: Arc<Mutex<WriteHandle<String, String>>>,
}

fn api_get(map: web::Data<KvMap>, param: web::Path<String>) -> impl Responder {
    let _timer = HTTP_REQUEST_DURATION
        .with_label_values(&["get"])
        .start_timer();

    let key = &*param;
    KEY_FLOW
        .with_label_values(&["read"])
        .inc_by(key.len() as f64);

    map.read_handle
        .get_and(key, |vals| {
            VALUE_FLOW
                .with_label_values(&["read"])
                .inc_by(vals[0].len() as f64);
            format!("Key {} = {}", key, vals[0])
        })
        .unwrap_or_else(|| format!("Key {} does not exist", key))
}

fn api_set(map: web::Data<KvMap>, param: web::Path<(String, String)>) -> impl Responder {
    let _timer = HTTP_REQUEST_DURATION
        .with_label_values(&["set"])
        .start_timer();

    let mut write_handle = map.write_handle.lock().unwrap();
    let (key, value) = (&param.0, &param.1);
    KEY_FLOW
        .with_label_values(&["write"])
        .inc_by(key.len() as f64);
    VALUE_FLOW
        .with_label_values(&["write"])
        .inc_by(value.len() as f64);

    write_handle.update(key.to_owned(), value.to_owned());
    write_handle.refresh();
    format!("Put key {} = {}", key, value)
}

fn api_metrics() -> impl Responder {
    use prometheus::{Encoder, TextEncoder};

    let _timer = HTTP_REQUEST_DURATION
        .with_label_values(&["metrics"])
        .start_timer();

    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
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
            .service(web::resource("/metrics").to(api_metrics))
    })
    .bind("0.0.0.0:8088")?
    .run()
}
