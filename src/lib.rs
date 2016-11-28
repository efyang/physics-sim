#[cfg(feature = "simd-opts")]
extern crate simd;

mod point;
mod vector;
mod object;
mod gravity;
mod universe;

#[cfg(feature = "simd-opts")]
mod simd_opts;

pub use point::Point;
pub use vector::Vector;
pub use object::Object;
pub use universe::Universe;
pub use gravity::gravitational_forces;
