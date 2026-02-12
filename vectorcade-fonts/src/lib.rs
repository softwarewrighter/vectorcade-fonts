//! Vector font implementations.
//!
//! These are intentionally simple stroke fonts; fidelity can be improved later.

mod atari;
mod cinematronics;
mod registry;

pub use atari::AtariMini;
pub use cinematronics::Cinematronics;
pub use registry::FontRegistry;
