// Copyright 2020 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except
// according to those terms.

/// `nhs_number` implements the NSH number validation “Modulus 11”.
/// See: https://www.datadictionary.nhs.uk/data_dictionary/attributes/n/nhs/nhs_number_de.asp

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

fn check_digit(digits: &[u16; 9]) -> u16 {
    let mut res = 0;

    for (idx, digit) in digits.iter().enumerate() {
        println!("{:?}, {:?}", 10 - idx, digit);
        res += (10 - (idx as u16)) * digit;
    }

    match 11 - (res % 11) {
        11 => 0,
        d if d >= 10 => {
            panic!("This number is invalid. NHS numbers don't have a check digit of 10.")
        }
        d => d,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_number() {
        assert_eq!(3, check_digit(&[8, 9, 3, 1, 7, 7, 4, 5, 8]));
        assert_eq!(3, check_digit(&[9, 7, 0, 9, 6, 3, 8, 5, 1]));
    }
}
