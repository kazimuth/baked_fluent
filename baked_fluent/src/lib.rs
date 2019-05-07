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
    fn localize(&self, message_id: &str, args: &[(&str, &runtime::I18nValue)]) -> Result<String>;

    /// Whether a localizer has a particular message available.
    fn has_message(&self, message_id: &str) -> bool;

    /// Default locale of this localizer.
    fn default_locale() -> &'static str;
}

#[macro_export]
macro_rules! localize {
    ($localizer:expr, $message:ident $(. $attr:ident)*, $($key:ident = $val:expr),*) => {
        $localizer.localize(concat!(stringify!($message), $(".", stringify!($attr)),*), &[
            $((stringify!($key), &$val.into())),*
        ])
    };
}

#[derive(Debug)]
pub struct Error(String);

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;
    struct T {}
    impl Localize for T {
        fn new(user_locales: &[&str], accept_language: Option<&str>) -> Self {
            T {}
        }
        fn localize(
            &self,
            message_id: &str,
            args: &[(&str, &runtime::I18nValue)],
        ) -> Result<String> {
            println!("localize {:?} {:?}", message_id, args);
            Ok("bees".into())
        }
        fn has_message(&self, message_id: &str) -> bool {
            true
        }
        fn default_locale() -> &'static str {
            "en_US"
        }
    }
    #[test]
    fn test() {
        let t = T {};
        localize!(t, bees.banana, x = 1, y = "hello", z = "fuck".to_string()).unwrap();
        panic!();
    }
}
