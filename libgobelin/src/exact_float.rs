use std::fmt::Display;
use std::iter::Sum;
use std::ops::Add;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExactFloat {
    /**
     * Will store integer part of the floating point number.
     */
    pub numerator: i32,
    /**
     * Will store decimal part of the floating point number.
     * The stored number is consider to be between 0 and 99 (included).
     * So 0.07 will be stored as 7, and 0.57 will be stored as 57.
     */
    pub denominator: u8,
}

impl ExactFloat {
    pub fn new(numerator: i32, denominator: u8) -> Self {
        ExactFloat {
            numerator,
            denominator,
        }
    }

    pub fn is_positive(&self) -> bool {
        self.numerator >= 0
    }

    pub fn is_negative(&self) -> bool {
        !self.is_positive()
    }

    pub fn numerator_digit_count(&self) -> usize {
        format!("{}", self.numerator.abs()).chars().count()
    }

    pub fn abs(&self) -> Self {
        if self.numerator >= 0 {
            self.clone()
        } else {
            ExactFloat {
                numerator: self.numerator.abs(),
                denominator: self.denominator,
            }
        }
    }

    pub fn sign(&self) -> &str {
        if self.is_positive() {
            "+"
        } else {
            "-"
        }
    }
}

impl FromStr for ExactFloat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let s = s.replace(" ", "");
        if let Some((numerator, denominator)) = s.split_once(".") {
            if let Ok(numerator) = numerator.parse::<i32>() {
                if let Ok(denominator) = denominator.parse::<u8>() {
                    Ok(ExactFloat::new(numerator, denominator))
                } else {
                    Err(format!("cannot parse as u8 denominator part of '{}'", s))
                }
            } else {
                Err(format!("cannot parse as i32 numerator part of '{}'", s))
            }
        } else {
            let numerator = s.parse::<i32>();
            if let Ok(numerator) = numerator {
                Ok(ExactFloat::new(numerator, 0))
            } else {
                Err(format!("cannot parse as i32 numerator part of '{}'", s))
            }
        }
    }
}

impl Display for ExactFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let denominator = if self.denominator > 0 {
            format!(".{:02}", self.denominator)
        } else {
            String::default()
        };
        if self.numerator.abs() >= 1_000_000 {
            let milions = self.numerator / 1_000_000;
            let thousands = (self.numerator % 1_000_000) / 1_000;
            let unit = self.numerator % 1_000;
            write!(f, "{} {:03} {:03}{}", milions, thousands, unit, denominator)
        } else if self.numerator.abs() >= 1_000 {
            let thousands = self.numerator / 1_000;
            let unit = self.numerator % 1_000;
            write!(f, "{} {:03}{}", thousands, unit, denominator)
        } else {
            write!(f, "{}{}", self.numerator, denominator)
        }
    }
}

impl Add for ExactFloat {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let self_sign: i64 = if self.sign() == "+" { 1 } else { -1 };
        let self_full: i64 = (self.numerator * 100).into();
        let self_denominator: i64 = self.denominator.into();
        let self_full: i64 = self_full + self_sign * self_denominator;
        let other_sign: i64 = if other.sign() == "+" { 1 } else { -1 };
        let other_full: i64 = (other.numerator * 100).into();
        let other_denominator: i64 = other.denominator.into();
        let other_full: i64 = other_full + other_sign * other_denominator;
        let full: i64 = self_full + other_full;
        let numerator: i32 = (full / 100).try_into().unwrap_or(0);
        let denominator: u8 = (full.abs() % 100).try_into().unwrap_or(0);
        Self {
            numerator,
            denominator,
        }
    }
}

impl Sum for ExactFloat {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(ExactFloat::new(0, 0), |total, current| total + current)
    }
}
