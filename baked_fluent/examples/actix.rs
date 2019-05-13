use actix_web::{web, App, HttpServer};

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

fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    HttpServer::new(|| App::new().service(web::resource("/{name}/{friend_count}/").to(index)))
        .bind("localhost:8088")?
        .run()
}
