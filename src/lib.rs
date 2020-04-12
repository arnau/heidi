// Copyright 2020 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except
// according to those terms.

/// `nhs_number` implements the NSH number validation “Modulus 11”.
/// See: https://www.datadictionary.nhs.uk/data_dictionary/attributes/n/nhs/nhs_number_de.asp
use std::error::Error;

// pub struct Number {
//     check: u16,
//     digits: [u16; 9],
// }

// impl Number {
//     fn new(digits: [u16; 9]) -> Self {
//         Number {
//             check: check_digit(&digits),
//             digits,
//         }
//     }
// }

fn check_digit(digits: &[u16; 9]) -> Result<u16, ValidationError> {
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
    fn valid_number() -> Result<(), ValidationError> {
        assert_eq!(3, check_digit(&[8, 9, 3, 1, 7, 7, 4, 5, 8])?);
        assert_eq!(3, check_digit(&[9, 7, 0, 9, 6, 3, 8, 5, 1])?);

        Ok(())
    }
}
