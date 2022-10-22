use std::{error::Error, fmt};

#[derive(Debug)]
pub struct PlacementError {
    details: String,
}

impl PlacementError {
    pub fn new(msg: &str) -> PlacementError {
        PlacementError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for PlacementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for PlacementError {
    fn description(&self) -> &str {
        &self.details
    }
}
