use failure::Fail;

pub mod runtime;

/// `Localize` trait; can be included in templates to allow using the `localize` filter.
/// Implementations are generally derived.
pub trait Localize: Sized {
    // Implementation notes:
    // All of the code that actually talks to fluent is in the `askama_shared::i18n::macro_impl` module.
    // Codegen for `impl_localize!` is in `askama_derive::gen_impl_localize`.
    // Codegen for the `localize` filter is in `askama_derive::generator`.

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
    ) -> Result<String>;

    /// Whether a localizer has a particular message available.
    fn has_message(&self, message_id: &'static str) -> bool;

    /// Default locale of this Localize implementation.
    fn default_locale() -> &'static str;
}

#[macro_export]
macro_rules! localize {
    ($localizer:expr, $message:ident $(. $attr:ident)* $(, $key:ident = $val:expr)* $(,)*) => {
        $localizer.localize(concat!(stringify!($message), $(".", stringify!($attr)),*), &[
            $((stringify!($key), &$val.into())),*
        ])
    };
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(
        display = "no non-erroring translations for message {} in locale chain {:?}",
        message, locale_chain
    )]
    NoTranslations {
        message: &'static str,
        locale_chain: Box<[&'static str]>,
    },
    #[fail(display = "io error: {}", _0)]
    Io(std::io::Error),
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
        fn localize(
            &self,
            message_id: &str,
            args: &[(&str, &runtime::I18nValue)],
        ) -> Result<String> {
            Ok(format!("localize {:?} {:?}", message_id, args))
        }
        fn has_message(&self, _: &str) -> bool {
            true
        }
        fn default_locale() -> &'static str {
            "en_US"
        }
    }
    #[test]
    fn localize_macro() {
        let t = T;
        assert_eq!(
            localize!(t, bees.banana, x = 1, y = "hello", z = "there".to_string()).unwrap(),
            "localize \"bees.banana\" [(\"x\", Number(\"1\")), (\"y\", String(\"hello\")), (\"z\", String(\"there\"))]"
        );
    }

    #[test]
    fn ui() {
        let t = trybuild::TestCases::new();

        t.compile_fail("tests/ui/*.rs");
    }
}
