# tiny-thiserror Crate
tiny-thiserror is a Rust error handling library created for learning procedural macros (unpublished). It's a self-made crate inspired by the [thiserror crate](https://crates.io/crates/thiserror), enabling easy definition of error types and automatic generation of error messages. However, its features are quite limited.

> [!IMPORTANT]
> This library is created for learning purposes and its features are limited. For serious projects, we recommend using more mature libraries (such as [thiserror crate](https://crates.io/crates/thiserror) and others).

## Main Features
- Automatic derivation of error types with #[derive(Error)] attribute
- Automatic generation of error messages with #[error("message")] attribute

## Usage Example
```rust
use tiny_thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorAttr {
    #[error("Unknown error")]
    Unknown,
    #[error("Invalid birth {0}/{1}/{2}")]
    InvalidBirth(u16, u8, u8),
    #[error("Invalid name {first} {last}")]
    InvalidName {
        first: String,
        middle: String,
        last: String,
    },
}
```

By defining error types as above, you can automatically derive error types and generate error messages.
