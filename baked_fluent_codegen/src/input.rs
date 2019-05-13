/// Input parser for the `impl_localize!` proc-macro.
use syn::parse::{Parse, ParseStream, Result};
use syn::{bracketed, parenthesized, Ident, LitBool, LitStr, Token};

/// An invocation of impl_localize
pub struct ImplLocalize {
    pub name: Ident,
    pub path: LitStr,
    pub default_locale: LitStr,
    pub custom_from_request: bool,
}

impl Parse for ImplLocalize {
    fn parse(input: ParseStream) -> Result<Self> {
        // parse #[thing(stuff)] options
        let mut path = None;
        let mut default_locale = None;
        let mut custom_from_request = false;
        loop {
            if !input.lookahead1().peek(Token![#]) {
                break;
            }
            input.parse::<Token![#]>()?;
            let ann;
            bracketed!(ann in input);

            let ann_name = ann.parse::<Ident>()?;
            match &*ann_name.to_string() {
                "path" => path = Some(Arg::<LitStr>::parse(&ann)?.value),
                "default_locale" => default_locale = Some(Arg::<LitStr>::parse(&ann)?.value),
                "custom_from_request" => {
                    custom_from_request = Arg::<LitBool>::parse(&ann)?.value.value
                }
                _ => {
                    return Err(syn::parse::Error::new(
                        ann_name.span(),
                        format!(
                            "unexpected attribute `{}` (allowed: path, default_locale)",
                            ann_name
                        ),
                    ))
                }
            }
        }

        // parse boilerplate `[pub] struct WhateverLocalizer(_);`
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![pub]) {
            // note: currently the output is always pub. Shrug emoji
            input.parse::<Token![pub]>()?;
        }

        input.parse::<Token![struct]>()?;
        let name = input.parse::<Ident>()?;
        let dummy;
        parenthesized!(dummy in input);
        dummy.parse::<Token![_]>()?;
        input.parse::<Token![;]>()?;

        let path = path.unwrap_or(LitStr::new("i18n", name.span()));
        let default_locale = default_locale.unwrap_or(LitStr::new("en_US", name.span()));

        Ok(ImplLocalize {
            name,
            path,
            default_locale,
            custom_from_request,
        })
    }
}

pub struct Arg<T: Parse> {
    pub value: T,
}
impl<T: Parse> Parse for Arg<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        let arg;
        parenthesized!(arg in input);
        Ok(Arg {
            value: T::parse(&arg)?,
        })
    }
}
