// Copyright 2020 Arnau Siches

// Licensed under the MIT license <LICENCE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except
// according to those terms.

use std::error::Error;
use std::fmt;

/// Represents an error after validating the integrity of a number.
#[derive(PartialEq, Debug, Clone)]
pub struct ValidationError(String);

impl ValidationError {
    pub fn new(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

impl Error for ValidationError {}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
