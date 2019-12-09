use std::{thread, time::Duration};
use reqwest;

fn main() {
    loop {
        
    let gps = reqwest::get("http://34.217.11.6:8080/7ff796351a7c446795b5a0babd1dc50a/get/V3")
    .unwrap()
    .text()
    .unwrap();

    println!("GPS: {:?}", gps);

    thread::sleep(Duration::from_millis(1000));
    }
}
