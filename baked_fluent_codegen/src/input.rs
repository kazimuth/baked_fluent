/// Input parser for the `impl_localize!` proc-macro.
use syn::parse::{Parse, ParseStream, Result};
use syn::{bracketed, parenthesized, Ident, LitStr, Path, Token};

/// An invocation of impl_localize, of the form:
///
/// ```no_build
/// impl_localize! {
///     #[path("i18n")]
///     #[default_locale = "en_US")]
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
    pub actix: Option<ActixOpts>,
}

/// Customization for the "actix" framework.
pub struct ActixOpts {
    pub custom_user_locale: Option<Path>,
}

impl Parse for ImplLocalize {
    fn parse(input: ParseStream) -> Result<Self> {
        // parse #[thing(stuff)] options
        let mut path = None;
        let mut default_locale = None;
        let mut actix = None;

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
                "actix" => {
                    actix = Some(if input.lookahead1().peek(syn::token::Paren) {
                        unimplemented!()
                    } else {
                        ActixOpts {
                            custom_user_locale: None,
                        }
                    })
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
            actix,
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

/// A named argument to an ann, like in #[cfg(thing = "bees")]
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
