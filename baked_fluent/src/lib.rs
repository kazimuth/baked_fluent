//! A library for easy serverside internationalization / localization using the
//! [Fluent](https://projectfluent.org) i18n system to parse and apply translations.
//!
//! # Quickstart
//! Create a folder structure at your project root, organized like so:
//! - `i18n`
//!   - `en_US`
//!     - `greeting.ftl`
//!   - `es_MX`
//!     - `greeting.ftl`
//!
//! Add some localizations in [Fluent's `FTL` Syntax](https://projectfluent.org/fluent/guide/):
//!
//! `i18n/en_US/greeting.ftl`:
//! ```txt
//! hello = Hello, $name!
//! age.tracker = You are $age_hours hours old.
//! ```
//! `i18n/es_MX/greeting.ftl`:
//! ```txt
//! hello = ¡Hola, $name!
//! age.tracker = Tiene $age_hours horas.
//! ```
//!
//! Call the `impl_localize!()` macro:
//! ```
//! # #[cfg(feature = "with-i18n")]
//! # mod lmao_rustc {
//! extern crate baked_fluent;
//! use baked_fluent::{Localize, impl_localize};
//!
//! impl_localize! {
//!     #[localize(path = "i18n", default_locale = "en_US")]
//!     pub struct AppLocalizer(_);
//! }
//! # }
//! ```
//!
//! This creates a struct called `AppLocalizer` which implements the askama `Localize` trait.
//!
//! This will bake translations you provide into the output executable, to ease
//! deployment; all you need is one binary.
//!
//! TODO: To create an instance
//!
//! Now, you can use the `localize` and `localize_into` ma

pub mod integrations;
pub mod runtime;

/// `Localize` trait; can be included in templates to allow using the `localize` filter.
/// Implementations are generally derived.
pub trait Localize: Sized {
    // Implementation notes:
    // All of the code that actually talks to fluent is in the `baked_fluent::runtime` module.
    // Codegen for `impl_localize!` is in `baked_fluent_codegen`.

    /// Create a localizer.
    ///
    /// Every localizer contains a chain of locales to look up messages in. If it can't find a message in the
    /// first locale, it will move on to the second, and so on.
    ///
    /// - `user_locales`: a list of locales preferred by the user, in descending order of preference.
    /// - `accept_language`: an `Accept-Language` HTTP header, if present.
    fn new(user_locales: &[&str], accept_language: Option<&str>) -> Self;

    /// Localize a particular message.
    fn localize(
        &self,
        message_id: &'static str,
        args: &[(&str, &runtime::I18nValue)],
    ) -> Result<String> {
        let mut result = String::new();
        self.localize_into(&mut result, message_id, args)?;
        Ok(result)
    }

    /// Localize a particular message into a std::fmt::Write.
    fn localize_into<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        message_id: &'static str,
        args: &[(&str, &runtime::I18nValue)],
    ) -> Result<()>;

    /// Whether a localizer has a particular message available.
    fn has_message(&self, message_id: &'static str) -> bool;

    /// Get the chain of locales this localizer looks up messages in.
    fn locale_chain(&self) -> &[&'static str];

    /// Default locale of this Localize implementation.
    fn default_locale() -> &'static str;
}

#[macro_export]
macro_rules! localize {
    ($localizer:expr, $message:ident $(. $attr:ident)* $(, $key:ident = $val:expr)* $(,)*) => {
        $crate::Localize::localize(&$localizer, concat!(stringify!($message), $(".", stringify!($attr)),*), &[
            $((stringify!($key), &$val.into())),*
        ])
    };
}

#[macro_export]
macro_rules! localize_into {
    ($localizer:expr, $writer:expr, $message:ident $(. $attr:ident)* $(, $key:ident = $val:expr)* $(,)*) => {
        $crate::Localize::localize_into(&$localizer, $writer, concat!(stringify!($message), $(".", stringify!($attr)),*), &[
            $((stringify!($key), &$val.into())),*
        ])
    };
}

/// An error in localization.
#[derive(Debug, Clone)]
pub enum Error {
    NoTranslations {
        message: &'static str,
        locale_chain: Box<[&'static str]>,
    },
    Fmt(std::fmt::Error),
}
impl From<std::fmt::Error> for Error {
    fn from(err: std::fmt::Error) -> Self {
        Error::Fmt(err)
    }
}
impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NoTranslations { .. } => "no translations",
            Error::Fmt(..) => "formatter error",
        }
    }
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Fmt(ref e) => Some(e),
            _ => None,
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::NoTranslations {
                ref message,
                ref locale_chain,
            } => write!(
                f,
                "no non-erroring translations for message {} in locale chain {:?}",
                message, locale_chain
            ),
            Error::Fmt(ref e) => write!(f, "fmt error: {}", e),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub use baked_fluent_codegen::impl_localize;

#[cfg(test)]
mod tests {
    use super::*;
    struct T;
    impl Localize for T {
        fn new(_: &[&str], _: Option<&str>) -> Self {
            T
        }
        fn localize_into<W: std::fmt::Write>(
            &self,
            writer: &mut W,
            message_id: &str,
            args: &[(&str, &runtime::I18nValue)],
        ) -> Result<()> {
            write!(writer, "localize {:?} {:?}", message_id, args)?;
            Ok(())
        }
        fn has_message(&self, _: &str) -> bool {
            true
        }
        /// Get the chain of locales this localizer looks up messages in.
        fn locale_chain(&self) -> &[&'static str] {
            &["en-US"]
        }

        fn default_locale() -> &'static str {
            "en_US"
        }
    }
    #[test]
    fn localize_macro() -> Result<()> {
        let _ = pretty_env_logger::try_init();
        let t = T;
        assert_eq!(
            localize!(t, bees.banana, x = 1, y = "hello", z = "there".to_string())?,
            "localize \"bees.banana\" [(\"x\", Number(\"1\")), (\"y\", String(\"hello\")), (\"z\", String(\"there\"))]"
        );
        let mut result = String::new();

        localize_into!(
            t,
            &mut result,
            bees.banana,
            x = 1,
            y = "hello",
            z = "there".to_string()
        )?;
        assert_eq!(
            result,
            "localize \"bees.banana\" [(\"x\", Number(\"1\")), (\"y\", String(\"hello\")), (\"z\", String(\"there\"))]"
        );

        Ok(())
    }

    #[test]
    fn ui() {
        let _ = pretty_env_logger::try_init();
        let t = trybuild::TestCases::new();

        t.compile_fail("tests/ui/*.rs");
    }
}
