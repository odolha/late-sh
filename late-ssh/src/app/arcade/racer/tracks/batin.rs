use crate::app::arcade::racer::theme;
use super::presets::*;
use crate::app::arcade::racer::track::{Lane, Lanes, Obstacle, ObstacleEffect, Road, RoadAspect, Sceneries, Shoulder, Shoulders, Stage, Theme, Track};

// PRESETS

const CLUJ_LANE: Lane = Lane {
    own_max_speed: 120.0,
    obstacles: &[
        Obstacle {
            style: theme::OBSTACLE_POTHOLE_SMALL,
            frequency: 0.02,
            effects: &[ObstacleEffect::SpeedChange { affect: -0.3 }],
        }
    ],
    ..CITY_LANE
};

const CLUJ_LANE_OUT: Lane = Lane {
    traffic_density: 0.4,
    ..CLUJ_LANE
};

const CLUJ_LANE_IN: Lane = Lane {
    own_max_speed: 150.0,
    traffic_density: 0.2,
    ..CLUJ_LANE
};

const OUTSKIRT_LANE: Lane = Lane {
    own_max_speed: 100.0,
    obstacles: &[
        Obstacle {
            style: theme::OBSTACLE_POTHOLE_BIG,
            frequency: 0.02,
            effects: &[ObstacleEffect::SpeedChange { affect: -0.6 }],
        },
        Obstacle {
            style: theme::OBSTACLE_POTHOLE_SMALL,
            frequency: 0.03,
            effects: &[ObstacleEffect::SpeedChange { affect: -0.3 }],
        }
    ],
    ..CITY_LANE
};

const OUTSKIRT_LANE_OUT: Lane = Lane {
    traffic_density: 0.3,
    ..OUTSKIRT_LANE
};

const OUTSKIRT_LANE_IN: Lane = Lane {
    own_max_speed: 120.0,
    traffic_density: 0.15,
    ..OUTSKIRT_LANE
};

// S01: Cluj-Napoca

const S01_CLUJ: Stage = Stage {
    name: "Cluj-Napoca",
    description: "My city of birth. Year 1998. Got our Dacia 1310 ready to go - washed and filled up. Car is full of bags of things we'll mostly bring back intact. 3 kids in the car. Let's go.",
    icon: theme::STAGE_CITY,
    theme: Theme::Standard,
    distance_km: 8.0,
    road: Road {
        aspect: RoadAspect { dividers: URBAN_DIVIDERS },
        lanes: Lanes {
            outgoing: &[CLUJ_LANE_OUT, CLUJ_LANE_OUT],
            incoming: &[CLUJ_LANE_IN, CLUJ_LANE_IN],
        },
        sceneries: Sceneries { left: CITY_SCENERY, right: CITY_SCENERY },
        shoulders: Shoulders { left: SIDEWALK_SHOULDERS, right: SIDEWALK_SHOULDERS },
    },
};

// S02: Sannicoara

const S02_SANNICOARA: Stage = Stage {
    name: "Sannicoara",
    description: "Moving out of the city.",
    icon: theme::STAGE_CITY_OUTSKIRTS,
    theme: Theme::Standard,
    distance_km: 2.0,
    road: Road {
        aspect: RoadAspect { dividers: URBAN_DIVIDERS },
        lanes: Lanes {
            outgoing: &[OUTSKIRT_LANE_OUT],
            incoming: &[OUTSKIRT_LANE_IN, OUTSKIRT_LANE_IN],
        },
        sceneries: Sceneries { left: CITY_SCENERY, right: HIGHWAY_SCENERY },
        shoulders: Shoulders {
            left: &[
                Shoulder { style: theme::SHOULDER_SOFT_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_SIDEWALK,  repeat: 0 },
            ],
            right: &[
                Shoulder { style: theme::SHOULDER_SOFT_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
                Shoulder { style: theme::SHOULDER_POLES,  repeat: 20 },
            ]
        },
    },
};

// S03: Apahida

const S03_APAHIDA: Stage = Stage {
    name: "Apahida",
    description: "This is a town close to the main city (Cluj). I live here now :)",
    icon: theme::STAGE_CITY_OUTSKIRTS,
    theme: Theme::Standard,
    distance_km: 3.0,
    road: Road {
        aspect: RoadAspect { dividers: URBAN_DIVIDERS },
        lanes: Lanes {
            outgoing: &[OUTSKIRT_LANE_OUT],
            incoming: &[OUTSKIRT_LANE_IN],
        },
        sceneries: Sceneries { left: CITY_SCENERY, right: HIGHWAY_SCENERY },
        shoulders: Shoulders {
            left: &[
                Shoulder { style: theme::SHOULDER_SOFT_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_SIDEWALK,  repeat: 0 },
            ],
            right: &[
                Shoulder { style: theme::SHOULDER_SOFT_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
                Shoulder { style: theme::SHOULDER_RAILROAD,  repeat: 20 },
            ]
        },
    },
};

pub const TRACK: Track = Track {
    name: "Batin",
    author: "odd",
    description: "A road trip I used to make when I was a kid. From the city to our countryside house. Starts decently and ends with extremely rugged roads.",
    stages: &[
        // S01_CLUJ,
        S02_SANNICOARA,
        S03_APAHIDA
    ],
    distance_scale: 0.5,
    speed_scale: 2.0,
};
