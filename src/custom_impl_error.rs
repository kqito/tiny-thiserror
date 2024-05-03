use std::{fmt, io};

#[derive(Debug)]
pub enum UserValidationError {
    InvalidEmail,
    InvalidName,
    Unknown(io::Error),
}


impl fmt::Display for UserValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserValidationError::InvalidEmail => write!(f, "Invalid email"),
            UserValidationError::InvalidName => write!(f, "Invalid name"),
            UserValidationError::Unknown(error) => write!(f, "Unknown error {:?}", error),
        }
    }
}

impl From<io::Error> for UserValidationError {
    fn from(error: io::Error) -> Self {
        UserValidationError::Unknown(error)
    }
}
