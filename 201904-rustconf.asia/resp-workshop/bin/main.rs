use resp::Message;
use bytes::BytesMut;


fn main() {
    let sdata = "+baka for you\r\n".as_bytes();
    for _ in 0..100_000_000 {
        Message::parse(&mut BytesMut::from(&sdata[..])).unwrap().unwrap();
    }
}
