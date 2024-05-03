use core::Error;

#[derive(Error)]
pub enum ThisErrorUserValidationError {
    #[error("Invalid username: {0}")]
    InvalidUsername,
    #[error("Invalid password: {0}")]
    InvalidPassword,
}
