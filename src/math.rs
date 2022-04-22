//! Perform mathematical operations and manipulate integer, float, and double values.

mod abs;
mod calculate_factorial;
mod ceil;
mod floor;
mod is_perfect;
mod power;

pub mod fibonacci;

pub use abs::*;
pub use calculate_factorial::calculate_factorial;
pub use ceil::ceil;
pub use floor::floor;
pub use is_perfect::is_perfect;
pub use power::power;
