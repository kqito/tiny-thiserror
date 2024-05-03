mod custom_impl_error;

fn main() {
    println!(
        "{:?} {:?} {:?}",
        custom_impl_error::UserValidationError::InvalidEmail,
        custom_impl_error::UserValidationError::InvalidName,
        custom_impl_error::UserValidationError::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Some error"
        )),
    );
}
