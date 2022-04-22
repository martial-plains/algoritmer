//! Useful algorithms for working with strings

mod capitalize;
mod is_anagram;
mod is_palindrome;
mod is_pangram;
mod remove_duplicates;
mod reverse_words;
mod swap_case;

pub use capitalize::capitalize;
pub use is_anagram::is_anagram;
pub use is_palindrome::is_palindrome;
pub use is_pangram::is_pangram;
pub use remove_duplicates::remove_duplicates;
pub use reverse_words::reverse_words;
pub use swap_case::swap_case;

pub mod search;
