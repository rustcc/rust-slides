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
    let timer = coarsetime::Instant::now();
    may_flush_metrics();

    let key = &*param;
    for _ in 0..100 {
        // Simulating a very fast api endpoint, or a lot of metrics
        TLS_KEY_FLOW_READ.with(|m| m.borrow_mut().inc_by(key.len() as i64));
    }

    let respond = map
        .read_handle
        .get_and(key, |vals| {
            for _ in 0..100 {
                TLS_VALUE_FLOW_READ.with(|m| m.borrow_mut().inc_by(vals[0].len() as i64));
            }
            format!("Key {} = {}", key, vals[0])
        })
        .unwrap_or_else(|| format!("Key {} does not exist", key));

    TLS_HTTP_REQUEST_DURATION_GET.with(|m| m.observe(timer.elapsed().as_f64()));

    respond
}

fn api_set(map: web::Data<KvMap>, param: web::Path<(String, String)>) -> impl Responder {
    let timer = coarsetime::Instant::now();
    may_flush_metrics();

    let mut write_handle = map.write_handle.lock().unwrap();
    let (key, value) = (&param.0, &param.1);
    for _ in 0..100 {
        TLS_KEY_FLOW_READ.with(|m| m.borrow_mut().inc_by(key.len() as i64));
        TLS_VALUE_FLOW_READ.with(|m| m.borrow_mut().inc_by(value.len() as i64));
    }

    write_handle.update(key.to_owned(), value.to_owned());
    write_handle.refresh();

    let respond = format!("Put key {} = {}", key, value);

    TLS_HTTP_REQUEST_DURATION_SET.with(|m| m.observe(timer.elapsed().as_f64()));

    respond
}

fn api_metrics() -> impl Responder {
    use prometheus::{Encoder, TextEncoder};

    let timer = coarsetime::Instant::now();
    may_flush_metrics();

    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    let respond = String::from_utf8(buffer).unwrap();

    TLS_HTTP_REQUEST_DURATION_METRICS.with(|m| m.observe(timer.elapsed().as_f64()));

    respond
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
