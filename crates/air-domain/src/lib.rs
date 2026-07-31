//! Pure domain models for AIR.SKILLS.
//! This crate contains zero I/O or external side-effects.

pub mod events;
pub mod models;
pub mod validation;

pub use events::*;
pub use models::*;
pub use validation::*;
