use crate::{
    custom_impl_error::CustomUserValidationError,
    thiserror_impl_error::ThisErrorUserValidationError,
};

mod custom_impl_error;
mod thiserror_impl_error;

fn main() {
    println!(
        "{:?} {:?} {:?}",
        CustomUserValidationError::InvalidEmail,
        CustomUserValidationError::InvalidName,
        CustomUserValidationError::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Some error"
        )),
    );

    println!(
        "{:?} {:?} {:?}",
        ThisErrorUserValidationError::InvalidEmail,
        ThisErrorUserValidationError::InvalidName,
        ThisErrorUserValidationError::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Some error"
        )),
    );
}
