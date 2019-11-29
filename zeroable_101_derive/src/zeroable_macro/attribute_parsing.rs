use crate::{
    attribute_parsing_shared::with_nested_meta,
    datastructure::{DataStructure, MyField, Struct},
    repr_attr::ReprAttr,
};

use syn::{Attribute, Lit, Meta, MetaList, MetaNameValue, WherePredicate};

use std::marker::PhantomData;

////////////////////////////////////////////////////////////////////////////////

pub(crate) struct ZeroConfig<'a> {
    pub(crate) extra_predicates: Vec<WherePredicate>,

    /// The type parameters that don't have a `Zeroable` bound.
    pub(crate) unbounded_typarams: Vec<IsBounded>,

    /// If true,panics with the output of the derive macro.
    pub(crate) debug_print: bool,

    pub(crate) zeroable_field: Option<usize>,

    pub(crate) repr_attr: ReprAttr,

    _marker: PhantomData<&'a ()>,
}

impl<'a> ZeroConfig<'a> {
    fn new(za: ZeroableAttrs<'a>) -> Result<Self, syn::Error> {
        let ZeroableAttrs {
            extra_predicates,
            unbounded_typarams,
            debug_print,
            zeroable_field,
            repr_attr,
            _marker,
        } = za;

        Ok(Self {
            extra_predicates,
            unbounded_typarams,
            debug_print,
            zeroable_field,
            repr_attr,
            _marker,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum IsBounded {
    Yes,
    No,
}

////////////////////////////////////////////////////////////////////////////////

struct ZeroableAttrs<'a> {
    extra_predicates: Vec<WherePredicate>,
    unbounded_typarams: Vec<IsBounded>,
    debug_print: bool,
    zeroable_field: Option<usize>,
    repr_attr: ReprAttr,
    _marker: PhantomData<&'a ()>,
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
enum ParseContext<'a> {
    TypeAttr { ds: &'a DataStructure<'a> },
    Variant { variant: &'a Struct<'a> },
    Field { field: &'a MyField<'a> },
}

pub(crate) fn parse_attrs_for_zeroed<'a>(
    ds: &'a DataStructure<'a>,
) -> Result<ZeroConfig<'a>, syn::Error> {
    let typaram_count = ds.generics.type_params().count();

    let mut this = ZeroableAttrs {
        extra_predicates: Vec::new(),
        unbounded_typarams: vec![IsBounded::Yes; typaram_count],
        debug_print: false,
        zeroable_field: None,
        repr_attr: ReprAttr::Rust,
        _marker: PhantomData,
    };

    parse_inner(&mut this, ds.attrs, ParseContext::TypeAttr { ds })?;

    for variant in &ds.variants {
        parse_inner(&mut this, variant.attrs, ParseContext::Variant { variant })?;
        for field in &variant.fields {
            parse_inner(&mut this, field.attrs, ParseContext::Field { field })?;
        }
    }

    ZeroConfig::new(this)
}

fn parse_inner<'a, I>(
    this: &mut ZeroableAttrs<'a>,
    attrs: I,
    pctx: ParseContext<'a>,
) -> Result<(), syn::Error>
where
    I: IntoIterator<Item = &'a Attribute>,
{
    for attr in attrs {
        match attr.parse_meta()? {
            Meta::List(list) => {
                parse_attr_list(this, list, pctx)?;
            }
            _ => {}
        }
    }
    Ok(())
}

fn parse_attr_list<'a>(
    this: &mut ZeroableAttrs<'a>,
    list: MetaList,
    pctx: ParseContext<'a>,
) -> Result<(), syn::Error> {
    if list.path.is_ident("zero") {
        with_nested_meta("zero", list.nested, |attr| {
            parse_sabi_attr(this, pctx, attr)
        })?;
    } else if list.path.is_ident("repr") {
        this.repr_attr = ReprAttr::new(list.nested)?;
    }
    Ok(())
}

fn parse_sabi_attr<'a>(
    this: &mut ZeroableAttrs<'a>,
    pctx: ParseContext<'a>,
    attr: Meta,
) -> Result<(), syn::Error> {
    match (pctx, attr) {
        (
            ParseContext::TypeAttr { .. },
            Meta::NameValue(MetaNameValue {
                lit: Lit::Str(ref value),
                ref path,
                ..
            }),
        ) => {
            if path.is_ident("bound") {
                this.extra_predicates.push(value.parse()?);
            } else {
                return_spanned_err! {path,"Unrecognized attribute"}
            }
        }
        (ParseContext::TypeAttr { ds }, Meta::List(list)) => {
            if list.path.is_ident("not_zeroable") {
                with_nested_meta("not_zeroable", list.nested, |attr| match &attr {
                    Meta::Path(path) => {
                        let tyident = path.get_ident().ok_or_else(|| {
                            spanned_err! {path,"Expected identifier for type parameter"}
                        })?;

                        let pos = ds
                            .generics
                            .type_params()
                            .position(|x| x.ident == *tyident)
                            .ok_or_else(|| spanned_err! {tyident,"Not a type parameter"})?;

                        this.unbounded_typarams[pos] = IsBounded::No;

                        Ok(())
                    }
                    _ => {
                        return_spanned_err! {attr,"Expected identifier for type parameter"}
                    }
                })?;
            } else {
                return_spanned_err! {list,"Unrecognized attribute"}
            }
        }
        (ParseContext::TypeAttr { .. }, Meta::Path(path)) => {
            if path.is_ident("debug_print") {
                this.debug_print = true;
            } else {
                return_spanned_err! {path,"Unrecognized attribute"}
            }
        }
        (ParseContext::Field { field }, Meta::Path(path)) => {
            if path.is_ident("zeroable") {
                this.zeroable_field = Some(field.index.pos);
            } else {
                return_spanned_err! {path,"Unrecognized attribute"}
            }
        }
        (_, attr) => {
            return_spanned_err! {attr,"Unrecognized attribute"}
        }
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////
