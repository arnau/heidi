// Copyright 2020 Arnau Siches

// Licensed under the MIT license <LICENCE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except
// according to those terms.

//! `heidi` implements the CHI number validation “Modulus 11”. See:
//! <https://www.ndc.scot.nhs.uk/Data-Dictionary/SMR-Datasets//Patient-Identification-and-Demographic-Information/Community-Health-Index-Number/>
//!
//! The CHI Number (Community Health Index) is a unique number allocated to
//! every patient registered with the NHS in Scotland.
//!
//! A CHI Number is always 10 digits long. The first 6 digits are the date of
//! birth as `DDMMYY`. The next 2 digits are random between 0 and 9. The 9th
//! digit is random as well but it is always even for females and odd for males.
//!
//! The last digit of the number is the “check digit” to aid in integrity checks.

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
    /// use heidi::chi::Number;
    ///
    /// let n: [u16; 9] = [0, 1, 0, 1, 9, 9, 0, 0, 1];
    /// let number = Number::new(n);
    ///
    /// assert_eq!(*number.unwrap().checkdigit(), 4);
    /// ```
    pub fn new(digits: [Digit; 9]) -> Result<Self, ValidationError> {
        validate(&digits)?;

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
    /// use heidi::chi::Number;
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
    /// use heidi::chi::Number;
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
    /// use heidi::chi::Number;
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
    /// use heidi::chi::Number;
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

/// Checks the date boundaries.
///
/// TODO: Validaton is naive. Does not check for real month limits nor leap years.
fn validate(digits: &[u16; 9]) -> Result<(), ValidationError> {
    let day = digits[0] * 10 + digits[1];
    let month = digits[2] * 10 + digits[3];

    if day == 0 || day > 31 || month == 0 || month > 12 {
        return Err(ValidationError::new("Invalid CHI number"));
    }

    Ok(())
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
                "CHI numbers don't have a check digit of 10",
            ));
        }
        d => Ok(d),
    }
}
