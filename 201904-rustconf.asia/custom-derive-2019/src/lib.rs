use std::fmt;
use style_derive::ToCss;
use style_traits::ToCss;

#[derive(ToCss)]
pub enum WhiteSpace {
    Normal,
    Nowrap,
    Pre,
    PreWrap,
    PreLine,
}

/*
impl ToCss for WhiteSpace {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        match self {
            WhiteSpace::Normal => dest.write_str("normal"),
            WhiteSpace::Nowrap => dest.write_str("nowrap"),
            WhiteSpace::Pre => dest.write_str("pre"),
            WhiteSpace::PreWrap => dest.write_str("pre-wrap"),
            WhiteSpace::PreLine => dest.write_str("pre-line"),
        }
    }
}
*/

#[derive(ToCss)]
pub enum InitialLetters {
    Normal,
    Values(f32, i32),
}

/*
impl ToCss for InitialLetters {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        match self {
            InitialLetters::Normal => dest.write_str("normal"),
            InitialLetters::Values(v1, v2) => {
                v1.to_css(dest)?;
                dest.write_char(' ')?;
                v2.to_css(dest)?;
                Ok(())
            }
        }
    }
}
*/

pub struct CustomIdent(String);

impl ToCss for CustomIdent {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        dest.write_str(&self.0)
    }
}

#[derive(ToCss)]
pub struct CounterPair {
    name: CustomIdent,
    value: i32,
}

/*
impl ToCss for CounterPair {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        match self {
            CounterPair { name, value } => {
                name.to_css(dest)?;
                dest.write_char(' ')?;
                value.to_css(dest)?;
                Ok(())
            }
        }
    }
}
*/

#[derive(ToCss)]
pub enum ColorOrAuto<C> {
    Color(C),
    Auto,
}

/*
impl<C> ToCss for ColorOrAuto<C>
where
    C: ToCss,
{
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        match self {
            ColorOrAuto::Color(c) => c.to_css(dest),
            ColorOrAuto::Auto => dest.write_str("auto"),
        }
    }
}
*/

#[derive(ToCss)]
pub enum TransformStyle {
    Flat,
    #[css(keyword = "preserve-3d")]
    Preserve3d,
}

#[cfg(test)]
mod tests {
    use super::*;
    use style_traits::ToCss;

    fn to_css<T: ToCss>(value: T) -> String {
        let mut result = String::new();
        value.to_css(&mut result).unwrap();
        result
    }

    #[test]
    fn test_white_space() {
        assert_eq!(to_css(WhiteSpace::Normal), "normal");
        assert_eq!(to_css(WhiteSpace::Nowrap), "nowrap");
        assert_eq!(to_css(WhiteSpace::Pre), "pre");
        assert_eq!(to_css(WhiteSpace::PreWrap), "pre-wrap");
        assert_eq!(to_css(WhiteSpace::PreLine), "pre-line");
    }

    #[test]
    fn test_initial_letters() {
        assert_eq!(to_css(InitialLetters::Normal), "normal");
        assert_eq!(to_css(InitialLetters::Values(3.0, 2)), "3 2");
        assert_eq!(to_css(InitialLetters::Values(2.51, 3)), "2.51 3");
    }

    #[test]
    fn test_counter_pair() {
        assert_eq!(
            to_css(CounterPair {
                name: CustomIdent("a".to_string()),
                value: 1
            }),
            "a 1",
        );
    }

    #[test]
    fn test_color_or_auto() {
        // `i32` is not a color, but for simplification we use it as a fake type.
        assert_eq!(to_css(ColorOrAuto::Color(0i32)), "0");
        assert_eq!(to_css(ColorOrAuto::<i32>::Auto), "auto");
    }

    #[test]
    fn test_transform_style() {
        assert_eq!(to_css(TransformStyle::Flat), "flat");
        assert_eq!(to_css(TransformStyle::Preserve3d), "preserve-3d");
    }
}
