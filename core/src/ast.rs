use syn::{DeriveInput, Ident, Variant};

pub struct Enum {
    pub ident: Ident,
    pub variants: Vec<Variant>,
}

pub struct Struct {
    ident: Ident,
    variants: Vec<Variant>,
}

// For usability handling AST
pub enum Input {
    Enum(Enum),
    Struct(Struct),
}

impl TryFrom<DeriveInput> for Input {
    type Error = &'static str;

    fn try_from(input: DeriveInput) -> Result<Self, &'static str> {
        match input.data {
            syn::Data::Enum(data) => Ok(Input::Enum(Enum {
                ident: input.ident.clone(),
                variants: data.variants.into_iter().collect(),
            })),
            syn::Data::Struct(_) => Ok(Input::Struct(Struct {
                ident: input.ident.clone(),
                variants: vec![],
            })),
            _ => Err("Unsupported type"),
        }
    }
}
