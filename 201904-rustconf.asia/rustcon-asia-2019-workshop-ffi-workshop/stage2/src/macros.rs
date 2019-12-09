#[macro_export]
macro_rules! try_ffi {
    ($t:expr,$s:expr, $e:expr) => {{
        if $t == 1 {
            return Ok($s);
        }
        return Err($e);
    }};
}
