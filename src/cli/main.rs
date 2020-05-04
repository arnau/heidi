// Copyright 2020 Arnau Siches

// Licensed under the MIT license <LICENCE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except
// according to those terms.

use clap::arg_enum;
use heidi::{chi, nhs};
use std::process;
use std::str::FromStr;
use structopt::StructOpt;

arg_enum! {
    #[derive(Debug)]
    enum Format {
        Compact,
        Official,
    }
}

arg_enum! {
    #[derive(Debug)]
    enum Typeid {
        Nhs,
        Chi,
    }
}

/// heidi helps dealing with health identifiers such as NHS numbers
/// or CHI numbers.
///
/// ## nhs type
///
/// An NHS number is the health identifier for England and Wales.
///
/// It is 10 digits long. The first 9 are the main digits and the last one
/// is the check digit that validates the integrity of the previous 9.
///
/// See <https://www.datadictionary.nhs.uk/data_dictionary/attributes/n/nhs/nhs_number_de.asp>
///
///
/// ## chi type
///
/// A CHI number is the health identifier for Scotland.
///
/// It is 10 digits long.
///
/// See <https://www.ndc.scot.nhs.uk/Data-Dictionary/SMR-Datasets/Patient-Identification-and-Demographic-Information/Community-Health-Index-Number/>
#[derive(StructOpt, Debug)]
enum Opt {
    /// Validates a health identifier number for the given type.
    Check {
        /// The type of health identifier.
        #[structopt(possible_values=&["nhs", "chi"])]
        _type: Typeid,

        /// The health identifier number to validate.
        number: String,
    },
    Generate {
        /// The output format. Official display requires a particular spacing, for example an NHS
        /// Number requires a 3-3-4 formatting: 123 456 7890.
        #[structopt(long, short="f", possible_values=&["compact", "official"], default_value="compact", case_insensitive=true)]
        format: Format,

        /// The type of health identifier.
        #[structopt(possible_values=&["nhs", "chi"])]
        _type: Typeid,
    },
}

fn main() {
    match Opt::from_args() {
        Opt::Check { _type, number } => match _type {
            Typeid::Nhs => {
                match nhs::Number::from_str(&number) {
                    Ok(n) => {
                        println!("NHS Number '{:#}' is valid.", &n);
                    }
                    Err(e) => {
                        eprintln!("NHS Number '{}' is invalid.", &number);
                        eprintln!("Error: {}.", &e);
                        process::exit(1);
                    }
                };
            }
            Typeid::Chi => {
                match chi::Number::from_str(&number) {
                    Ok(n) => {
                        println!("Chi Number '{:#}' is valid.", &n);
                    }
                    Err(e) => {
                        eprintln!("Chi Number '{}' is invalid.", &number);
                        eprintln!("Error: {}.", &e);
                        process::exit(1);
                    }
                };
            }
        },
        Opt::Generate { _type, format } => match _type {
            Typeid::Nhs => {
                match nhs::lottery() {
                    Ok(n) => match format {
                        Format::Official => println!("{:#}", &n),
                        _ => println!("{}", &n),
                    },
                    Err(e) => {
                        eprintln!("{}", &e);
                        process::exit(1);
                    }
                };
            }
            Typeid::Chi => {
                match chi::lottery() {
                    Ok(n) => println!("{}", &n),
                    Err(e) => {
                        eprintln!("{}", &e);
                        process::exit(1);
                    }
                };
            }
        },
    };
}
