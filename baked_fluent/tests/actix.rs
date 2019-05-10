use actix_web::{http, test, web, HttpMessage, HttpRequest, HttpResponse};

use baked_fluent::{impl_localize, localize};

// Create a struct called `Localizer` implementing `baked_fluent::Localize`
impl_localize! {
    #[path("tests/i18n")]
    #[default_locale("en_US")]
    #[actix]
    pub struct Localizer(_);
}

fn index((loc, info): (Localizer, web::Path<(String, isize)>)) -> String {
    localize!(loc, greeting, name = &info.0[..], friends = info.1).unwrap()
}

/*
#[test]
fn test_accept_langauge() {
    let resp = test::TestRequest::with_header("Accept-Language", "en_US")
        .uri("http://localhost/Jamie/12")
        .run(&index)
        .unwrap();
    let resp = test::block_on(index(req)).unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.status(), http::StatusCode::OK);
    assert_eq!(resp.body(), "banana");
}
*/
