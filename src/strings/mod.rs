//! Useful algorithms for working with strings

mod capitalize;
mod is_anagram;
mod is_palindrome;
mod is_pangram;
mod jaro_winkler;
mod remove_duplicates;
mod reverse_words;
mod swap_case;

pub use capitalize::*;
pub use is_anagram::*;
pub use is_palindrome::*;
pub use is_pangram::*;
pub use jaro_winkler::*;
pub use remove_duplicates::*;
pub use reverse_words::*;
pub use swap_case::*;

pub mod search;
