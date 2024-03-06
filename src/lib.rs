//! # Algorithms for Rust
#![deny(
    bad_style,
    dead_code,
    improper_ctypes,
    missing_debug_implementations,
    missing_docs,
    no_mangle_generic_items,
    non_shorthand_field_patterns,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    trivial_casts,
    trivial_numeric_casts,
    unconditional_recursion,
    unused_allocation,
    unused_comparisons,
    unused_extern_crates,
    unused_import_braces,
    unused_parens,
    unused_qualifications,
    unused_results,
    unused,
    while_true
)]
#![warn(clippy::all, clippy::pedantic, missing_debug_implementations)]
#![feature(cfg_match, decl_macro)]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

pub mod dynamic_programming;
pub mod macros;
pub mod math;
#[doc = include_str!("search/README.md")]
pub mod search;

pub mod higher_order_functions;
#[doc = include_str!("sorts/README.md")]
pub mod sorts;
pub mod strings;
