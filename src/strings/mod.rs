//! Useful algorithms for working with strings

mod anagram;
mod capitalize;
mod jaro_winkler;
mod palindrome;
mod pangram;
mod remove_duplicates;
mod reverse_words;
mod swap_case;

pub use anagram::*;
pub use capitalize::*;
pub use jaro_winkler::*;
pub use palindrome::*;
pub use pangram::*;
pub use remove_duplicates::*;
pub use reverse_words::*;
pub use swap_case::*;

pub mod search;
