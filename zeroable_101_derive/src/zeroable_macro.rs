use crate::{
    datastructure::{DataStructure, DataVariant, MyField},
    repr_attr::ReprAttr,
    utils::ExprExt,
};

use proc_macro2::TokenStream as TokenStream2;

use quote::quote;

use syn::{punctuated::Punctuated, DeriveInput};

use std::iter;

mod attribute_parsing;

use self::attribute_parsing::{IsBounded, ZeroConfig};

pub fn derive(ref data: DeriveInput) -> Result<TokenStream2, syn::Error> {
    let ds = &DataStructure::new(data);

    let config = &attribute_parsing::parse_attrs_for_zeroed(ds)?;

    let field_asserts = match ds.data_variant {
        DataVariant::Struct => emit_field_assertions(&ds.variants[0].fields),
        DataVariant::Enum => checks_and_emit_enum_field_assertions(ds, config)?,
        DataVariant::Union => checks_and_emit_union_field_assertions(ds, config)?,
    };

    let name = ds.name;

    let mut unbounded_tp = config.unbounded_typarams.iter().cloned();
    let ty_params = ds
        .generics
        .type_params()
        .map(|x| &x.ident)
        .filter(move |_| unbounded_tp.next() != Some(IsBounded::No));

    let extra_predicates = &*config.extra_predicates;

    let (impl_generics, ty_generics, where_clause) = ds.generics.split_for_impl();

    let empty_preds = Punctuated::new();

    let where_preds = where_clause
        .as_ref()
        .map_or(&empty_preds, |x| &x.predicates)
        .into_iter();

    let where_clause_tokens = quote!(
        where
            #( #where_preds ,)*
            #( #ty_params: ::zeroable_101::Zeroable, )*
            #( #extra_predicates ,)*
    );

    let tokens = quote!(
        impl #impl_generics #name #ty_generics
        #where_clause_tokens
        {
            const _ASSERT_IS_ZEROABLE_101:()=
                #field_asserts;
        }

        unsafe impl #impl_generics ::zeroable_101::Zeroable for #name #ty_generics
        #where_clause_tokens
        {}
    );

    if config.debug_print {
        panic!("\n\n\n{}\n\n\n", tokens);
    }

    Ok(tokens)
}

fn checks_and_emit_enum_field_assertions(
    ds: &'_ DataStructure<'_>,
    config: &'_ ZeroConfig<'_>,
) -> Result<TokenStream2, syn::Error> {
    assert_eq!(ds.data_variant, DataVariant::Enum);

    const REPR_ERR: &str = "Expected a `#[repr(C/<integer_type>/Transparent)]`enum";

    const ENUM_DISCR_ERR: &str = "
Expected either:

- The first variant to have an implicit discriminant,

- Any variant with an explicit `0` discriminant,
    ";

    if ds.variants.len() == 0 {
        return_spanned_err! { ds.name,"Zero variant enums cannot implement Zeroable." }
    }

    let zero_variant = match config.repr_attr {
        ReprAttr::C { .. } | ReprAttr::IntegerRepr { .. } => {
            if ds.variants[0].discriminant.map_or(true, ExprExt::is_zero) {
                0
            } else {
                ds.variants[1..]
                    .iter()
                    .position(|var| var.discriminant.map_or(false, ExprExt::is_zero))
                    .ok_or_else(move || spanned_err! { ds.name,"{}",ENUM_DISCR_ERR })?
            }
        }
        ReprAttr::Transparent => 0,
        ReprAttr::Rust => {
            return_spanned_err! { ds.name,"{}",REPR_ERR }
        }
    };

    Ok(emit_field_assertions(&ds.variants[zero_variant].fields))
}

fn checks_and_emit_union_field_assertions(
    ds: &'_ DataStructure<'_>,
    config: &'_ ZeroConfig<'_>,
) -> Result<TokenStream2, syn::Error> {
    assert_eq!(ds.data_variant, DataVariant::Union);

    const UNION_ATTR_ERR: &str = "\
Expected either:

- A field with a `#[zero(zeroable)]` attribute.

- A `#[repr(transparent)]` attribute.
    ";

    let union_ = &ds.variants[0];

    if union_.fields.len() == 0 {
        Ok(quote!())
    } else {
        dbg!(config.repr_attr);
        let zeroable_field = if config.repr_attr == ReprAttr::Transparent {
            0
        } else {
            config
                .zeroable_field
                .ok_or_else(|| spanned_err!(ds.name, "{}", UNION_ATTR_ERR))?
        };

        let the_field = &union_.fields[zeroable_field];

        Ok(emit_field_assertions(iter::once(the_field)))
    }
}

fn emit_field_assertions<'a, I>(fields: I) -> TokenStream2
where
    I: IntoIterator<Item = &'a MyField<'a>>,
{
    let tys = fields.into_iter().map(|x| x.ty);
    quote!({
        #(
            let _=::zeroable_101::AssertZeroable::<#tys>::NEW;
        )*
    })
}
