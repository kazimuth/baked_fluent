//! Integrations with other frameworks.
//! See each individual module for information on how to use that framework with baked_fluent.

#[cfg(feature = "with-actix")]
pub mod actix;

#[cfg(feature = "with-maud")]
pub mod maud;
