use std::ffi::CString;
use std::os::raw::c_char;

static MSG: &'static str = "[mozMeetup] hello from rust";

#[no_mangle]
pub fn hello() -> *mut c_char {
	let s = CString::new(MSG).unwrap();
  s.into_raw()
}

#[no_mangle]
pub fn get_len() -> usize {
  MSG.len()
}

fn main() {
  // blank intentionally
}
