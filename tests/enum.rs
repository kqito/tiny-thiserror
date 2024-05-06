use core::Error;

#[derive(Error, Debug)]
pub enum ErrorAttr {
    #[error("Unknown error")]
    Unknown,
    #[error("Invalid email {0}")]
    InvalidEmail(String),
    #[error("Invalid name {0} {1}")]
    InvalidName(String, String),
}

#[test]
fn test_error_attr() {
    let test = format!("{}", ErrorAttr::Unknown);
    assert_eq!(test, "Unknown error");

    let test = format!("{}", ErrorAttr::InvalidEmail("test".to_string()));
    assert_eq!(test, "Invalid email test");

    let test = format!(
        "{}",
        ErrorAttr::InvalidName("a".to_string(), "b".to_string())
    );
    assert_eq!(test, "Invalid name a b");
}

#[derive(Error, Debug)]
pub enum WithoutErrorAttr {
    InvalidEmail(String),
    InvalidName,
}

#[test]
fn test_without_error_attr() {
    let test = format!("{}", WithoutErrorAttr::InvalidEmail("test".to_string()));
    assert_eq!(test, "InvalidEmail(test)");

    let test = format!("{}", WithoutErrorAttr::InvalidName);
    assert_eq!(test, "InvalidName");
}