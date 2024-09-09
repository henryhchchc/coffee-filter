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

mod macros;
mod sys;

pub mod jni;
pub mod jvmti;
