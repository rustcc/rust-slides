mod simdfind;
use bytes::BytesMut;
#[macro_use]
extern crate failure;
use failure::Error;

use std::usize;

pub const RESP_INLINE: u8 = 0u8;
pub const RESP_STRING: u8 = b'+';
pub const RESP_INT: u8 = b':';
pub const RESP_ERROR: u8 = b'-';
pub const RESP_BULK: u8 = b'$';
pub const RESP_ARRAY: u8 = b'*';

pub const BYTE_CR: u8 = b'\r';
pub const BYTE_LF: u8 = b'\n';

#[derive(Debug, Fail)]
pub enum RespError {
    #[fail(display = "invalid message")]
    BadMessage,

    #[fail(display = "invalid bulk string size")]
    BadBulkSize,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub rtype: RespType,
    pub data: BytesMut,
}

struct MsgPack {
    rtype: RespType,
    size: usize,
}

impl Message {
    fn parse_inner(cursor: usize, src: &[u8]) -> Result<Option<MsgPack>, Error> {
        // println!("");
        // println!(
        //     "input data is {:?} cursor {:?} avaliable data {:?}",
        //     String::from_utf8_lossy(&src),
        //     cursor,
        //     String::from_utf8_lossy(&src[cursor..])
        // );
        let pos = if let Some(p) = simdfind::find_lf_simd(&src[cursor..]) {
            // println!("pos is {}", p);
            p
        } else {
            return Ok(None);
        };

        if pos == 0 {
            return Err(RespError::BadMessage.into());
        }

        // detect pos -1 is CR
        if src[cursor + pos - 1] != BYTE_CR {
            // should detect inline
            return Err(RespError::BadMessage.into());
        }

        // println!("data is {:?}", String::from_utf8_lossy(&src[cursor..]));
        match src[cursor] {
            RESP_STRING => {
                return Ok(Some(MsgPack {
                    rtype: RespType::String,
                    size: pos + 1,
                }));
            }
            RESP_INT => {
                return Ok(Some(MsgPack {
                    rtype: RespType::Integer,
                    size: pos + 1,
                }));
            }
            RESP_ERROR => {
                return Ok(Some(MsgPack {
                    rtype: RespType::Error,
                    size: pos + 1,
                }));
            }
            RESP_BULK => {
                let csize = match btoi::btoi::<isize>(&src[cursor + 1..cursor+pos - 1]) {
                    Ok(csize) => csize,
                    Err(_err) => return Err(RespError::BadBulkSize.into()),
                };
                // println!("total_size = pos({}) + csize({})", pos, csize);

                if csize == -1 {
                    return Ok(Some(MsgPack {
                        rtype: RespType::Bulk { head_end: 4+cursor },
                        size: 5,
                    }));
                } else if csize < 0 {
                    return Err(RespError::BadMessage.into());
                }

                let total_size = (pos + 1) + (csize as usize) + 2;

                if src.len() >= cursor + total_size {
                    return Ok(Some(MsgPack {
                        rtype: RespType::Bulk { head_end: pos +cursor },
                        size: total_size,
                    }));
                }
            }
            RESP_ARRAY => {
                let csize = match btoi::btoi::<isize>(&src[cursor + 1..cursor+pos - 1]) {
                    Ok(csize) => csize,
                    Err(_err) => return Err(RespError::BadBulkSize.into()),
                };
                // println!("cisze {:?}", csize);
                if csize == -1 {
                    return Ok(Some(MsgPack {
                        rtype: RespType::Bulk { head_end: 4 },
                        size: 5,
                    }));
                } else if csize < 0 {
                    return Err(RespError::BadMessage.into());
                }
                let mut mycursor = cursor + pos + 1;
                let mut items = Vec::new();
                for _ in 0..csize {
                    if let Some(MsgPack { rtype, size }) = Self::parse_inner(mycursor, &src[..])? {
                        // println!("parse_inner MsgPack rtype={:?} size={:?}", rtype, size);
                        mycursor += size;
                        items.push(rtype);
                    } else {
                        return Ok(None);
                    }
                }
                return Ok(Some(MsgPack {
                    rtype: RespType::Array {
                        head_end: pos,
                        items,
                    },
                    size: mycursor - cursor,
                }));
            }
            _ => {
                return Err(RespError::BadMessage.into());
            }
        }

        Ok(None)
    }

    pub fn parse(src: &mut BytesMut) -> Result<Option<Message>, Error> {
        let rslt = match Self::parse_inner(0, &src[..]) {
            Ok(r) => r,
            Err(err) => {
                if let Some(pos) = simdfind::find_lf_simd(&src[..]) {
                    src.advance(pos + 1);
                }
                return Err(err);
            }
        };

        if let Some(MsgPack { size, rtype }) = rslt {
            let data = src.split_to(size);
            return Ok(Some(Message { data, rtype }));
        }
        Ok(None)
    }
}

#[derive(Debug, Clone)]
pub enum RespType {
    String,
    Error,
    Integer,
    Bulk {
        head_end: usize,
    },
    Array {
        head_end: usize,
        items: Vec<RespType>,
    },
}

#[test]
fn test_parse() {
    let data = b"*2\r\n$3\r\nget\r\n$4\r\nab\nc\r\n";
    let mut src = BytesMut::from(&data[..]);
    Message::parse(&mut src).unwrap().unwrap();
}
