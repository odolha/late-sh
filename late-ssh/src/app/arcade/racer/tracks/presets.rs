//! Reusable building blocks for track authoring.
//!
//! Tracks should compose these via struct-update syntax instead of duplicating
//! field-by-field configurations. Example:
//!
//! ```ignore
//! use crate::app::arcade::racer::track::{Lane, LaneAspect};
//! use crate::app::arcade::racer::tracks::presets;
//!
//! const FAST_LANE: Lane = Lane {
//!     aspect: LaneAspect::AsphaltPremium,
//!     own_max_speed: 220.0,
//!     ..presets::HIGHWAY_LANE
//! };
//! ```

use crate::app::arcade::racer::track::{
    Car, Divider, DividerAspect, Lane, LaneAspect, Object, ObjectAspect, Scenery,
    SceneryBackground, Shoulder, ShoulderAspect,
};

// ─── Car shapes ─────────────────────────────────────────────────────────────

pub const CAR_SEDAN: Car = Car { height: 3, incidence: 0.45 };
pub const CAR_HATCHBACK: Car = Car { height: 3, incidence: 0.25 };
pub const CAR_VAN: Car = Car { height: 5, incidence: 0.20 };
pub const CAR_TRUCK: Car = Car { height: 7, incidence: 0.08 };
pub const CAR_SEMI: Car = Car { height: 11, incidence: 0.02 };

pub const CITY_CAR_MIX: &[Car] = &[CAR_SEDAN, CAR_HATCHBACK, CAR_VAN];
pub const HIGHWAY_CAR_MIX: &[Car] = &[CAR_SEDAN, CAR_HATCHBACK, CAR_VAN, CAR_TRUCK, CAR_SEMI];
pub const RURAL_CAR_MIX: &[Car] = &[CAR_HATCHBACK, CAR_VAN, CAR_TRUCK];

// ─── Lane templates ─────────────────────────────────────────────────────────

/// Standard city street lane — moderate speed, mixed traffic, no obstacles.
pub const CITY_LANE: Lane = Lane {
    aspect: LaneAspect::AsphaltStandard,
    own_min_speed: 0.0,
    own_max_speed: 150.0,
    passive_decel: 0.0,
    traffic_min_speed: 40.0,
    traffic_max_speed: 90.0,
    traffic_size: 12,
    traffic_cars: CITY_CAR_MIX,
    obstacles: &[],
};

/// Highway main lane — high speed, smooth asphalt.
pub const HIGHWAY_LANE: Lane = Lane {
    aspect: LaneAspect::AsphaltPremium,
    own_min_speed: 50.0,
    own_max_speed: 260.0,
    passive_decel: 0.0,
    traffic_min_speed: 60.0,
    traffic_max_speed: 150.0,
    traffic_size: 7,
    traffic_cars: HIGHWAY_CAR_MIX,
    obstacles: &[],
};

/// Outskirts / rural — patchy asphalt, slower.
pub const RURAL_LANE: Lane = Lane {
    aspect: LaneAspect::AsphaltPatchy,
    own_min_speed: 0.0,
    own_max_speed: 100.0,
    passive_decel: 2.0,
    traffic_min_speed: 30.0,
    traffic_max_speed: 80.0,
    traffic_size: 5,
    traffic_cars: RURAL_CAR_MIX,
    obstacles: &[],
};

/// Forest dirt road — slow, narrow feel.
pub const FOREST_LANE: Lane = Lane {
    aspect: LaneAspect::DirtRoad,
    own_min_speed: 0.0,
    own_max_speed: 70.0,
    passive_decel: 4.0,
    traffic_min_speed: 20.0,
    traffic_max_speed: 50.0,
    traffic_size: 3,
    traffic_cars: RURAL_CAR_MIX,
    obstacles: &[],
};

// ─── Dividers ───────────────────────────────────────────────────────────────

pub const URBAN_DIVIDERS: Divider = Divider {
    primary: DividerAspect::YellowDouble,
    lane: DividerAspect::WhiteDash,
};
pub const HIGHWAY_DIVIDERS: Divider = Divider {
    primary: DividerAspect::YellowDouble,
    lane: DividerAspect::WhiteDash,
};
pub const RURAL_DIVIDERS: Divider = Divider {
    primary: DividerAspect::YellowDash,
    lane: DividerAspect::Faint,
};
pub const FOREST_DIVIDERS: Divider = Divider {
    primary: DividerAspect::Faint,
    lane: DividerAspect::None,
};

// ─── Scenery objects ────────────────────────────────────────────────────────

pub const CITY_OBJECTS: &[Object] = &[
    Object { aspect: ObjectAspect::BuildingHouse, incidence: 0.35 },
    Object { aspect: ObjectAspect::BuildingApartments, incidence: 0.25 },
    Object { aspect: ObjectAspect::Skyscraper, incidence: 0.10 },
    Object { aspect: ObjectAspect::TreeOak, incidence: 0.20 },
    Object { aspect: ObjectAspect::Bush, incidence: 0.10 },
];

pub const HIGHWAY_OBJECTS: &[Object] = &[
    Object { aspect: ObjectAspect::TreePine, incidence: 0.50 },
    Object { aspect: ObjectAspect::TreeOak, incidence: 0.25 },
    Object { aspect: ObjectAspect::Bush, incidence: 0.20 },
    Object { aspect: ObjectAspect::Flower, incidence: 0.05 },
];

pub const RURAL_OBJECTS: &[Object] = &[
    Object { aspect: ObjectAspect::TreeOak, incidence: 0.45 },
    Object { aspect: ObjectAspect::Bush, incidence: 0.25 },
    Object { aspect: ObjectAspect::Grass, incidence: 0.20 },
    Object { aspect: ObjectAspect::Flower, incidence: 0.10 },
];

pub const FOREST_OBJECTS: &[Object] = &[
    Object { aspect: ObjectAspect::TreePine, incidence: 0.80 },
    Object { aspect: ObjectAspect::Bush, incidence: 0.20 },
];

pub const DESERT_OBJECTS: &[Object] = &[
    Object { aspect: ObjectAspect::Grass, incidence: 0.40 },
    Object { aspect: ObjectAspect::Bush, incidence: 0.40 },
    Object { aspect: ObjectAspect::TreePalm, incidence: 0.20 },
];

// ─── Sceneries ──────────────────────────────────────────────────────────────

pub const CITY_SCENERY: Scenery = Scenery {
    width: 14,
    background: SceneryBackground::Concrete,
    objects: CITY_OBJECTS,
};
pub const HIGHWAY_SCENERY: Scenery = Scenery {
    width: 12,
    background: SceneryBackground::Grass,
    objects: HIGHWAY_OBJECTS,
};
pub const RURAL_SCENERY: Scenery = Scenery {
    width: 10,
    background: SceneryBackground::Grass,
    objects: RURAL_OBJECTS,
};
pub const FOREST_SCENERY: Scenery = Scenery {
    width: 14,
    background: SceneryBackground::Dirt,
    objects: FOREST_OBJECTS,
};
pub const DESERT_SCENERY: Scenery = Scenery {
    width: 12,
    background: SceneryBackground::Dirt,
    objects: DESERT_OBJECTS,
};

// ─── Shoulders ──────────────────────────────────────────────────────────────

pub const SIDEWALK_SHOULDERS: &[Shoulder] = &[
    Shoulder { aspect: ShoulderAspect::HardEdge, repeat: 0 },
    Shoulder { aspect: ShoulderAspect::Sidewalk, repeat: 0 },
];

pub const HIGHWAY_SHOULDERS: &[Shoulder] = &[
    Shoulder { aspect: ShoulderAspect::HardEdge, repeat: 0 },
    Shoulder { aspect: ShoulderAspect::Poles, repeat: 6 },
];

pub const RURAL_SHOULDERS: &[Shoulder] = &[
    Shoulder { aspect: ShoulderAspect::SoftEdge, repeat: 0 },
];

pub const FOREST_SHOULDERS: &[Shoulder] = &[
    Shoulder { aspect: ShoulderAspect::SoftEdge, repeat: 0 },
    Shoulder { aspect: ShoulderAspect::TreePine, repeat: 4 },
];
