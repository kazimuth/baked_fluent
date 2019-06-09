//! Integrations with other frameworks.
//! See each individual modules for information on how to use that framework with baked_fluent.

#[cfg(feature = "with-actix")]
pub mod actix;

#[cfg(feature = "with-rouille")]
pub mod rouille;

pub mod maud;
