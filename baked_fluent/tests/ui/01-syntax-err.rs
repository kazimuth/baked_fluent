use baked_fluent::impl_localize;

impl_localize! {
    #[localize(path = "../../../baked_fluent/tests/ui/i18n-syntax-err", default_locale = "en_US")]
    struct TestLocalizer(_);
}

fn main() {}
