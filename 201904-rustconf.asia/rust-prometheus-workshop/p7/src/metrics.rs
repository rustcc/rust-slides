use coarsetime::Instant;
use prometheus::local::*;
use prometheus::*;
use std::cell::{Cell, RefCell};

lazy_static::lazy_static! {
    static ref HTTP_REQUEST_DURATION: HistogramVec = register_histogram_vec!(
        "workshop_http_request_duration",
        "HTTP request durations",
        &["endpoint"],
        exponential_buckets(0.0005, 2.0, 20).unwrap()
    ).unwrap();
    static ref HTTP_REQUEST_DURATION_GET: Histogram = HTTP_REQUEST_DURATION.with_label_values(&["get"]);
    static ref HTTP_REQUEST_DURATION_SET: Histogram = HTTP_REQUEST_DURATION.with_label_values(&["set"]);
    static ref HTTP_REQUEST_DURATION_METRICS: Histogram = HTTP_REQUEST_DURATION.with_label_values(&["metrics"]);

    static ref KEY_FLOW: IntCounterVec = register_int_counter_vec!(
        "workshop_key_flow",
        "Number of flowed key bytes",
        &["op"]
    ).unwrap();
    static ref KEY_FLOW_READ: IntCounter = KEY_FLOW.with_label_values(&["read"]);
    static ref KEY_FLOW_WRITE: IntCounter = KEY_FLOW.with_label_values(&["write"]);

    static ref VALUE_FLOW: IntCounterVec = register_int_counter_vec!(
        "workshop_value_flow",
        "Number of flowed value bytes",
        &["op"]
    ).unwrap();
    static ref VALUE_FLOW_READ: IntCounter = VALUE_FLOW.with_label_values(&["read"]);
    static ref VALUE_FLOW_WRITE: IntCounter = VALUE_FLOW.with_label_values(&["write"]);
}

thread_local! {
    static THREAD_LAST_TICK_TIME: Cell<Instant> = Cell::new(Instant::now());

    pub static TLS_HTTP_REQUEST_DURATION_GET: LocalHistogram = HTTP_REQUEST_DURATION_GET.local();
    pub static TLS_HTTP_REQUEST_DURATION_SET: LocalHistogram = HTTP_REQUEST_DURATION_SET.local();
    pub static TLS_HTTP_REQUEST_DURATION_METRICS: LocalHistogram = HTTP_REQUEST_DURATION_METRICS.local();
    pub static TLS_KEY_FLOW_READ: RefCell<LocalIntCounter> = RefCell::new(KEY_FLOW_READ.local());
    pub static TLS_KEY_FLOW_WRITE: RefCell<LocalIntCounter> = RefCell::new(KEY_FLOW_WRITE.local());
    pub static TLS_VALUE_FLOW_READ: RefCell<LocalIntCounter> = RefCell::new(VALUE_FLOW_READ.local());
    pub static TLS_VALUE_FLOW_WRITE: RefCell<LocalIntCounter> = RefCell::new(VALUE_FLOW_WRITE.local());
}

pub fn may_flush_metrics() {
    THREAD_LAST_TICK_TIME.with(|tls_last_tick| {
        let now = Instant::now();
        let last_tick = tls_last_tick.get();
        if now.duration_since(last_tick).as_f64() < 1.0 {
            return;
        }
        tls_last_tick.set(now);
        TLS_HTTP_REQUEST_DURATION_GET.with(|m| m.flush());
        TLS_HTTP_REQUEST_DURATION_SET.with(|m| m.flush());
        TLS_HTTP_REQUEST_DURATION_METRICS.with(|m| m.flush());
        TLS_KEY_FLOW_READ.with(|m| m.borrow_mut().flush());
        TLS_KEY_FLOW_WRITE.with(|m| m.borrow_mut().flush());
        TLS_VALUE_FLOW_READ.with(|m| m.borrow_mut().flush());
        TLS_VALUE_FLOW_WRITE.with(|m| m.borrow_mut().flush());
    });
}
