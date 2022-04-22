//! # Algorithms for Rust

// #![deny(
//     bad_style,
//     const_err,
//     dead_code,
//     improper_ctypes,
//     missing_debug_implementations,
//     missing_docs,
//     no_mangle_generic_items,
//     non_shorthand_field_patterns,
//     overflowing_literals,
//     path_statements,
//     patterns_in_fns_without_body,
//     private_in_public,
//     trivial_casts,
//     trivial_numeric_casts,
//     unconditional_recursion,
//     unused_allocation,
//     unused_comparisons,
//     unused_extern_crates,
//     unused_import_braces,
//     unused_parens,
//     unused_qualifications,
//     unused_results,
//     unused,
//     while_true
// )]
#![feature(decl_macro)]
#![warn(clippy::all, missing_debug_implementations)]

pub mod dynamic_programming;
pub mod macros;
pub mod math;
pub mod sorts;
pub mod strings;
