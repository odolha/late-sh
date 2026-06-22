use super::presets::*;
use crate::app::arcade::racer::track::{Lane, LaneAspect, Lanes, Obstacle, ObstacleAspect, ObstacleEffect, Road, RoadAspect, Sceneries, Shoulders, Stage, StageIcon, Theme, Track};

// presets
pub const CLUJ_LANE: Lane = Lane {
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

pub const TRACK: Track = Track {
    name: "Batin",
    author: "odd",
    description: "A road trip I used to make when I was a kid. From the city to our countryside house. Starts decently and ends with extremely rugged roads.",
    stages: &[
        Stage {
            name: "Cluj-Napoca",
            description: "Got our Dacia 1310 ready to go - washed and filled up. Car is full of bags of things we'll mostly bring back intact. 3 kids in the car. Let's go.",
            icon: StageIcon::City,
            theme: Theme::Standard,
            distance_km: 7.0,
            road: Road {
                aspect: RoadAspect { dividers: URBAN_DIVIDERS },
                lanes: Lanes {
                    incoming: &[CLUJ_LANE, CLUJ_LANE],
                    outgoing: &[CLUJ_LANE, CLUJ_LANE],
                },
                sceneries: Sceneries { left: CITY_SCENERY, right: CITY_SCENERY },
                shoulders: Shoulders { left: SIDEWALK_SHOULDERS, right: SIDEWALK_SHOULDERS },
            },
        },
    ],
    distance_scale: 0.2,
    speed_scale: 2.0,
};
