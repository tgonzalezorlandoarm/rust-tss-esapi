// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::unseparated_literal_suffix,
    improper_ctypes,
    missing_debug_implementations,
    trivial_casts,
    clippy::all,
    unused
)]
// Suppress warnings from bindgen-generated code
// Remove on resolution of
// https://github.com/rust-lang/rust-bindgen/issues/1651
#![allow(deref_nullptr)]
///! This crate provides a basic interface for accessing the TSS Enhanced
///! System API from Rust. No abstraction is provided beyond what is
///! automatically generated by `bindgen` - the `tss-esapi` crate is the
///! one exposing an idiomatic Rust interface built on top of this FFI.

// For supported targets: use the generated and committed bindings.
#[cfg(all(
    not(feature = "generate-bindings"),
    target_arch = "x86_64",
    target_os = "linux"
))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/bindings/x86_64-unknown-linux-gnu.rs"
));

#[cfg(all(
    not(feature = "generate-bindings"),
    target_arch = "aarch64",
    target_os = "linux"
))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/bindings/aarch64-unknown-linux-gnu.rs"
));

#[cfg(all(
    not(feature = "generate-bindings"),
    target_arch = "arm",
    target_os = "linux"
))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/bindings/arm-unknown-linux-gnueabi.rs"
));

#[cfg(all(
    not(feature = "generate-bindings"),
    target_arch = "x86_64",
    target_os = "macos"
))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/bindings/x86_64-unknown-darwin.rs"
));

// If the "generate-bindings" feature is on, use the generated bindings.
#[cfg(feature = "generate-bindings")]
include!(concat!(env!("OUT_DIR"), "/tss_esapi_bindings.rs"));