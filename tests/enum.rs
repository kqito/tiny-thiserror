use core::Error;

fn assert(test: impl std::fmt::Display, expect: &str) {
    assert_eq!(format!("{}", test), expect);
}

#[derive(Error, Debug)]
pub enum ErrorAttr {
    #[error("Unknown error")]
    Unknown,
    #[error("Invalid birth {0}/{1}/{2}")]
    InvalidBirth(u16, u8, u8),
    #[error("Invalid name {first} {last}")]
    InvalidName {
        first: String,
        _middle: String,
        last: String,
    },
    // Unused fields
    #[error("Invalid email {0}")]
    InvalidEmail(String),
}

#[test]
fn test_error_attr() {
    assert(ErrorAttr::Unknown, "Unknown error");
    assert(
        ErrorAttr::InvalidEmail("test".to_string()),
        "Invalid email test",
    );
    assert(
        ErrorAttr::InvalidName {
            first: "John".to_string(),
            _middle: "Smith".to_string(),
            last: "Doe".to_string(),
        },
        "Invalid name John Doe",
    );
    assert(
        ErrorAttr::InvalidBirth(2000, 7, 4),
        "Invalid birth 2000/7/4",
    );
}

#[derive(Error, Debug)]
pub enum WithoutErrorAttr {
    InvalidEmail(String),
    InvalidName,
}

#[test]
fn test_without_error_attr() {
    assert(
        WithoutErrorAttr::InvalidEmail("test".to_string()),
        "InvalidEmail(test)",
    );
    assert(WithoutErrorAttr::InvalidName, "InvalidName");
}
