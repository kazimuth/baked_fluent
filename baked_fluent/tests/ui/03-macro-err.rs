use baked_fluent::impl_localize;

impl_localize! {
    #[path("whatever")]
    #[default_locale("bees")]
    #[invalid_thing("banana")]
    struct TestLocalizer(_);
}

fn main() {}
