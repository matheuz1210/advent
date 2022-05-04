use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Height {
    Centimeter(i64),
    Inch(i64),
}

impl FromStr for Height {
    type Err = anyhow::Error; //  can't call ParseIntError directly and i don't feel like making my own errortype  so ¯\_(ツ)_/¯

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let suffix_len = match s.len().checked_sub(2) {
            Some(s) => s,
            None => anyhow::bail!(""),
        };
        match &s[suffix_len..] {
            "cm" => Ok(Self::Centimeter(s[..suffix_len].parse()?)),
            "in" => Ok(Self::Inch(s[..suffix_len].parse()?)),
            _ => anyhow::bail!(""),
        }
    }
}
impl TryFrom<&str> for Height {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
macro_rules! height_impl {
    ( $($x:ident)* ) => {$(

    impl From<$x> for Height {
        fn from(value: $x) -> Self {Height::Centimeter(i64::from(value))}

    })*};
}
height_impl!( u8 u16 u32 i8 i16 i32 i64 );

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn height_from_str() {
        assert_eq!("158cm".parse::<Height>().unwrap(), Height::Centimeter(158));
        assert_eq!("158in".parse::<Height>().unwrap(), Height::Inch(158));
        assert_ne!("158".parse::<Height>().unwrap(), Height::Centimeter(158));
        assert_eq!(
            "15800cm".parse::<Height>().unwrap(),
            Height::Centimeter(15800)
        );
    }
}
