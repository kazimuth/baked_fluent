use actix_web::{web, App, HttpServer, Result};
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

fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::resource("/{name}/{friend_count}/").to(index)))
        .bind("localhost:8080")?
        .run()
}
