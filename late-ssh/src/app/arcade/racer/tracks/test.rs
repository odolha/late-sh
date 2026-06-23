use super::presets::*;
use crate::app::arcade::racer::track::{Car, Lane, LaneAspect, Lanes, Object, ObjectAspect, Obstacle, ObstacleAspect, ObstacleEffect, Road, RoadAspect, Sceneries, Scenery, SceneryBackground, Shoulder, ShoulderAspect, Shoulders, Stage, StageIcon, Theme, Track};

// presets
pub const TEST_LANE: Lane = Lane {
    aspect: LaneAspect::AsphaltStandard,
    own_min_speed: 0.0,
    own_max_speed: 150.0,
    passive_decel: 0.0,
    traffic_min_speed: 40.0,
    traffic_max_speed: 90.0,
    traffic_size: 12,
    obstacles: &[
        Obstacle {
            aspect: ObstacleAspect::PotholeSmall,
            frequency: 0.2,
            effects: &[
                ObstacleEffect::SpeedChange {
                    affect: -0.2
                }
            ]
        },
        Obstacle {
            aspect: ObstacleAspect::PotholeBig,
            frequency: 0.1,
            effects: &[
                ObstacleEffect::SpeedChange {
                    affect: -0.5
                }
            ]
        },
        Obstacle {
            aspect: ObstacleAspect::SpeedBump,
            frequency: 0.01,
            effects: &[
                ObstacleEffect::SpeedChange {
                    affect: -0.5
                }
            ]
        },
        Obstacle {
            aspect: ObstacleAspect::PotholeCrater,
            frequency: 0.05,
            effects: &[
                ObstacleEffect::SpeedChange {
                    affect: -0.9
                }
            ]
        },
        Obstacle {
            aspect: ObstacleAspect::Spikes,
            frequency: 0.01,
            effects: &[
                ObstacleEffect::Crash
            ]
        },
        Obstacle {
            aspect: ObstacleAspect::FallenTree,
            frequency: 0.01,
            effects: &[
                ObstacleEffect::Crash
            ]
        },
    ],
    traffic_cars: &[
        Car { height: 3, incidence: 0.4 },
        Car { height: 5, incidence: 0.4 },
        Car { height: 1, incidence: 0.1 },
        Car { height: 10, incidence: 0.1 }
    ],
};

pub const TRACK: Track = Track {
    name: "Test",
    author: "odd",
    description: "Test track",
    stages: &[
        Stage {
            name: "Stage0",
            description: "This is stage0 test.",
            icon: StageIcon::City,
            theme: Theme::Desert,
            distance_km: 20.0,
            road: Road {
                aspect: RoadAspect { dividers: URBAN_DIVIDERS },
                lanes: Lanes {
                    incoming: &[TEST_LANE, TEST_LANE],
                    outgoing: &[TEST_LANE, TEST_LANE],
                },
                sceneries: Sceneries {
                    left: Scenery {
                        width: 16,
                        background: SceneryBackground::Concrete,
                        objects: &[
                            Object { aspect: ObjectAspect::BuildingHouse, incidence: 1.0 },
                        ],
                    },
                    right: Scenery {
                        width: 16,
                        background: SceneryBackground::Concrete,
                        objects: &[
                            Object { aspect: ObjectAspect::BuildingHouse, incidence: 1.0 },
                        ],
                    }
                },
                shoulders: Shoulders {
                    left: &[
                        Shoulder { aspect: ShoulderAspect::SoftEdge, repeat: 0 },
                        Shoulder { aspect: ShoulderAspect::Empty, repeat: 0 },
                        Shoulder { aspect: ShoulderAspect::CountryRoad, repeat: 0 },
                        Shoulder { aspect: ShoulderAspect::Empty, repeat: 0 },
                        Shoulder { aspect: ShoulderAspect::TreePalm, repeat: 10 },
                        Shoulder { aspect: ShoulderAspect::Empty, repeat: 0 },
                    ],
                    right: &[
                        Shoulder { aspect: ShoulderAspect::SoftEdge, repeat: 0 },
                        Shoulder { aspect: ShoulderAspect::Empty, repeat: 0 },
                        Shoulder { aspect: ShoulderAspect::River, repeat: 0 },
                        Shoulder { aspect: ShoulderAspect::Empty, repeat: 0 },
                        Shoulder { aspect: ShoulderAspect::TreePine, repeat: 10 },
                        Shoulder { aspect: ShoulderAspect::Empty, repeat: 0 },
                    ]
                },
            },
        },
    ],
    distance_scale: 0.5,
    speed_scale: 1.0,
};
