mod height;
use height::Height;
use std::str::FromStr;
mod color;
use color::Color;
use itertools::Itertools;

pub fn main() {
    let count = include_str!("input")
        .split("\n\n")
        .filter_map(|i| i.parse().ok())
        .filter(Passport::is_valid)
        .count();

    println!("day4: {count} valid passports!");
}

#[derive(Debug)]
struct Passport {
    byr: u16,    // birth year
    iyr: u16,    // issue year
    eyr: u16,    // expiration year
    hgt: Height, // height
    hcl: Color,  // hair color
    ecl: Color,  // eye color

    // pid and cid are strings because they can contain 0's at the beginning
    #[allow(clippy::similar_names)]
    pid: String, // passport id
    cid: Option<String>, // country id
}
// macro_rules! none {
//     ( $( $x:ident: $t:ident ),* ) => { $(let mut $x: Option<$t> = None;)* };
// }
impl FromStr for Passport {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Passport::parse(s).ok_or(())
    }
}
impl Passport {
    fn parse(s: &str) -> Option<Self> {
        let mut byr: Option<u16> = None;
        let mut iyr: Option<u16> = None;
        let mut eyr: Option<u16> = None;
        let mut hgt: Option<Height> = None;
        let mut hcl: Option<Color> = None;
        let mut ecl: Option<Color> = None;
        let mut pid: Option<String> = None;
        let mut cid: Option<String> = None;

        for pairs in s.split_whitespace() {
            let (key, value) = pairs.split(':').collect_tuple()?;
            match key {
                "byr" => byr = Some(value.parse().ok()?),
                "iyr" => iyr = Some(value.parse().ok()?),
                "eyr" => eyr = Some(value.parse().ok()?),
                "hgt" => hgt = Some(value.parse().ok()?),
                "hcl" => hcl = Some(value.parse().ok()?),
                "ecl" => ecl = Some(value.parse().ok()?),
                "pid" => pid = Some(value.to_string()),
                "cid" => cid = Some(value.to_string()),
                _ => continue,
            };
        }
        Some(Self {
            byr: byr?,
            iyr: iyr?,
            eyr: eyr?,
            hgt: hgt?,
            hcl: hcl?,
            ecl: ecl?,
            pid: pid?,
            cid,
        })
    }
    //byr (Birth Year) - four digits; at least 1920 and at most 2002.
    //iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    //eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    //hgt (Height) - a number followed by either cm or in:
    //If cm, the number must be at least 150 and at most 193.
    //If in, the number must be at least 59 and at most 76.
    //hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    //ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    //pid (Passport ID) - a nine-digit number, including leading zeroes.
    //cid (Country ID) - ignored, missing or not.
    pub fn is_valid(&self) -> bool {
        if self.byr < 1920 || self.byr > 2002 {
            return false;
        }
        if self.iyr < 2010 || self.iyr > 2020 {
            return false;
        }
        if self.eyr < 2020 || self.eyr > 2030 {
            return false;
        }

        match self.hgt {
            Height::Centimeter(cm) => {
                if cm < 150 || cm > 193 {
                    {
                        return false;
                    }
                }
            }
            Height::Inch(inch) => {
                if inch < 59 || inch > 76 {
                    {
                        return false;
                    }
                }
            }
        }
        if let Color::Hex(_) = self.hcl {
            // if hex, continue
        } else {
            return false;
        }

        if let Color::Hex(_) = self.ecl {
            return false;
        }

        match self.pid.parse::<i64>() {
            Ok(_) => {
                // if it parses then is a number
                if self.pid.len() != 9 {
                    return false;
                }
            }
            Err(_) => return false,
        }
        true
    }
}
