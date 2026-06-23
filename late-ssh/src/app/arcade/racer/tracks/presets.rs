//! Reusable building blocks for track authoring.
//!
//! Tracks should compose these via struct-update syntax instead of duplicating
//! field-by-field configurations.  Example:
//!
//! ```ignore
//! use crate::app::arcade::racer::tracks::presets;
//!
//! const FAST_LANE: Lane = Lane {
//!     style: theme::LANE_ASPHALT_PREMIUM,
//!     own_max_speed: 220.0,
//!     ..presets::HIGHWAY_LANE
//! };
//! ```

use crate::app::arcade::racer::theme;
use crate::app::arcade::racer::track::{Car, Divider, Lane, Object, Scenery, Shoulder, Shoulders};

// ─── Car shapes ──────────────────────────────────────────────────────────────

pub const CAR_SEDAN:    Car = Car { height: 3,  incidence: 0.45 };
pub const CAR_HATCHBACK:Car = Car { height: 3,  incidence: 0.25 };
pub const CAR_VAN:      Car = Car { height: 5,  incidence: 0.20 };
pub const CAR_TRUCK:    Car = Car { height: 7,  incidence: 0.08 };
pub const CAR_SEMI:     Car = Car { height: 11, incidence: 0.02 };

pub const CITY_CAR_MIX:    &[Car] = &[CAR_SEDAN, CAR_HATCHBACK, CAR_VAN];
pub const HIGHWAY_CAR_MIX: &[Car] = &[CAR_SEDAN, CAR_HATCHBACK, CAR_VAN, CAR_TRUCK, CAR_SEMI];
pub const RURAL_CAR_MIX:   &[Car] = &[CAR_HATCHBACK, CAR_VAN, CAR_TRUCK];

// ─── Lane templates ──────────────────────────────────────────────────────────

pub const CITY_LANE: Lane = Lane {
    style: theme::LANE_ASPHALT_STANDARD,
    own_min_speed: 0.0,
    own_max_speed: 150.0,
    passive_decel: 0.0,
    traffic_min_speed: 40.0,
    traffic_max_speed: 90.0,
    traffic_density: 0.4,
    traffic_cars: CITY_CAR_MIX,
    obstacles: &[],
};

pub const HIGHWAY_LANE: Lane = Lane {
    style: theme::LANE_ASPHALT_PREMIUM,
    own_min_speed: 50.0,
    own_max_speed: 260.0,
    passive_decel: 0.0,
    traffic_min_speed: 60.0,
    traffic_max_speed: 150.0,
    traffic_density: 0.25,
    traffic_cars: HIGHWAY_CAR_MIX,
    obstacles: &[],
};

pub const RURAL_LANE: Lane = Lane {
    style: theme::LANE_ASPHALT_PATCHY,
    own_min_speed: 0.0,
    own_max_speed: 100.0,
    passive_decel: 2.0,
    traffic_min_speed: 30.0,
    traffic_max_speed: 80.0,
    traffic_density: 0.2,
    traffic_cars: RURAL_CAR_MIX,
    obstacles: &[],
};

pub const FOREST_LANE: Lane = Lane {
    style: theme::LANE_DIRT,
    own_min_speed: 0.0,
    own_max_speed: 70.0,
    passive_decel: 4.0,
    traffic_min_speed: 20.0,
    traffic_max_speed: 50.0,
    traffic_density: 0.1,
    traffic_cars: RURAL_CAR_MIX,
    obstacles: &[],
};

// ─── Dividers ────────────────────────────────────────────────────────────────

pub const URBAN_DIVIDERS: Divider = Divider {
    primary: theme::DIV_YELLOW_DOUBLE,
    lane: theme::DIV_WHITE_DASH,
};
pub const HIGHWAY_DIVIDERS: Divider = Divider {
    primary: theme::DIV_YELLOW_DOUBLE,
    lane: theme::DIV_WHITE_DASH,
};
pub const RURAL_DIVIDERS: Divider = Divider {
    primary: theme::DIV_YELLOW_DASH,
    lane: theme::DIV_FAINT,
};
pub const FOREST_DIVIDERS: Divider = Divider {
    primary: theme::DIV_FAINT,
    lane: theme::DIV_NONE,
};

// ─── Scenery objects ─────────────────────────────────────────────────────────

pub const CITY_OBJECTS: &[Object] = &[
    Object { style: theme::OBJ_BUILDING_HOUSE,       incidence: 0.35 },
    Object { style: theme::OBJ_BUILDING_APARTMENTS,  incidence: 0.25 },
    Object { style: theme::OBJ_SKYSCRAPER,           incidence: 0.10 },
    Object { style: theme::OBJ_TREE_OAK,             incidence: 0.20 },
    Object { style: theme::OBJ_BUSH,                 incidence: 0.10 },
];

pub const HIGHWAY_OBJECTS: &[Object] = &[
    Object { style: theme::OBJ_TREE_PINE,  incidence: 0.50 },
    Object { style: theme::OBJ_TREE_OAK,  incidence: 0.25 },
    Object { style: theme::OBJ_BUSH,      incidence: 0.20 },
    Object { style: theme::OBJ_FLOWER,    incidence: 0.05 },
];

pub const RURAL_OBJECTS: &[Object] = &[
    Object { style: theme::OBJ_TREE_OAK, incidence: 0.45 },
    Object { style: theme::OBJ_BUSH,     incidence: 0.25 },
    Object { style: theme::OBJ_GRASS,    incidence: 0.20 },
    Object { style: theme::OBJ_FLOWER,   incidence: 0.10 },
];

pub const FOREST_OBJECTS: &[Object] = &[
    Object { style: theme::OBJ_TREE_PINE, incidence: 0.80 },
    Object { style: theme::OBJ_BUSH,      incidence: 0.20 },
];

pub const DESERT_OBJECTS: &[Object] = &[
    Object { style: theme::OBJ_GRASS,     incidence: 0.40 },
    Object { style: theme::OBJ_BUSH,      incidence: 0.40 },
    Object { style: theme::OBJ_TREE_PALM, incidence: 0.20 },
];

// ─── Sceneries ───────────────────────────────────────────────────────────────

pub const CITY_SCENERY: Scenery = Scenery {
    width: 14,
    background: theme::SCENERY_CONCRETE,
    objects: CITY_OBJECTS,
};
pub const HIGHWAY_SCENERY: Scenery = Scenery {
    width: 12,
    background: theme::SCENERY_GRASS,
    objects: HIGHWAY_OBJECTS,
};
pub const RURAL_SCENERY: Scenery = Scenery {
    width: 10,
    background: theme::SCENERY_GRASS,
    objects: RURAL_OBJECTS,
};
pub const FOREST_SCENERY: Scenery = Scenery {
    width: 14,
    background: theme::SCENERY_DIRT,
    objects: FOREST_OBJECTS,
};
pub const DESERT_SCENERY: Scenery = Scenery {
    width: 12,
    background: theme::SCENERY_DIRT,
    objects: DESERT_OBJECTS,
};

// ─── Shoulders ───────────────────────────────────────────────────────────────

pub const SIDEWALK_SHOULDERS: &[Shoulder] = &[
    Shoulder { style: theme::SHOULDER_HARD_EDGE, repeat: 0 },
    Shoulder { style: theme::SHOULDER_SIDEWALK,  repeat: 0 },
];

pub const HIGHWAY_SHOULDERS: &[Shoulder] = &[
    Shoulder { style: theme::SHOULDER_HARD_EDGE, repeat: 0 },
    Shoulder { style: theme::SHOULDER_POLES,     repeat: 6 },
];

pub const RURAL_SHOULDERS: &[Shoulder] = &[
    Shoulder { style: theme::SHOULDER_SOFT_EDGE, repeat: 0 },
];

pub const FOREST_SHOULDERS: &[Shoulder] = &[
    Shoulder { style: theme::SHOULDER_SOFT_EDGE,  repeat: 0 },
    Shoulder { style: theme::SHOULDER_TREE_PINE,  repeat: 4 },
];

pub const NO_SHOULDERS: Shoulders = Shoulders { left: &[], right: &[] };
