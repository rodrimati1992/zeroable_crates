use proc_macro2::TokenStream as TokenStream2;

use quote::quote;

use syn::{punctuated::Punctuated, DeriveInput};

use crate::datastructure::{DataStructure, DataVariant, MyField};

pub fn derive(ref data: DeriveInput) -> Result<TokenStream2, syn::Error> {
    let ref ds = DataStructure::new(data);

    let field_asserts = match ds.data_variant {
        DataVariant::Struct => emit_field_assertions(&ds.variants[0].fields),
        DataVariant::Enum => unimplemented!("Enums cannot derive Zeroable yet"),
        DataVariant::Union => unimplemented!("Unions cannot derive Zeroable yet"),
    };

    let name = ds.name;

    let ty_params = ds.generics.type_params().map(|x| &x.ident);

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
    );

    Ok(quote!(
        impl #impl_generics #name #ty_generics
        #where_clause_tokens
        {
            const _ASSERT_IS_ZEROABLE_101:()=
                #field_asserts;
        }

        unsafe impl #impl_generics ::zeroable_101::Zeroable for #name #ty_generics
        #where_clause_tokens
        {}
    ))
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
