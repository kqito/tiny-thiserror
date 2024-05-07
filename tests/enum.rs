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
    #[error("Invalid email")]
    InvalidEmail(String),
}

#[test]
fn test_error_attr() {
    assert(ErrorAttr::Unknown, "Unknown error");
    assert(
        ErrorAttr::InvalidBirth(2000, 7, 4),
        "Invalid birth 2000/7/4",
    );
    assert(
        ErrorAttr::InvalidName {
            first: "First".to_string(),
            _middle: "Middle".to_string(),
            last: "Last".to_string(),
        },
        "Invalid name First Last",
    );
    assert(ErrorAttr::InvalidEmail("test".to_string()), "Invalid email");
}

#[derive(Error, Debug)]
pub enum WithoutErrorAttr {
    InvalidName,
    InvalidEmail(String),
}

#[test]
fn test_without_error_attr() {
    assert(WithoutErrorAttr::InvalidName, "InvalidName");
    assert(
        WithoutErrorAttr::InvalidEmail("test".to_string()),
        "InvalidEmail(test)",
    );
}
