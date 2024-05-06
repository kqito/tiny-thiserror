use syn::{Attribute, LitStr};

pub struct NamedDyamicField {
    pub ident: String,
}

pub struct UnamedDyamicField {
    pub ident: usize,
}

pub enum DyamicField {
    Named(NamedDyamicField),
    Unnamed(UnamedDyamicField),
}

pub struct Formatter {
    pub format: String,
    pub dynamic_fields: Vec<DyamicField>,
}

/// Extract the formatter from the error attribute
/// Supporting for `#[error("formatter")]` now.
pub fn extract_error_formatter(attrs: &[Attribute]) -> Option<Formatter> {
    attrs.iter().find_map(|attr| {
        if attr.path().is_ident("error") {
            let maybe_format = attr.parse_args::<LitStr>().ok().map(|f| f.value());
            let original_format = match maybe_format {
                Some(f) => f,
                None => return None,
            };

            // e.g. convert `name: {first} {last}` to `name: {} {}` using Reg
            let reg = regex::Regex::new(r"\{[a-zA-Z0-9_]*\}").unwrap();
            let format = reg.replace_all(&original_format, "{}").to_string();

            let dynamic_fields: Vec<DyamicField> = original_format
                .split("{")
                .skip(1)
                .map(|field| {
                    let ident = field.split("}").next().unwrap();
                    if let Ok(index) = ident.parse::<usize>() {
                        DyamicField::Unnamed(UnamedDyamicField { ident: index })
                    } else {
                        DyamicField::Named(NamedDyamicField {
                            ident: ident.to_string(),
                        })
                    }
                })
                .collect();

            return Some(Formatter {
                format,
                dynamic_fields,
            });
        }
        None
    })
}
