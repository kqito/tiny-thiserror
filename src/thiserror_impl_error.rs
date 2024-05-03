use core::Error;

#[derive(Error, Debug)]
pub enum ThisErrorUserValidationError {
    #[error("Invalid email")]
    InvalidEmail,
    #[error("Invalid name")]
    InvalidName,
}
