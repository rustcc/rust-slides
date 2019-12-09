use prometheus::*;

lazy_static::lazy_static! {
    pub static ref HTTP_REQUEST_DURATION: HistogramVec = register_histogram_vec!(
        "workshop_http_request_duration",
        "HTTP request durations",
        &["endpoint"],
        exponential_buckets(0.0005, 2.0, 20).unwrap()
    ).unwrap();

    pub static ref KEY_FLOW: CounterVec = register_counter_vec!(
        "workshop_key_flow",
        "Number of flowed key bytes",
        &["op"]
    ).unwrap();

    pub static ref VALUE_FLOW: CounterVec = register_counter_vec!(
        "workshop_value_flow",
        "Number of flowed value bytes",
        &["op"]
    ).unwrap();
}
