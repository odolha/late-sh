//! Track data model for the Racer game.
//!
//! A **track** is a named, multi-stage course. Each [`Stage`] defines the road
//! geometry, traffic, scenery, and theme that apply while the player is in
//! that stage. Stages succeed each other when the player covers their
//! `distance` budget; the next stage's road can have completely different
//! lane counts, themes, sceneries, and dividers.
//!
//! Tracks are hard-coded as `&'static` Rust data (no file I/O), so authoring
//! a new one is a code change. See `tracks/sample.rs` for an end-to-end
//! example and `tracks/presets.rs` for reusable building blocks.
//!
//! # How to add a new track
//!
//! 1. Add a new file under `racer/tracks/`, e.g. `mountain.rs`.
//! 2. Inside, declare a `pub const TRACK: Track = Track { … };` using the
//!    presets in [`super::presets`] to avoid repetition.
//! 3. Register it in `racer/tracks/mod.rs::ALL_TRACKS`.
//!
//! # Coordinate units
//!
//! All distance/speed values in this module are in **displayed** units
//! (the numbers the player sees on the speedometer and odometer).
//! [`Track::distance_scale`] and [`Track::speed_scale`] convert between
//! these and the internal physics units used by `racer::state`.

#![allow(dead_code)]

// ─── Track / Stage ──────────────────────────────────────────────────────────

/// A complete drivable course composed of one or more [`Stage`]s.
#[derive(Debug, Clone, Copy)]
pub struct Track {
    /// Title shown above the road and in the picker.
    pub name: &'static str,
    /// Credit shown next to the title.
    pub author: &'static str,
    /// Long-form description shown in the picker only (multi-line allowed).
    pub description: &'static str,
    /// Ordered list of stages. Must contain at least one entry.
    pub stages: &'static [Stage],
    /// Multiplies displayed distance vs. physics distance.
    /// `1.0` = 1 displayed km per 1 physics km.
    /// `0.5` = the player drives only 0.5 physics km to cover 1 displayed km
    ///         (i.e. distances feel "compressed" — longer-looking course in
    ///         less play time).
    pub distance_scale: f32,
    /// Multiplies the displayed speedometer reading vs. the physics speed.
    /// `1.0` = realistic. `2.0` = the speedometer reads 2× physics speed
    /// (the car *looks* twice as fast as it really moves).
    pub speed_scale: f32,
}

/// A contiguous segment of a [`Track`] with its own road, theme, and scenery.
#[derive(Debug, Clone, Copy)]
pub struct Stage {
    /// Stage name shown beneath the track name during play.
    pub name: &'static str,
    /// Optional short description shown in the bottom-right of the HUD.
    pub description: &'static str,
    /// Environmental icon (e.g. metropolis, forest) shown next to the track name.
    pub icon: StageIcon,
    /// Aesthetic theme applied to most rendered cells.
    pub theme: Theme,
    /// Displayed distance the player must travel before advancing to the
    /// next stage. Stored as displayed kilometres.
    pub distance_km: f32,
    /// Road geometry, lanes, scenery, and shoulders for this stage.
    pub road: Road,
}

// ─── Icons / Theme ──────────────────────────────────────────────────────────

/// Environmental icon, shown beside the track title. Theme-independent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StageIcon {
    Metropolis,
    City,
    CityOutskirts,
    Village,
    Highway,
    WildPlains,
    WildHills,
    WildForest,
    SlopeUp,
    SlopeDown,
    Special,
}

/// Visual theme. Applies to lane backgrounds, scenery, objects, shoulders.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Standard,
    Winter,
    Desert,
}

// ─── Road ───────────────────────────────────────────────────────────────────

/// Geometry, traffic, and scenery for a single stage.
#[derive(Debug, Clone, Copy)]
pub struct Road {
    pub aspect: RoadAspect,
    pub lanes: Lanes,
    pub sceneries: Sceneries,
    pub shoulders: Shoulders,
}

/// Road-wide aspects that don't apply per-lane.
#[derive(Debug, Clone, Copy)]
pub struct RoadAspect {
    pub dividers: Divider,
}

/// Divider styling between groups (primary) and within a group (lane).
#[derive(Debug, Clone, Copy)]
pub struct Divider {
    /// Divider between the two traffic directions (incoming vs. outgoing).
    pub primary: DividerAspect,
    /// Divider between two lanes in the same direction.
    pub lane: DividerAspect,
}

/// Visual style of a lane separator column.
///
/// Theme-independent. All are rendered in a single column with a glyph and color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DividerAspect {
    YellowDouble,
    YellowSingle,
    YellowDash,
    YellowDots,
    WhiteSingle,
    WhiteDash,
    WhiteDots,
    Faint,
    None,
}

// ─── Lanes ──────────────────────────────────────────────────────────────────

/// Lane configuration for both traffic directions.
///
/// `incoming` lists oncoming-traffic lanes, indexed *from the center divider
/// outward* (i.e. `incoming[0]` is the lane closest to the player's side).
/// `outgoing` lists same-direction lanes the player drives in, also indexed
/// from the center outward.
///
/// **Invariants:** at least one `outgoing` lane; at least 2 lanes in total.
/// Left- vs. right-hand driving is a future user setting and does *not* affect
/// the meaning of these fields — `incoming` always means "the direction the
/// player is *not* moving in".
#[derive(Debug, Clone, Copy)]
pub struct Lanes {
    pub incoming: &'static [Lane],
    pub outgoing: &'static [Lane],
}

/// One lane: surface, allowed speeds, traffic, and obstacles.
#[derive(Debug, Clone, Copy)]
pub struct Lane {
    pub aspect: LaneAspect,
    /// Minimum displayed km/h the player may drive at on this lane.
    pub own_min_speed: f32,
    /// Maximum displayed km/h the player may drive at on this lane.
    pub own_max_speed: f32,
    /// Passive deceleration in displayed km/h per second when the player
    /// is not pressing accelerate. Drives the player toward `own_min_speed`.
    pub passive_decel: f32,
    /// Lower bound (displayed km/h) for AI cars spawning on this lane.
    pub traffic_min_speed: f32,
    /// Upper bound (displayed km/h) for AI cars spawning on this lane.
    pub traffic_max_speed: f32,
    /// Approximate total AI car count to maintain on this lane across the
    /// full spawn area (visible + look-ahead). Not the on-screen count.
    pub traffic_size: u16,
    /// Catalogue of AI car body shapes for this lane. Weighted random pick.
    pub traffic_cars: &'static [Car],
    /// Obstacles randomly sprinkled along this lane.
    pub obstacles: &'static [Obstacle],
}

/// Lane surface. Themed — see `theme::lane_aspect_cell`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaneAspect {
    AsphaltPremium,
    AsphaltStandard,
    AsphaltPatchy,
    GravelRoad,
    DirtRoad,
    Grass,
}

/// AI car body. `height` is in rows; width is always 3 chars.
#[derive(Debug, Clone, Copy)]
pub struct Car {
    /// Height in rendered rows. Common values: 3 (sedan), 4–5 (van), 6+ (truck).
    pub height: u8,
    /// Relative weight when picking a car shape at spawn time. Higher = more common.
    pub incidence: f32,
}

// ─── Obstacles ──────────────────────────────────────────────────────────────

/// Stationary hazard sprinkled along a lane.
#[derive(Debug, Clone, Copy)]
pub struct Obstacle {
    pub aspect: ObstacleAspect,
    /// Coverage fraction along this lane in `[0.0, 1.0]`. Roughly: average
    /// fraction of lane-length occupied by this obstacle type. Placement
    /// is deterministic-but-pseudo-random along the lane.
    pub frequency: f32,
    /// Effects that fire when the player drives over the obstacle.
    /// Applied once on crossing — they do not persist across the obstacle.
    pub effects: &'static [ObstacleEffect],
}

/// Obstacle visual / category. Theme-independent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObstacleAspect {
    PotholeSmall,
    PotholeBig,
    PotholeCrater,
    SpeedBump,
    Spikes,
    FallenTree,
}

/// One-shot effect triggered when the player crosses an obstacle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ObstacleEffect {
    /// Instant game-over.
    Crash,
    /// Multiplicative speed change applied to current speed.
    /// `affect = -1.0` → full stop. `affect = +0.5` → +50%. Always clamped to
    /// the current lane's min/max bounds afterwards.
    SpeedChange { affect: f32 },
    /// Player can't accelerate for `cooldown_ms` milliseconds.
    BlockGas { cooldown_ms: u16 },
    /// Player can't brake for `cooldown_ms` milliseconds.
    BlockBreaks { cooldown_ms: u16 },
    /// Player can't change lanes for `cooldown_ms` milliseconds.
    BlockWheels { cooldown_ms: u16 },
}

// ─── Scenery ────────────────────────────────────────────────────────────────

/// Left and right scenery slabs flanking the road.
///
/// "Left" and "right" are absolute screen sides — they do **not** flip when
/// the user switches to left-hand driving in a future setting.
#[derive(Debug, Clone, Copy)]
pub struct Sceneries {
    pub left: Scenery,
    pub right: Scenery,
}

/// One side's scenery: a band of fixed character width, with a uniform
/// background and randomly-placed objects on top.
#[derive(Debug, Clone, Copy)]
pub struct Scenery {
    /// Width of this scenery band in characters.
    pub width: u8,
    /// Background fill. Themed.
    pub background: SceneryBackground,
    /// Catalogue of objects to scatter on top of the background.
    /// Objects only spawn in cells *not* covered by a shoulder.
    pub objects: &'static [Object],
}

/// Scenery background. Themed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SceneryBackground {
    Concrete,
    Grass,
    Dirt,
    Void,
}

/// One scattered scenery object.
#[derive(Debug, Clone, Copy)]
pub struct Object {
    pub aspect: ObjectAspect,
    /// Relative weight when picking which object to render at a given cell.
    pub incidence: f32,
}

/// Scenery object. ASCII-rendered. Theme-aware.
///
/// Multi-row objects (apartments, skyscraper) extend upward from their anchor
/// cell. If the scenery band isn't wide enough (or the cell is occupied by a
/// shoulder), the object is skipped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectAspect {
    Grass,
    Bush,
    TreePine,
    TreeOak,
    TreePalm,
    BuildingHouse,
    BuildingApartments,
    /// Tall city building (the spec's "Pedestrian" variant was a typo for this).
    Skyscraper,
    Flower,
    Star,
}

// ─── Shoulders ──────────────────────────────────────────────────────────────

/// Shoulder strips on each side of the road, drawn on top of scenery.
///
/// Each [`Shoulder`] occupies exactly 1 character column. Total shoulder
/// width on each side must not exceed that side's scenery width.
#[derive(Debug, Clone, Copy)]
pub struct Shoulders {
    /// Listed from the road edge outward (`left[0]` is adjacent to the road).
    pub left: &'static [Shoulder],
    /// Listed from the road edge outward.
    pub right: &'static [Shoulder],
}

/// One shoulder column.
#[derive(Debug, Clone, Copy)]
pub struct Shoulder {
    pub aspect: ShoulderAspect,
    /// Repetition period in rows: `0` = continuous; `n > 0` = every `n` rows.
    pub repeat: u8,
}

/// Visual content of a shoulder column. Themed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShoulderAspect {
    Sidewalk,
    HardEdge,
    SoftEdge,
    ParkedCar,
    Poles,
    Railroad,
    River,
    CountryRoad,
    TreePine,
    TreeOak,
    TreePalm,
}

// ─── Helpers ────────────────────────────────────────────────────────────────

impl Track {
    /// Total displayed track length across all stages.
    pub fn total_distance_km(&self) -> f32 {
        self.stages.iter().map(|s| s.distance_km).sum()
    }
}

impl Lanes {
    /// Total number of lanes across both directions.
    pub fn total(&self) -> usize {
        self.incoming.len() + self.outgoing.len()
    }

    /// Look up a lane by its flat index (0-based, incoming first).
    ///
    /// Layout: `[incoming[0], …, incoming[n-1], outgoing[0], …, outgoing[m-1]]`.
    pub fn get(&self, flat_idx: usize) -> Option<&Lane> {
        if flat_idx < self.incoming.len() {
            Some(&self.incoming[flat_idx])
        } else {
            self.outgoing.get(flat_idx - self.incoming.len())
        }
    }

    /// Flat index of the first outgoing lane (player's default starting lane).
    pub fn player_start_idx(&self) -> usize {
        self.incoming.len()
    }

    /// True if the lane at `flat_idx` is an incoming (oncoming) lane.
    pub fn is_incoming(&self, flat_idx: usize) -> bool {
        flat_idx < self.incoming.len()
    }
}
