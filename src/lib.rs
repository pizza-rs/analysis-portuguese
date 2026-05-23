#![cfg_attr(not(feature = "std"), no_std)]
//! Portuguese language analysis for Pizza search engine.
//!
//! Provides a full-featured Portuguese analyzer with light stemming and stop words.
extern crate alloc;
mod stem;
mod stop;

pub mod register;

pub use register::register_all;
pub use stem::PortugueseLightStemFilter;
pub use stop::PortugueseStopFilter;
