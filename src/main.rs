use crate::{
    custom_impl_error::CustomUserValidationError,
    thiserror_impl_error::ThisErrorUserValidationError,
};

mod custom_impl_error;
mod thiserror_impl_error;

fn main() {
    println!(
        "custom implement error \n {:?} \n {:?} \n {:?}",
        CustomUserValidationError::InvalidEmail,
        CustomUserValidationError::InvalidName,
        CustomUserValidationError::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Some error"
        )),
    );

    println!(
        "thiserror implement error \n {:?} \n {:?}",
        ThisErrorUserValidationError::InvalidEmail,
        ThisErrorUserValidationError::InvalidName,
    );
}
