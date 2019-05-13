//! No special support is needed to use the [rouille](https://crates.io/crates/rouille)
//! microframework with `baked_fluent`. Simply create your localizer by hand:
//!
//! ```no_run
//! use baked_fluent::{localize, Localize, impl_localize};
//! use rouille::{router, Response, try_or_404};
//! impl_localize! {
//!     #[path("tests/i18n")]
//!     pub struct Localizer(_);
//! }
//!
//! fn main() {
//!     rouille::start_server("0.0.0.0:8080", move |request| {
//!         let loc = Localizer::new(&[], request.header("Accept-Language"));
//!         router!(request,
//!             (GET) (/{name: String}/{friend_count: isize}/) => {
//!                 Response::text(try_or_404!(localize!(loc, greeting, name=name, friends=friend_count)))
//!             },
//!             _ => Response::empty_404()
//!         )
//!     });
//! }
//! ```
