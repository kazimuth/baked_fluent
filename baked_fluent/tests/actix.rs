#![cfg(feature = "with-actix")]

use actix_web::{http::StatusCode, test, web, App, Result};

use baked_fluent::{impl_localize, localize};

// Create a struct called `Localizer` implementing `baked_fluent::Localize`
impl_localize! {
    #[path("tests/i18n")]
    #[default_locale("en_US")]
    pub struct Localizer(_);
}

fn index((loc, info): (Localizer, web::Path<(String, isize)>)) -> Result<String> {
    Ok(localize!(
        loc,
        greeting,
        name = &info.0[..],
        friends = info.1
    )?)
}

#[test]
fn actix() {
    let mut app =
        test::init_service(App::new().service(web::resource("/{name}/{friend_count}/").to(index)));

    // Create request object
    let req = test::TestRequest::with_uri("/Jamie/12/")
        .header("Accept-Language", "es")
        .to_request();

    // Call application
    let resp = test::call_service(&mut app, req);
    assert_eq!(resp.status(), StatusCode::OK);
    let body = &test::read_body(resp);
    let body = std::str::from_utf8(body).unwrap();
    assert_eq!(body, "¡Hola, Jamie! Tienes 12 amigos.");
}
