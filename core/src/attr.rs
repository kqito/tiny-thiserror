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
            let format = match maybe_format {
                Some(f) => f,
                None => return None,
            };

            let dynamic_fields: Vec<DyamicField> = format
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

#[cfg(test)]
mod test {
    #[test]
    fn test_extract_error_formatter() {
        let attrs = vec![
            syn::parse_quote! { #[error("Invalid email {0}")] },
            syn::parse_quote! { #[error("Invalid name {0} {1}")] },
            syn::parse_quote! { #[error("Invalid birth")] },
        ];

        let formatter = formatter.unwrap();
        assert_eq!(formatter.format, "Invalid email {0}");
        assert_eq!(dynamic_fields.len(), 1);
        match &dynamic_fields[0] {
            super::DyamicField::Unnamed(field) => {
                assert_eq!(field.ident, 0);
            }
            _ => panic!("Invalid dynamic field"),
        }
    }
}
