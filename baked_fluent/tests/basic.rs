use baked_fluent::{impl_localize, localize, Localize};

impl_localize! {
    #[path("tests/i18n")]
    #[default_locale("en_US")]
    struct TestLocalizer(_);
}

#[test]
fn init() {
    // default
    let loc = TestLocalizer::new(&[], None);

    assert_eq!(
        localize!(loc, greeting, name = "Jamie", friends = 5).unwrap(),
        "Hello Jamie! You have 5 friends."
    );
}
