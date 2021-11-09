use adventofcode::*;
use once_cell::sync::Lazy;
use regex::Regex;
use std::str::FromStr;

const INPUT_FILE: &str = "inputs/day4.txt";

static HEX_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^#[0-9a-f]{6}$").unwrap());
static COLOR_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("amb|blu|brn|gry|grn|hzl|oth").unwrap());
static ID_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^[0-9]{9}$").unwrap());
static UNIT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("([0-9]+?)(cm|in)").unwrap());

trait Field {
    fn is_filled(&self) -> bool;
    fn set_value<V: Into<String>>(&mut self, val: V);
    fn verify(&self) -> bool;
}

#[derive(Debug, Default)]
struct YearField<const L: u32, const M: u32> {
    value: Option<String>,
}

impl<const L: u32, const M: u32> Field for YearField<L, M> {
    fn is_filled(&self) -> bool {
        self.value.is_some()
    }

    fn set_value<V: Into<String>>(&mut self, val: V) {
        self.value = Some(val.into())
    }

    fn verify(&self) -> bool {
        if let Some(Ok(val)) = self.value.as_ref().map(|v| v.parse::<u32>()) {
            val >= L && val <= M
        } else {
            false
        }
    }
}

#[derive(Debug, Default)]
struct HexColorField {
    value: Option<String>,
}

impl Field for HexColorField {
    fn is_filled(&self) -> bool {
        self.value.is_some()
    }

    fn set_value<V: Into<String>>(&mut self, val: V) {
        self.value = Some(val.into())
    }

    fn verify(&self) -> bool {
        if let Some(val) = &self.value {
            HEX_REGEX.is_match(val)
        } else {
            false
        }
    }
}

#[derive(Debug, Default)]
struct ColorField {
    value: Option<String>,
}

impl Field for ColorField {
    fn is_filled(&self) -> bool {
        self.value.is_some()
    }

    fn set_value<V: Into<String>>(&mut self, val: V) {
        self.value = Some(val.into())
    }

    fn verify(&self) -> bool {
        if let Some(val) = &self.value {
            COLOR_REGEX.is_match(val)
        } else {
            false
        }
    }
}

#[derive(Debug, Default)]
struct IdField {
    value: Option<String>,
}

impl Field for IdField {
    fn is_filled(&self) -> bool {
        self.value.is_some()
    }

    fn set_value<V: Into<String>>(&mut self, val: V) {
        self.value = Some(val.into())
    }

    fn verify(&self) -> bool {
        if let Some(val) = &self.value {
            ID_REGEX.is_match(val)
        } else {
            false
        }
    }
}

#[derive(Debug, Default)]
struct UnitField {
    value: Option<String>,
}

impl Field for UnitField {
    fn is_filled(&self) -> bool {
        self.value.is_some()
    }

    fn set_value<V: Into<String>>(&mut self, val: V) {
        self.value = Some(val.into())
    }

    fn verify(&self) -> bool {
        if let Some(val) = &self.value {
            if let Some(cap) = UNIT_REGEX.captures(val) {
                if let (Some(val), Some(unit)) = (cap.get(1), cap.get(2)) {
                    if let Ok(val) = val.as_str().parse::<u32>() {
                        return match unit.as_str() {
                            "cm" => (150..=193).contains(&val),
                            "in" => (59..=76).contains(&val),
                            _ => false,
                        };
                    }
                }
            }
        }
        false
    }
}

#[derive(Debug, Default)]
struct IgnoreField;

impl Field for IgnoreField {
    fn is_filled(&self) -> bool {
        true
    }

    fn set_value<V: Into<String>>(&mut self, _val: V) {}

    fn verify(&self) -> bool {
        true
    }
}

#[derive(Debug, Default)]
struct Passport {
    byr: YearField<1920, 2002>,
    iyr: YearField<2010, 2020>,
    eyr: YearField<2020, 2030>,
    hgt: UnitField,
    hcl: HexColorField,
    ecl: ColorField,
    pid: IdField,
    cid: IgnoreField,
}

impl Passport {
    fn validate_fill(&self) -> bool {
        self.byr.is_filled()
            && self.iyr.is_filled()
            && self.eyr.is_filled()
            && self.hgt.is_filled()
            && self.hcl.is_filled()
            && self.ecl.is_filled()
            && self.pid.is_filled()
            && self.cid.is_filled()
    }

    fn validate(&self) -> bool {
        self.byr.verify()
            && self.iyr.verify()
            && self.eyr.verify()
            && self.hgt.verify()
            && self.hcl.verify()
            && self.ecl.verify()
            && self.pid.verify()
            && self.cid.verify()
    }
}

impl FromStr for Passport {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pass = Passport::default();
        let mut chars = s.chars();

        let mut setter = |field: &str| {
            let mut field = field.split(':');
            if let (Some(key), Some(val)) = (field.next(), field.next()) {
                match key {
                    "byr" => pass.byr.set_value(val),
                    "iyr" => pass.iyr.set_value(val),
                    "eyr" => pass.eyr.set_value(val),
                    "hgt" => pass.hgt.set_value(val),
                    "hcl" => pass.hcl.set_value(val),
                    "ecl" => pass.ecl.set_value(val),
                    "pid" => pass.pid.set_value(val),
                    "cid" => pass.cid.set_value(val),
                    _ => {}
                }
            }
        };

        let mut prev = 0;
        while let Some(pos) = chars.position(|c| c == '\n' || c == ' ') {
            setter(&s[prev..prev + pos]);
            prev += pos + 1;
        }
        if prev < s.len() {
            setter(&s[prev..]);
        }

        Ok(pass)
    }
}

fn main() {
    let input: Vec<Passport> = fs::parse_input(INPUT_FILE, "\n\n").unwrap();
    let answer1 = input.iter().filter(|p| p.validate_fill()).count();
    println!("answer1: {:?}", answer1);
    let answer2 = input.iter().filter(|p| p.validate()).count();
    println!("answer2: {:?}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year_field() {
        let mut f = YearField::<1920, 2002>::default();
        assert!(!f.is_filled());
        assert!(!f.verify());
        f.set_value("1908");
        assert!(f.is_filled());
        assert!(!f.verify());
        f.set_value("1921");
        assert!(f.verify());
        f.set_value("2003");
        assert!(!f.verify());
        f.set_value("2002");
        assert!(f.verify());
    }

    #[test]
    fn hex_field() {
        let mut f = HexColorField::default();
        assert!(!f.is_filled());
        assert!(!f.verify());
        f.set_value("unknown");
        assert!(f.is_filled());
        assert!(!f.verify());
        f.set_value("#123");
        assert!(!f.verify());
        f.set_value("#a123");
        assert!(!f.verify());
        f.set_value("#abf123");
        assert!(f.verify());
    }

    #[test]
    fn color_field() {
        let mut f = ColorField::default();
        assert!(!f.is_filled());
        assert!(!f.verify());
        f.set_value("unknown");
        assert!(f.is_filled());
        f.set_value("amb");
        assert!(f.verify());
        f.set_value("blu");
        assert!(f.verify());
        f.set_value("brn");
        assert!(f.verify());
        f.set_value("gry");
        assert!(f.verify());
        f.set_value("grn");
        assert!(f.verify());
        f.set_value("hzl");
        assert!(f.verify());
        f.set_value("oth");
        assert!(f.verify());
        f.set_value("red");
        assert!(!f.verify());
    }

    #[test]
    fn pid_field() {
        let mut f = IdField::default();
        assert!(!f.is_filled());
        assert!(!f.verify());
        f.set_value("unknown");
        assert!(f.is_filled());
        f.set_value("02.2");
        assert!(!f.verify());
        f.set_value("02333");
        assert!(!f.verify());
        f.set_value("02333434");
        assert!(!f.verify());
        f.set_value("023334349");
        assert!(f.verify());
        f.set_value("0123456789");
        assert!(f.verify());
    }

    #[test]
    fn ignore_field() {
        let f = IgnoreField::default();
        assert!(f.is_filled());
        assert!(f.verify());
    }

    #[test]
    fn unit_field() {
        let mut f = UnitField::default();
        assert!(!f.is_filled());
        assert!(!f.verify());
        f.set_value("unknown");
        assert!(f.is_filled());
        f.set_value("190in");
        assert!(!f.verify());
        f.set_value("60");
        assert!(!f.verify());
        f.set_value("59in");
        assert!(!f.verify());
        f.set_value("77in");
        assert!(!f.verify());
        f.set_value("150cm");
        assert!(!f.verify());
        f.set_value("194cm");
        assert!(!f.verify());
        f.set_value("190mm");
        assert!(!f.verify());
        f.set_value("190cm");
        assert!(f.verify());
        f.set_value("60in");
        assert!(f.verify());
    }
}
