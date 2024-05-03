use std::{fmt, io};

#[derive(Debug)]
pub enum CustomUserValidationError {
    InvalidEmail,
    InvalidName,
    Unknown(io::Error),
}

impl fmt::Display for CustomUserValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomUserValidationError::InvalidEmail => write!(f, "Invalid email"),
            CustomUserValidationError::InvalidName => write!(f, "Invalid name"),
            CustomUserValidationError::Unknown(error) => write!(f, "Unknown error {:?}", error),
        }
    }
}

impl From<io::Error> for CustomUserValidationError {
    fn from(error: io::Error) -> Self {
        CustomUserValidationError::Unknown(error)
    }
}
