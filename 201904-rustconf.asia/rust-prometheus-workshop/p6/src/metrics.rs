use prometheus::*;

lazy_static::lazy_static! {
    pub static ref HTTP_REQUEST_DURATION: HistogramVec = register_histogram_vec!(
        "workshop_http_request_duration",
        "HTTP request durations",
        &["endpoint"],
        exponential_buckets(0.0005, 2.0, 20).unwrap()
    ).unwrap();
    pub static ref HTTP_REQUEST_DURATION_GET: Histogram = HTTP_REQUEST_DURATION.with_label_values(&["get"]);
    pub static ref HTTP_REQUEST_DURATION_SET: Histogram = HTTP_REQUEST_DURATION.with_label_values(&["set"]);
    pub static ref HTTP_REQUEST_DURATION_METRICS: Histogram = HTTP_REQUEST_DURATION.with_label_values(&["metrics"]);

    pub static ref KEY_FLOW: IntCounterVec = register_int_counter_vec!(
        "workshop_key_flow",
        "Number of flowed key bytes",
        &["op"]
    ).unwrap();
    pub static ref KEY_FLOW_READ: IntCounter = KEY_FLOW.with_label_values(&["read"]);
    pub static ref KEY_FLOW_WRITE: IntCounter = KEY_FLOW.with_label_values(&["write"]);

    pub static ref VALUE_FLOW: IntCounterVec = register_int_counter_vec!(
        "workshop_value_flow",
        "Number of flowed value bytes",
        &["op"]
    ).unwrap();
    pub static ref VALUE_FLOW_READ: IntCounter = VALUE_FLOW.with_label_values(&["read"]);
    pub static ref VALUE_FLOW_WRITE: IntCounter = VALUE_FLOW.with_label_values(&["write"]);
}
