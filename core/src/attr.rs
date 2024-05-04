use syn::{Attribute, LitStr};

/// Extract the formatter from the error attribute
/// Supporting for `#[error("formatter")]` now.
pub fn extract_error_formatter(attrs: &[Attribute]) -> Option<String> {
    attrs.iter().find_map(|attr| {
        if attr.path().is_ident("error") {
            attr.parse_args::<LitStr>().ok().map(|f| f.value())
        } else {
            None
        }
    })
}
