use std::str::FromStr;

use darling::FromAttributes;
use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
pub(crate) enum IntRepr {
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
}

impl IntRepr {
    const ALL_FMT: &[&str] = &[
        "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize",
    ];
}

impl FromStr for IntRepr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "u8" => Ok(Self::U8),
            "u16" => Ok(Self::U16),
            "u32" => Ok(Self::U32),
            "u64" => Ok(Self::U64),
            "u128" => Ok(Self::U128),
            "usize" => Ok(Self::Usize),
            "i8" => Ok(Self::I8),
            "i16" => Ok(Self::I16),
            "i32" => Ok(Self::I32),
            "i64" => Ok(Self::I64),
            "i128" => Ok(Self::I128),
            "isize" => Ok(Self::Isize),
            _ => Err(()),
        }
    }
}

impl core::fmt::Display for IntRepr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match self {
            IntRepr::U8 => "u8",
            IntRepr::U16 => "u16",
            IntRepr::U32 => "u32",
            IntRepr::U64 => "u64",
            IntRepr::U128 => "u128",
            IntRepr::Usize => "usize",
            IntRepr::I8 => "i8",
            IntRepr::I16 => "i16",
            IntRepr::I32 => "i32",
            IntRepr::I64 => "i64",
            IntRepr::I128 => "i128",
            IntRepr::Isize => "isize",
        };

        write!(f, "{display}")
    }
}

impl FromAttributes for IntRepr {
    fn from_attributes(attrs: &[syn::Attribute]) -> darling::Result<Self> {
        let mut int_repr = None;
        let repr_attr = attrs
            .iter()
            .find(|a| a.path().is_ident("repr"))
            .ok_or_else(|| darling::Error::custom("Unable to find `repr` attribute"))?;

        repr_attr.parse_nested_meta(|m| {
            let repr_type = m
                .path
                .get_ident()
                .ok_or_else(|| syn::Error::new_spanned(&m.path, "Missing `repr` type"))?
                .to_string();
            let ir = IntRepr::from_str(&repr_type).map_err(|()| {
                darling::Error::custom(&format!(
                    "Unsupported `repr` type. Supported types are {}",
                    IntRepr::ALL_FMT.iter().map(|s| format!("`{s}`")).join(" ")
                ))
                .with_span(&m.path)
                .add_sibling_alts_for_unknown_field(IntRepr::ALL_FMT)
            })?;
            int_repr = Some(ir);

            Ok(())
        })?;

        int_repr.ok_or_else(|| darling::Error::custom("unable to find a valid `repr` attribute"))
    }
}
