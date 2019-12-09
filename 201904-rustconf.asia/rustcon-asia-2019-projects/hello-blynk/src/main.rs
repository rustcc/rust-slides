use std::{thread, time::Duration};
use reqwest;

fn main() {
    loop {
        
    let res_on = reqwest::get("http://34.217.11.6:8080/7ff796351a7c446795b5a0babd1dc50a/update/V0?value=1024").unwrap();
    println!("Turning V0 ON Status: {}", res_on.status());

    thread::sleep(Duration::from_millis(1000));

    let res_off = reqwest::get("http://34.217.11.6:8080/7ff796351a7c446795b5a0babd1dc50a/update/V0?value=0").unwrap();
    println!("Turning V0 OFF Status: {}", res_off.status());
    thread::sleep(Duration::from_millis(1000));
    }
}
