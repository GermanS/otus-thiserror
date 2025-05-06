pub mod device;
pub mod location;
pub mod report;

// Это как то феноменально, что Named сlippy помечает как unused_imports????
#[allow(unused_imports)]
pub use device::{Named, Pluggable};
pub use report::Reportable;
