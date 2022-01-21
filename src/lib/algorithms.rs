pub mod all;
pub mod one;
pub mod two;

pub use all::*;
pub use one::*;
pub use two::*;

/// Best algorithm
pub fn run_all() -> (usize, &'static str) {
    (0, "")
}
