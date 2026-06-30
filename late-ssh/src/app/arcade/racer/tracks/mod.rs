//! Hard-coded track catalogue.
//!
//! Tracks live in their own files in this directory and are registered here.

pub mod presets;
pub mod batin;
pub mod route66;
pub mod eurotrip;
pub mod fantasy;
pub mod solar_system;
pub mod crazy;

#[cfg(debug_assertions)]
pub mod test;
#[cfg(debug_assertions)]
pub mod sample;

use super::track::Track;

/// Every track available in the picker, in display order.
#[cfg(debug_assertions)]
pub const ALL_TRACKS: &[&Track] = &[
    &test::TRACK,
    &sample::TRACK,
    &batin::TRACK,
    &route66::TRACK,
    &eurotrip::TRACK,
    &fantasy::TRACK,
    &solar_system::TRACK,
    &crazy::TRACK,
];

#[cfg(not(debug_assertions))]
pub const ALL_TRACKS: &[&Track] = &[
    &batin::TRACK,
    &route66::TRACK,
    &eurotrip::TRACK,
    &fantasy::TRACK,
    &solar_system::TRACK,
    &crazy::TRACK,
];

/// Default track loaded when none has been selected yet.
#[cfg(debug_assertions)]
pub const DEFAULT_TRACK: &Track = &test::TRACK;

#[cfg(not(debug_assertions))]
pub const DEFAULT_TRACK: &Track = &batin::TRACK;
