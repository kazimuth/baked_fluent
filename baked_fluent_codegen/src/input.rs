/// Input parser for the `impl_localize!` proc-macro.
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{bracketed, parenthesized, token, Ident, LitStr, Token};

/// An invocation of impl_localize, of the form:
///
/// ```no_build
/// impl_localize! {
///     #[localize(path = "i18n", default_locale = "en_US")]
///     pub struct AppLocalizer(_);
/// }
/// ```
///
/// This creates a struct called `AppLocalizer` which implements the Askama `Localize` trait.
///
/// For more information, see the top-level documentation for Askama.
pub struct ImplLocalize {
    pub name: Ident,
    pub path: LitStr,
    pub default_locale: LitStr,
}

impl Parse for ImplLocalize {
    fn parse(input: ParseStream) -> Result<Self> {
        // parse annotation #[localize(...)]
        let mut path = None;
        let mut default_locale = None;

        input.parse::<Token![#]>()?;
        let annotation;
        bracketed!(annotation in input);

        let ann_name = annotation.parse::<Ident>()?;
        if ann_name.to_string() != "localize" {
            return Err(syn::parse::Error::new(
                ann_name.span(),
                "expected `#[localize]` or `#[localize(path = \"...\", default_locale = \"...\")]",
            ));
        }

        let lookahead = annotation.lookahead1();
        if lookahead.peek(token::Paren) {
            let args;
            parenthesized!(args in annotation);
            let args = Punctuated::<NamedArg, Token![,]>::parse_terminated(&args)?;
            for arg in args.iter() {
                match &arg.name.to_string()[..] {
                    "path" => path = Some(arg.value.clone()),
                    "default_locale" => default_locale = Some(arg.value.clone()),
                    _ => {
                        return Err(syn::parse::Error::new(
                            arg.name.span(),
                            "expected one of `path = \"...\"`, `default_locale = \"...\"`",
                        ));
                    }
                }
            }
        }

        let path = path.unwrap_or(LitStr::new("i18n", ann_name.span()));
        let default_locale = default_locale.unwrap_or(LitStr::new("en_US", ann_name.span()));

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

        Ok(ImplLocalize {
            name,
            path,
            default_locale,
        })
    }
}

/// A named argument to an annotation, like in #[cfg(thing = "bees")]
///                                                  ^^^^^^^^^^^^^^ this bit
pub struct NamedArg {
    pub name: Ident,
    pub value: LitStr,
}

impl Parse for NamedArg {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?;
        input.parse::<Token![=]>()?;
        let value = input.parse::<LitStr>()?;
        Ok(NamedArg { name, value })
    }
}
