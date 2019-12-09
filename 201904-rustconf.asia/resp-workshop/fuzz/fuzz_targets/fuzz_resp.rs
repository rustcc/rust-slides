#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate resp;
extern crate bytes;

use bytes::BytesMut;

use resp::Message;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    match Message::parse(&mut BytesMut::from(data)) {
        Err(err) => {
            // println!("{}, {}",err.cause(), err.backtrace());
        }
        Ok(Some(_)) => {},
        Ok(None) => {},
    };
});
