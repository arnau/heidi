// Copyright 2020 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except
// according to those terms.

/// `nhs_number` implements the NSH number validation “Modulus 11”.
/// See: https://www.datadictionary.nhs.uk/data_dictionary/attributes/n/nhs/nhs_number_de.asp
use std::convert::TryFrom;
use std::error::Error;

/// A digit can be from 0 to 9.
type Digit = u16;

#[derive(PartialEq, Clone, Debug)]
pub struct Number {
    digits: [Digit; 9],
    checkdigit: Digit,
}

impl Number {
    fn new(digits: [Digit; 9]) -> Result<Self, ValidationError> {
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

use std::str::FromStr;
impl FromStr for Number {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 10 {
            return Err(ValidationError::new("Numbers must be of 10 digits."));
        }

        let mut digits: [Digit; 10] = [0; 10];
        let vec: Vec<Digit> = s.chars().map(|d| d.to_digit(10).unwrap() as u16).collect();

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
