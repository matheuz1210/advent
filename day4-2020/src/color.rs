use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Color {
    Hex([u8; 3]),
    Green,
    Blue,
    Amber,
    Gray,
    Brown,
    Hazel,
    Other,
}
impl FromStr for Color {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('#') {
            return Ok(Self::Hex([
                u8::from_str_radix(&s[1..=2], 16)?,
                u8::from_str_radix(&s[3..=4], 16)?,
                u8::from_str_radix(&s[5..=6], 16)?,
            ]));
        }
        match s {
            "amb" => Ok(Self::Amber),
            "blu" => Ok(Self::Blue),
            "brn" => Ok(Self::Brown),
            "gry" => Ok(Self::Gray),
            "grn" => Ok(Self::Green),
            "hzl" => Ok(Self::Hazel),
            "oth" => Ok(Self::Other),
            _ => anyhow::bail!(""),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_from_str() {
        assert_eq!(
            "#ff00ff".parse::<Color>().unwrap(),
            Color::Hex([255, 0, 255])
        );
    }
}
