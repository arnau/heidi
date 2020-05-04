// Copyright 2020 Arnau Siches

// Licensed under the MIT license <LICENCE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except
// according to those terms.

//! A generic identifier of 9 digits plus a check digit.

use crate::error::ValidationError;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

/// A digit can be from 0 to 9.
pub type Digit = u16;

#[derive(PartialEq, Clone, Debug)]
pub struct Number {
    digits: [Digit; 9],
    checkdigit: Digit,
}

impl Number {
    /// Creates a new Number from the main 9 digits.
    ///
    /// Prefer `FromStr` or `TryFrom<[Digit; 10]>` if you have a full NHS number.
    ///
    /// # Examples
    ///
    /// ```
    /// use heidi::number::Number;
    ///
    /// let n: [u16; 9] = [0, 1, 0, 1, 9, 9, 0, 0, 1];
    /// let number = Number::new(n);
    ///
    /// assert_eq!(*number.unwrap().checkdigit(), 4);
    /// ```
    pub fn new(digits: [Digit; 9]) -> Result<Self, ValidationError> {
        Ok(Number {
            checkdigit: check_digit(&digits)?,
            digits,
        })
    }

    pub fn checkdigit(&self) -> &Digit {
        &self.checkdigit
    }

    pub fn digits(&self) -> &[Digit; 9] {
        &self.digits
    }
}

impl fmt::Display for Number {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", &self.digits[0])?;
        write!(formatter, "{}", &self.digits[1])?;
        write!(formatter, "{}", &self.digits[2])?;
        write!(formatter, "{}", &self.digits[3])?;
        write!(formatter, "{}", &self.digits[4])?;
        write!(formatter, "{}", &self.digits[5])?;
        write!(formatter, "{}", &self.digits[6])?;
        write!(formatter, "{}", &self.digits[7])?;
        write!(formatter, "{}", &self.digits[8])?;
        write!(formatter, "{}", &self.checkdigit)?;

        Ok(())
    }
}

impl TryFrom<&[Digit; 10]> for Number {
    type Error = ValidationError;

    /// Converts an array slice of 10 decimal `u16` into a [`Number`].
    ///
    /// # Examples
    ///
    /// ```
    /// use heidi::number::Number;
    /// use std::convert::TryFrom;
    ///
    /// let n: [u16; 10] = [3, 1, 0, 1, 0, 0, 3, 2, 3, 7];
    /// let number = Number::try_from(&n);
    ///
    /// assert_eq!(*number.unwrap().checkdigit(), 7);
    /// ```
    ///
    /// # Errors
    ///
    /// Fails with [ValidationError] when the check digit cannot be verified.
    fn try_from(value: &[Digit; 10]) -> Result<Self, Self::Error> {
        let control = value.last().expect("The given slice is empty!");
        let mut digits: [Digit; 9] = [0; 9];

        digits.copy_from_slice(&value[..9]);

        let number = Number::new(digits)?;

        if number.checkdigit() != control {
            return Err(ValidationError::new(&format!(
                "The given check digit {} does not match the actual check digit {}",
                control,
                number.checkdigit()
            )));
        }

        Ok(number)
    }
}

impl TryFrom<String> for Number {
    type Error = ValidationError;

    /// Converts a string of 10 digits into a [`Number`].
    ///
    /// ```
    /// use heidi::number::Number;
    /// use std::convert::TryFrom;
    ///
    /// let n = String::from("2511473232");
    /// let number = Number::try_from(n);
    ///
    /// assert_eq!(*number.unwrap().checkdigit(), 2);
    /// ```
    ///
    /// # Errors
    ///
    /// Fails with [ValidationError] when the check digit cannot be verified.
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Number::from_str(&value)
    }
}

impl TryFrom<usize> for Number {
    type Error = ValidationError;

    /// Converts an unsigned integer into a [`Number`].
    ///
    /// # Examples
    ///
    /// ```
    /// use heidi::number::Number;
    /// use std::convert::TryFrom;
    ///
    /// let n: usize = 1412773237;
    /// let number = Number::try_from(n);
    ///
    /// assert_eq!(*number.unwrap().checkdigit(), 7);
    /// ```
    ///
    /// # Errors
    ///
    /// Fails with [ValidationError] when the check digit cannot be verified.
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let mut digits: [Digit; 10] = [0; 10];
        let mut idx: usize = 0;
        let mut div = 1_000_000_000;

        if (value / div * 10) % 10 != 0 {
            return Err(ValidationError::new(&format!(
                "The given number {} has more than 10 digits.",
                &value
            )));
        }

        while idx <= 9 {
            digits[idx] = ((value / div) % 10) as u16;

            div = div / 10;
            idx = idx + 1;
        }

        Number::try_from(&digits)
    }
}

impl FromStr for Number {
    type Err = ValidationError;

    /// Converts a string slice of 10 digits into a [`Number`].
    ///
    /// ```
    /// use heidi::number::Number;
    /// use std::str::FromStr;
    ///
    /// let n = "3011203237";
    /// let number = Number::from_str(n);
    ///
    /// assert_eq!(*number.unwrap().checkdigit(), 7);
    /// ```
    ///
    /// # Errors
    ///
    /// Fails with [ValidationError] when the check digit cannot be verified.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits: [Digit; 10] = [0; 10];
        let vec: Vec<Digit> = s
            .chars()
            .filter_map(|d| {
                if d.is_whitespace() {
                    None
                } else {
                    Some(d.to_digit(10).unwrap() as u16)
                }
            })
            .collect();

        if vec.len() != 10 {
            return Err(ValidationError::new(
                "NHS Numbers must be of ten-digit long",
            ));
        }

        digits.copy_from_slice(&vec);

        Number::try_from(&digits)
    }
}

fn check_digit(digits: &[u16; 9]) -> Result<Digit, ValidationError> {
    let weighted_sum = digits
        .iter()
        .enumerate()
        .fold(0, |sum, (idx, digit)| sum + (digit * (10 - (idx as u16))));
    let chi = 11 - (weighted_sum % 11);

    match chi {
        11 => Ok(0),
        d if d >= 10 => {
            return Err(ValidationError::new(
                "Modulus 11 numbers cannot have a check digit of 10",
            ));
        }
        d => Ok(d),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_checkdigit() -> Result<(), ValidationError> {
        assert_eq!(3, check_digit(&[8, 9, 3, 1, 7, 7, 4, 5, 8])?);
        assert_eq!(3, check_digit(&[9, 7, 0, 9, 6, 3, 8, 5, 1])?);

        Ok(())
    }

    #[test]
    fn valid_number() {
        assert!(Number::new([8, 9, 3, 1, 7, 7, 4, 5, 8]).is_ok());
    }

    #[test]
    fn valid_number_from_slice10() {
        assert!(Number::try_from(&[8, 9, 3, 1, 7, 7, 4, 5, 8, 3]).is_ok());
    }

    #[test]
    fn display_compact() -> Result<(), ValidationError> {
        let n = "893 177 4583";
        let number = Number::from_str(n)?;
        let expected = "8931774583";

        assert_eq!(format!("{}", number), expected.to_string());

        Ok(())
    }

    #[test]
    fn valid_usize() -> Result<(), ValidationError> {
        let n = 893_177_4583;
        let number = Number::try_from(n)?;

        assert_eq!(*number.checkdigit(), 3);

        Ok(())
    }
}
