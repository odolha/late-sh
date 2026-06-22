//! Hard-coded track catalogue.
//!
//! Tracks live in their own files in this directory and are registered here.

pub mod presets;
pub mod sample;

use super::track::Track;

/// Every track available in the picker, in display order.
pub const ALL_TRACKS: &[&Track] = &[&sample::TRACK];

/// Default track loaded when none has been selected yet.
pub const DEFAULT_TRACK: &Track = &sample::TRACK;
