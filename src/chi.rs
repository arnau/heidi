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
use crate::number;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

/// A digit can be from 0 to 9.
pub type Digit = u16;

#[derive(PartialEq, Clone, Debug)]
pub struct Number(number::Number);

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

        Ok(Number(number::Number::new(digits)?))
    }

    pub fn checkdigit(&self) -> &Digit {
        &self.0.checkdigit()
    }

    pub fn digits(&self) -> &[Digit; 9] {
        &self.0.digits()
    }
}

impl fmt::Display for Number {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl TryFrom<&[Digit; 10]> for Number {
    type Error = ValidationError;

    fn try_from(value: &[Digit; 10]) -> Result<Self, Self::Error> {
        Ok(Number(number::Number::try_from(value)?))
    }
}

impl TryFrom<String> for Number {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let number = number::Number::try_from(value)?;

        validate(number.digits())?;

        Ok(Number(number))
    }
}

impl TryFrom<usize> for Number {
    type Error = ValidationError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let number = number::Number::try_from(value)?;

        validate(number.digits())?;

        Ok(Number(number))
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
        let number = number::Number::from_str(s)?;

        validate(number.digits())?;

        Ok(Number(number))
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
