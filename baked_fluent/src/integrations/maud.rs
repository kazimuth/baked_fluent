//! No special support is needed to use the [maud](https://maud.lambda.xyz/)
//! templating library with `baked_fluent`. Simply pass a localizer into your
//! [partial function](https://maud.lambda.xyz/partials.html):
//!
//! ```no_build
//! use baked_fluent::{impl_localize, Localize, localize};
//! use maud::{DOCTYPE, html, Markup};
//! impl_localize! {
//! #     #[path("tests/i18n")]
//!     pub struct Localizer(_);
//! }
//!
//! fn header(loc: &Localizer) -> Markup {
//!    html! {
//!        h1 {
//!            (localize!(loc, "greeting", name="Jamie", friends="5"))
//!        }
//!    }
//! }
//! ```
