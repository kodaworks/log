use std::{
    error::Error,
    fmt,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Fp<const DECIMALS: usize>(i128);

impl<const DECIMALS: usize> Fp<DECIMALS> {
    const SCALE: i128 = 10i128.pow(DECIMALS as u32);

    pub fn from_bytes<const N: usize>(buf: &[u8]) -> Result<Self, ParseFpError> {
        let mut i = 0;
        let mut sign = 1i128;

        // Check if there is a negative sign in the first element
        if buf.get(0) == Some(&b'-') {
            sign = -1;
            i += 1;
        }

        // Parse the integer part
        let mut int_val: i128 = 0;
        while buf[i] != b'.' {
            // Check if the character is a digit
            if buf[i] >= b'0' && buf[i] <= b'9' {
                int_val = int_val * 10 + (buf[i] - b'0') as i128;
            } else {
                return Err(ParseFpError {
                    kind: FpErrorKind::InvalidInteger,
                });
            }

            i += 1;
            // Check if we have reached the end of the buffer
            // If there is no decimal point, return an error
            if i >= buf.len() {
                return Err(ParseFpError {
                    kind: FpErrorKind::InvalidFormat,
                });
            }
        }
        i += 1; // skip '.'

        // Parse the fractional part
        let mut frac_val: i128 = 0;
        let mut j = 0;
        while j < N {
            if buf[i] >= b'0' && buf[i] <= b'9' {
                frac_val = frac_val * 10 + (buf[i] - b'0') as i128;
            } else {
                return Err(ParseFpError {
                    kind: FpErrorKind::InvalidFraction,
                });
            }

            i += 1;
            j += 1;
        }

        Ok(Fp(sign * (int_val * Self::SCALE + frac_val)))
    }
}

impl<const DECIMALS: usize> Add for Fp<DECIMALS> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Fp(self.0 + rhs.0)
    }
}

impl<const DECIMALS: usize> Sub for Fp<DECIMALS> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Fp(self.0 - rhs.0)
    }
}

impl<const DECIMALS: usize> Mul for Fp<DECIMALS> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let result = (self.0 * rhs.0) / Self::SCALE;
        Fp(result)
    }
}

impl<const DECIMALS: usize> Div for Fp<DECIMALS> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let result = (self.0 * Self::SCALE) / rhs.0;
        Fp(result)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseFpError {
    pub(super) kind: FpErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FpErrorKind {
    InvalidInteger,
    InvalidFraction,
    InvalidFormat,
    TooManyDecimals,
    Overflow,
}

impl fmt::Display for ParseFpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[allow(deprecated)]
        self.description().fmt(f)
    }
}

impl Error for ParseFpError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        match self.kind {
            FpErrorKind::InvalidInteger => "cannot parse integer from empty string",
            FpErrorKind::InvalidFraction => "invalid digit found in string",
            FpErrorKind::InvalidFormat => "invalid format",
            FpErrorKind::TooManyDecimals => "too many decimals",
            FpErrorKind::Overflow => "number too large to fit in target type",
        }
    }
}

impl<'de, const N: usize> Deserialize<'de> for Fp<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FixedVisitor<const N: usize>;

        impl<'de, const N: usize> Visitor<'de> for FixedVisitor<N> {
            type Value = Fp<N>;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "a decimal string with exactly {} fractional digits", N)
            }

            // Custom deserialization for borrowed string
            fn visit_borrowed_str<E>(self, s: &'de str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Fp::<N>::from_bytes::<N>(s.as_bytes()).map_err(E::custom)
            }
        }

        deserializer.deserialize_any(FixedVisitor::<N>)
    }
}

impl<const DECIMALS: usize> FromStr for Fp<DECIMALS> {
    type Err = ParseFpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Fp::<DECIMALS>::from_bytes::<DECIMALS>(s.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fp_arithmetic() {
        let a = Fp::<3>::from_str("1.234").unwrap();
        let b = Fp::<3>::from_str("2.345").unwrap();

        // Test addition
        assert_eq!(a + b, Fp::<3>::from_str("3.579").unwrap());

        // Test subtraction
        assert_eq!(b - a, Fp::<3>::from_str("1.111").unwrap());

        // Test multiplication
        assert_eq!(a * b, Fp::<3>::from_str("2.893").unwrap());

        // Test division
        assert_eq!(b / a, Fp::<3>::from_str("1.900").unwrap());
    }

    #[test]
    fn test_fp_parsing() {
        // Test valid parsing
        assert_eq!(Fp::<3>::from_str("1.234").unwrap().0, 1234);
        assert_eq!(Fp::<3>::from_str("0.001").unwrap().0, 1);
        assert_eq!(Fp::<3>::from_str("-1.234").unwrap().0, -1234);

        // Test invalid format
        assert!(matches!(
            Fp::<3>::from_str("1234").unwrap_err().kind,
            FpErrorKind::InvalidFormat
        ));

        // Test invalid integer
        assert!(matches!(
            Fp::<3>::from_str("abc.123").unwrap_err().kind,
            FpErrorKind::InvalidInteger
        ));

        // Test invalid fraction
        assert!(matches!(
            Fp::<3>::from_str("1.abc").unwrap_err().kind,
            FpErrorKind::InvalidFraction
        ));
    }

    #[test]
    fn test_fp_edge_cases() {
        // Test zero
        assert_eq!(Fp::<3>::from_str("0.000").unwrap().0, 0);

        // Test large numbers
        assert_eq!(Fp::<3>::from_str("999.999").unwrap().0, 999999);
        assert_eq!(Fp::<3>::from_str("-999.999").unwrap().0, -999999);

        // Test small numbers
        assert_eq!(Fp::<3>::from_str("0.001").unwrap().0, 1);
        assert_eq!(Fp::<3>::from_str("-0.001").unwrap().0, -1);
    }

    #[test]
    fn test_fp_arithmetic_edge_cases() {
        let zero = Fp::<3>::from_str("0.000").unwrap();
        let one = Fp::<3>::from_str("1.000").unwrap();
        let two = Fp::<3>::from_str("2.000").unwrap();

        // Test operations with zero
        assert_eq!(zero + one, one);
        assert_eq!(one + zero, one);
        assert_eq!(zero * one, zero);
        assert_eq!(one * zero, zero);

        // Test division by one
        assert_eq!(one / one, one);
        assert_eq!(two / one, two);

        // Test negative numbers
        let neg_one = Fp::<3>::from_str("-1.000").unwrap();
        assert_eq!(one + neg_one, zero);
        assert_eq!(one * neg_one, neg_one);
    }
}
