//! To use `baked_fluent` with [actix-web](https://actix.rs/), enable the `"with-actix"` feature in your Cargo.toml.
//!
//! ```no_run
//! use actix_web::{web, App, HttpServer, Result};
//! use baked_fluent::{impl_localize, localize};
//!
//! // Create a struct called `Localizer` implementing `baked_fluent::Localize`
//! impl_localize! {
//! #    #[path("tests/i18n")]
//!     pub struct Localizer(_);
//! }
//!
//! // When with-actix is enabled, your Localize impl will automatically implement actix_web::FromRequest;
//! // use it like so to create a localizer automatically.
//! // By default, it is created from the Accept-Language header; to override this behavior, see the
//! // custom_from_request baked_fluent example.
//! fn index((loc, info): (Localizer, web::Path<(String, isize)>)) -> Result<String> {
//!     Ok(localize!(
//!         loc,
//!         greeting,
//!         name = &info.0[..],
//!         friends = info.1
//!     )?)
//! }
//!
//! fn main() -> std::io::Result<()> {
//!     HttpServer::new(|| App::new().service(web::resource("/{name}/{friend_count}/").to(index)))
//!         .bind("localhost:8080")?
//!         .run()
//! }
//! ```
//!

impl actix_web::ResponseError for super::super::Error {}
