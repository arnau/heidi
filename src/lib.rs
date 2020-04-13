// Copyright 2020 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except
// according to those terms.

//! `nhs_number` implements the NSH number validation “Modulus 11”.
//! See: <https://www.datadictionary.nhs.uk/data_dictionary/attributes/n/nhs/nhs_number_de.asp>
//!
//! Example numbers were generated with <http://danielbayley.uk/nhs-number/>

use std::convert::TryFrom;
use std::error::Error;
use std::str::FromStr;

/// A digit can be from 0 to 9.
pub type Digit = u16;

/// Represents an NHS Number as a list of 9 digits (`Number.digits()`) plus 1
/// check digit (`Number.checkdigit()`).
///
/// # Examples
///
/// ```
/// use nhs_number::Number;
/// use std::str::FromStr;
///
/// let n = "6541003238";
/// let number = Number::from_str(n);
///
/// assert_eq!(*number.unwrap().checkdigit(), 8);
/// ```
///
/// Or from a `String`:
///
/// ```
/// use std::convert::TryFrom;
/// use nhs_number::Number;
///
/// let n = String::from("6541003238");
/// let number = Number::try_from(n);
///
/// assert_eq!(*number.unwrap().checkdigit(), 8);
/// ```
///
/// Finally, with a `u16` slice:
///
/// ```
/// use std::convert::TryFrom;
/// use nhs_number::Number;
///
/// let n: [u16; 10] = [6, 5, 4, 1, 0, 0, 3, 2, 3, 8];
/// let number = Number::try_from(&n);
///
/// assert_eq!(*number.unwrap().checkdigit(), 8);
/// ```
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
    /// use nhs_number::Number;
    ///
    /// let n: [u16; 9] = [3, 7, 8, 3, 9, 5, 5, 6, 0];
    /// let number = Number::new(n);
    ///
    /// assert_eq!(*number.unwrap().checkdigit(), 2);
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

impl TryFrom<&[Digit; 10]> for Number {
    type Error = ValidationError;

    /// Converts an array slice of 10 decimal `u16` into a [`Number`].
    ///
    /// # Examples
    ///
    /// ```
    /// use nhs_number::Number;
    /// use std::convert::TryFrom;
    ///
    /// let n: [u16; 10] = [6, 5, 4, 1, 0, 0, 3, 2, 3, 8];
    /// let number = Number::try_from(&n);
    ///
    /// assert_eq!(*number.unwrap().checkdigit(), 8);
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
            return Err(ValidationError(format!(
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
    /// use nhs_number::Number;
    /// use std::convert::TryFrom;
    ///
    /// let n = String::from("6541003238");
    /// let number = Number::try_from(n);
    ///
    /// assert_eq!(*number.unwrap().checkdigit(), 8);
    /// ```
    ///
    /// # Errors
    ///
    /// Fails with [ValidationError] when the check digit cannot be verified.
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Number::from_str(&value)
    }
}

impl FromStr for Number {
    type Err = ValidationError;

    /// Converts a string slice of 10 digits into a [`Number`].
    ///
    /// ```
    /// use nhs_number::Number;
    /// use std::str::FromStr;
    ///
    /// let n = "6541003238";
    /// let number = Number::from_str(n);
    ///
    /// assert_eq!(*number.unwrap().checkdigit(), 8);
    /// ```
    ///
    /// # Errors
    ///
    /// Fails with [ValidationError] when the check digit cannot be verified.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits: [Digit; 10] = [0; 10];
        let vec: Vec<Digit> = s.chars().map(|d| d.to_digit(10).unwrap() as u16).collect();

        if s.len() != 10 {
            return Err(ValidationError::new("Numbers must be of 10 digits."));
        }

        digits.copy_from_slice(&vec);

        Number::try_from(&digits)
    }
}

fn check_digit(digits: &[u16; 9]) -> Result<Digit, ValidationError> {
    let mut res = 0;

    for (idx, digit) in digits.iter().enumerate() {
        res += (10 - (idx as u16)) * digit;
    }

    match 11 - (res % 11) {
        11 => Ok(0),
        d if d >= 10 => {
            return Err(ValidationError::new(
                "This number is invalid. NHS numbers don't have a check digit of 10.",
            ));
        }
        d => Ok(d),
    }
}

/// Represents an error after validating the integrity of a number.
#[derive(PartialEq, Debug, Clone)]
pub struct ValidationError(String);

impl ValidationError {
    fn new(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

impl Error for ValidationError {}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
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

    // #[test]
    // fn invalid_number() -> Result<(), ValidationError> {
    //     // assert_eq!(Number::try_from(&[8, 9, 3, 1, 7, 7, 4, 5, 8])?);
    //     assert_eq!(3, check_digit(&[9, 9, 9, 9, 9, 9, 9, 9, 9])?);

    //     Ok(())
    // }

    #[test]
    fn valid_number_from_slice10() {
        assert!(Number::try_from(&[8, 9, 3, 1, 7, 7, 4, 5, 8, 3]).is_ok());
    }
}
