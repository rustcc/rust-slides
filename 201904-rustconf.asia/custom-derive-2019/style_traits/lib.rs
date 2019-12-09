use std::fmt;

pub trait ToCss {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write;
}

impl ToCss for i32 {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        write!(dest, "{}", self)
    }
}

impl ToCss for f32 {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        write!(dest, "{}", self)
    }
}
