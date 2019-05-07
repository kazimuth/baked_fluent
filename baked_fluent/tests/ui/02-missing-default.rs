use baked_fluent::impl_localize;

impl_localize! {
    #[localize(path = "../../../baked_fluent/tests/ui/i18n-missing-default", default_locale = "xy_ZW")]
    struct TestLocalizer(_);
}

fn main() {}
