#![warn(
    missing_debug_implementations,
    rust_2018_idioms,
    // missing_docs,
    rust_2021_compatibility,
    future_incompatible,
    clippy::pedantic
)]
//! # Coffee Filter
//! Rust bindings for the JVM Tool Interface (JVM TI).

pub mod agent_callback;
pub mod jvm;
mod macros;
mod prelude;
mod sys;
