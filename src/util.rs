use std::{error::Error, fmt};

#[derive(Debug)]
pub struct SimpleError(String);

impl SimpleError {
    pub fn from<T: Into<String>>(error_message: T) -> SimpleError {
        SimpleError(error_message.into())
    }
}

impl Error for SimpleError {}

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
