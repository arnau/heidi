// Copyright 2020 Arnau Siches

// Licensed under the MIT license <LICENCE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except
// according to those terms.

//! `heidi` implements the NHS number validation “Modulus 11”. See:
//! <https://www.datadictionary.nhs.uk/data_dictionary/attributes/n/nhs/nhs_number_de.asp>
//!
//! Example numbers were generated with <http://danielbayley.uk/nhs-number/>
//!
//! The NHS Number is a unique number allocated to every patient registered with
//! the NHS in England, Wales and the Isle of Man.
//!
//! In short, an NHS Number is always 10 digits long sometimes formatted in a 3-3-4 manner.
//! For example, `6541003238` can be presented as `654 100 3238`.
//!
//! The last digit of the number is the “check digit” to aid in integrity checks.

use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// A digit can be from 0 to 9.
pub type Digit = u16;

/// Represents an NHS Number as a list of 9 digits (`Number.digits()`) plus 1
/// check digit (`Number.checkdigit()`).
///
/// # Examples
///
/// ```
/// use heidi::nhs::Number;
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
/// use heidi::nhs::Number;
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
/// use heidi::nhs::Number;
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
    /// use heidi::nhs::Number;
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

impl fmt::Display for Number {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", &self.digits[0])?;
        write!(formatter, "{}", &self.digits[1])?;
        write!(formatter, "{}", &self.digits[2])?;
        if formatter.alternate() {
            write!(formatter, " ")?;
        }
        write!(formatter, "{}", &self.digits[3])?;
        write!(formatter, "{}", &self.digits[4])?;
        write!(formatter, "{}", &self.digits[5])?;
        if formatter.alternate() {
            write!(formatter, " ")?;
        }
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
    /// use heidi::nhs::Number;
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
    /// use heidi::nhs::Number;
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

impl TryFrom<usize> for Number {
    type Error = ValidationError;

    /// Converts an unsigned integer into a [`Number`].
    ///
    /// # Examples
    ///
    /// ```
    /// use heidi::nhs::Number;
    /// use std::convert::TryFrom;
    ///
    /// let n: usize = 6541003238;
    /// let number = Number::try_from(n);
    ///
    /// assert_eq!(*number.unwrap().checkdigit(), 8);
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
            return Err(ValidationError(format!(
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
    /// use heidi::nhs::Number;
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
                "NHS numbers don't have a check digit of 10",
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

/// Returns a random NHS Number.
///
/// If the result is not valid (e.g. the modulus 11 is 10) it will generate a new one.
///
/// # Examples
///
/// ```
/// use heidi::nhs::lottery;
///
/// let number = lottery();
/// assert!(number.is_ok());
/// ```
pub fn lottery() -> Result<Number, ValidationError> {
    use rand::prelude::*;

    let mut rng = rand::thread_rng();
    let mut digits = [0u16; 9];
    let distr = rand::distributions::Uniform::new_inclusive(0, 9);

    for x in &mut digits {
        *x = rng.sample(distr);
    }

    match Number::new(digits) {
        Err(_) => lottery(),
        number => number,
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
    fn valid_formatted_string() -> Result<(), ValidationError> {
        let f = Number::from_str("893 177 4583")?;
        let u = Number::from_str("8931774583")?;

        assert_eq!(f.checkdigit(), u.checkdigit());
        assert_eq!(f.digits(), u.digits());

        Ok(())
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
    fn display_alternate() -> Result<(), ValidationError> {
        let n = String::from("893 177 4583");
        let number = Number::from_str(&n)?;

        assert_eq!(format!("{:#}", number), n);

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
