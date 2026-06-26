use crate::app::arcade::racer::theme;
use super::presets::*;
use crate::app::arcade::racer::track::{Lane, Lanes, Obstacle, ObstacleEffect, Road, RoadAspect, Sceneries, Shoulder, Shoulders, Stage, Theme, Track};

// PRESETS

const MY_CITY_LANE: Lane = Lane {
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

const MY_CITY_LANE_OUT: Lane = Lane {
    traffic_density: 0.4,
    ..MY_CITY_LANE
};

const MY_CITY_LANE_IN: Lane = Lane {
    own_max_speed: 150.0,
    traffic_density: 0.2,
    ..MY_CITY_LANE
};

const MY_TOWN_LANE: Lane = Lane {
    own_max_speed: 100.0,
    traffic_max_speed: 80.0,
    obstacles: &[
        Obstacle {
            style: theme::OBSTACLE_POTHOLE_SMALL,
            frequency: 0.02,
            effects: &[ObstacleEffect::SpeedChange { affect: -0.3 }],
        },
        Obstacle {
            style: theme::OBSTACLE_POTHOLE_BIG,
            frequency: 0.01,
            effects: &[ObstacleEffect::SpeedChange { affect: -0.6 }],
        },
        Obstacle {
            style: theme::OBSTACLE_SPEED_BUMP,
            frequency: 0.01,
            effects: &[ObstacleEffect::SpeedChange { affect: -0.8 }],
        }
    ],
    ..CITY_LANE
};

const MY_TOWN_LANE_OUT: Lane = Lane {
    traffic_density: 0.6,
    ..MY_TOWN_LANE
};

const MY_TOWN_LANE_IN: Lane = Lane {
    own_max_speed: 140.0,
    traffic_density: 0.4,
    ..MY_TOWN_LANE
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

const OUTSIDE_LANE: Lane = Lane {
    own_max_speed: 150.0,
    obstacles: &[
        Obstacle {
            style: theme::OBSTACLE_POTHOLE_CRATER,
            frequency: 0.005,
            effects: &[ObstacleEffect::SpeedChange { affect: -0.9 }],
        },
        Obstacle {
            style: theme::OBSTACLE_POTHOLE_BIG,
            frequency: 0.02,
            effects: &[ObstacleEffect::SpeedChange { affect: -0.6 }],
        }
    ],
    ..CITY_LANE
};

const OUTSIDE_LANE_OUT: Lane = Lane {
    traffic_density: 0.35,
    ..OUTSIDE_LANE
};

const OUTSIDE_LANE_OUT_SLOW: Lane = Lane {
    traffic_density: 0.15,
    own_max_speed: 100.0,
    traffic_max_speed: 60.0,
    style: theme::LANE_ASPHALT_PATCHY,
    ..OUTSIDE_LANE
};

const OUTSIDE_LANE_IN: Lane = Lane {
    own_max_speed: 200.0,
    traffic_max_speed: 110.0,
    traffic_density: 0.15,
    ..OUTSIDE_LANE
};

const OUTSIDE_LANE_IN_SLOW: Lane = Lane {
    own_max_speed: 140.0,
    traffic_max_speed: 100.0,
    traffic_density: 0.15,
    style: theme::LANE_ASPHALT_PATCHY,
    ..OUTSIDE_LANE
};

// STAGES

const S01_CLUJ: Stage = Stage {
    name: "Cluj-Napoca",
    description: "My city of birth. Year 1998. Got our Dacia 1310 ready to go - washed and filled up. Car is full of bags of things we'll mostly bring back intact. 3 kids in the car. Let's go.",
    icon: theme::STAGE_CITY,
    theme: Theme::Standard,
    distance_km: 8.0,
    road: Road {
        aspect: RoadAspect { dividers: URBAN_DIVIDERS },
        lanes: Lanes {
            outgoing: &[MY_CITY_LANE_OUT, MY_CITY_LANE_OUT],
            incoming: &[MY_CITY_LANE_IN, MY_CITY_LANE_IN],
        },
        sceneries: Sceneries {
            left: CITY_SCENERY,
            right: CITY_SCENERY
        },
        shoulders: Shoulders {
            left: &[
                Shoulder { style: theme::SHOULDER_HARD_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_SIDEWALK,  repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
            ],
            right: &[
                Shoulder { style: theme::SHOULDER_HARD_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_SIDEWALK,  repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
            ]
        },
    },
};

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
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
            ],
            right: &[
                Shoulder { style: theme::SHOULDER_SOFT_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
                Shoulder { style: theme::SHOULDER_POLES,  repeat: 20 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
            ]
        },
    },
};

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
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
            ],
            right: &[
                Shoulder { style: theme::SHOULDER_SOFT_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
                Shoulder { style: theme::SHOULDER_RAILROAD,  repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
                Shoulder { style: theme::SHOULDER_TREE_OAK, repeat: 1 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
            ]
        },
    },
};

const S04_JUCU: Stage = Stage {
    name: "Jucu",
    description: "Jucu is actually a group of 3 villages. We're moving alongside all of them, not really through.",
    icon: theme::STAGE_WILD_PLAINS,
    theme: Theme::Standard,
    distance_km: 4.0,
    road: Road {
        aspect: RoadAspect { dividers: HIGHWAY_DIVIDERS },
        lanes: Lanes {
            outgoing: &[OUTSIDE_LANE_OUT, OUTSIDE_LANE_OUT, OUTSIDE_LANE_OUT_SLOW],
            incoming: &[OUTSIDE_LANE_IN, OUTSIDE_LANE_IN],
        },
        sceneries: Sceneries { left: RURAL_SCENERY, right: RURAL_SCENERY },
        shoulders: Shoulders {
            left: &[
                Shoulder { style: theme::SHOULDER_SOFT_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
            ],
            right: &[
                Shoulder { style: theme::SHOULDER_SOFT_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
                Shoulder { style: theme::SHOULDER_RAILROAD,  repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
            ]
        },
    },
};

const S05_E576_1: Stage = Stage {
    name: "E576/1",
    description: "Rural area begins.",
    icon: theme::STAGE_WILD_PLAINS,
    theme: Theme::Standard,
    distance_km: 7.0,
    road: Road {
        aspect: RoadAspect { dividers: HIGHWAY_DIVIDERS },
        lanes: Lanes {
            outgoing: &[OUTSIDE_LANE_OUT, OUTSIDE_LANE_OUT_SLOW],
            incoming: &[OUTSIDE_LANE_IN, OUTSIDE_LANE_IN_SLOW],
        },
        sceneries: Sceneries { left: RURAL_SCENERY, right: RURAL_SCENERY },
        shoulders: Shoulders {
            left: &[
                Shoulder { style: theme::SHOULDER_SOFT_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
            ],
            right: &[
                Shoulder { style: theme::SHOULDER_SOFT_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
                Shoulder { style: theme::SHOULDER_RAILROAD,  repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
            ]
        },
    },
};

const S06_RASCRUCI: Stage = Stage {
    name: "Rascruci",
    description: "Boring village.",
    icon: theme::STAGE_WILD_PLAINS,
    theme: Theme::Standard,
    distance_km: 2.0,
    road: Road {
        aspect: RoadAspect { dividers: RURAL_DIVIDERS },
        lanes: Lanes {
            outgoing: &[OUTSIDE_LANE_OUT],
            incoming: &[OUTSIDE_LANE_IN],
        },
        sceneries: Sceneries { left: RURAL_SCENERY, right: RURAL_SCENERY },
        shoulders: Shoulders {
            left: &[
                Shoulder { style: theme::SHOULDER_SOFT_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
            ],
            right: &[
                Shoulder { style: theme::SHOULDER_SOFT_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
                Shoulder { style: theme::SHOULDER_RAILROAD,  repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
                Shoulder { style: theme::SHOULDER_RIVER,  repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
            ]
        },
    },
};

const S07_BONTIDA: Stage = Stage {
    name: "Bontida",
    description: "Another boring village.",
    icon: theme::STAGE_WILD_PLAINS,
    theme: Theme::Standard,
    distance_km: 2.0,
    road: Road {
        aspect: RoadAspect { dividers: RURAL_DIVIDERS },
        lanes: Lanes {
            outgoing: &[OUTSIDE_LANE_OUT],
            incoming: &[OUTSIDE_LANE_IN],
        },
        sceneries: Sceneries { left: RURAL_SCENERY, right: RURAL_SCENERY },
        shoulders: Shoulders {
            left: &[
                Shoulder { style: theme::SHOULDER_SOFT_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
            ],
            right: &[
                Shoulder { style: theme::SHOULDER_SOFT_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
                Shoulder { style: theme::SHOULDER_RAILROAD,  repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
            ]
        },
    },
};

const S08_FUNDATURA: Stage = Stage {
    name: "Fundatura",
    description: "Yet another boring village.",
    icon: theme::STAGE_WILD_PLAINS,
    theme: Theme::Standard,
    distance_km: 2.0,
    road: Road {
        aspect: RoadAspect { dividers: RURAL_DIVIDERS },
        lanes: Lanes {
            outgoing: &[OUTSIDE_LANE_OUT],
            incoming: &[OUTSIDE_LANE_IN],
        },
        sceneries: Sceneries { left: RURAL_SCENERY, right: RURAL_SCENERY },
        shoulders: Shoulders {
            left: &[
                Shoulder { style: theme::SHOULDER_SOFT_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
            ],
            right: &[
                Shoulder { style: theme::SHOULDER_SOFT_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
                Shoulder { style: theme::SHOULDER_RAILROAD,  repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
                Shoulder { style: theme::SHOULDER_RIVER,  repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
            ]
        },
    },
};

const S09_ICLOD: Stage = Stage {
    name: "Iclod",
    description: "Boring village, again...",
    icon: theme::STAGE_VILLAGE,
    theme: Theme::Standard,
    distance_km: 2.0,
    road: Road {
        aspect: RoadAspect { dividers: RURAL_DIVIDERS },
        lanes: Lanes {
            outgoing: &[OUTSIDE_LANE_OUT, OUTSIDE_LANE_OUT_SLOW],
            incoming: &[OUTSIDE_LANE_IN],
        },
        sceneries: Sceneries { left: VILLAGE_SCENERY, right: VILLAGE_SCENERY },
        shoulders: Shoulders {
            left: &[
                Shoulder { style: theme::SHOULDER_SOFT_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
            ],
            right: &[
                Shoulder { style: theme::SHOULDER_SOFT_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
                Shoulder { style: theme::SHOULDER_RAILROAD,  repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
                Shoulder { style: theme::SHOULDER_RIVER,  repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
            ]
        },
    },
};

const S10_LIVADA: Stage = Stage {
    name: "Livada",
    description: "Aaaand... it's another boring village.",
    icon: theme::STAGE_VILLAGE,
    theme: Theme::Standard,
    distance_km: 2.0,
    road: Road {
        aspect: RoadAspect { dividers: RURAL_DIVIDERS },
        lanes: Lanes {
            outgoing: &[OUTSIDE_LANE_OUT, OUTSIDE_LANE_OUT_SLOW],
            incoming: &[OUTSIDE_LANE_IN],
        },
        sceneries: Sceneries { left: RURAL_SCENERY, right: VILLAGE_SCENERY },
        shoulders: Shoulders {
            left: &[
                Shoulder { style: theme::SHOULDER_SOFT_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
            ],
            right: &[
                Shoulder { style: theme::SHOULDER_SOFT_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
                Shoulder { style: theme::SHOULDER_RAILROAD,  repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
                Shoulder { style: theme::SHOULDER_RIVER,  repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
            ]
        },
    },
};

const S11_BAITA: Stage = Stage {
    name: "Baita",
    description: "Coming up to something else...",
    icon: theme::STAGE_WILD_PLAINS,
    theme: Theme::Standard,
    distance_km: 2.0,
    road: Road {
        aspect: RoadAspect { dividers: RURAL_DIVIDERS },
        lanes: Lanes {
            outgoing: &[OUTSIDE_LANE_OUT],
            incoming: &[OUTSIDE_LANE_IN],
        },
        sceneries: Sceneries { left: RURAL_SCENERY, right: RURAL_SCENERY },
        shoulders: Shoulders {
            left: &[
                Shoulder { style: theme::SHOULDER_SOFT_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
            ],
            right: &[
                Shoulder { style: theme::SHOULDER_SOFT_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
                Shoulder { style: theme::SHOULDER_RAILROAD,  repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
                Shoulder { style: theme::SHOULDER_RIVER,  repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
            ]
        },
    },
};


const S12_GHERLA: Stage = Stage {
    name: "Gherla",
    description: "Hey, a town! It is pretty nice. There's also a prison nearby.",
    icon: theme::STAGE_CITY_OUTSKIRTS,
    theme: Theme::Standard,
    distance_km: 5.0,
    road: Road {
        aspect: RoadAspect { dividers: URBAN_DIVIDERS },
        lanes: Lanes {
            outgoing: &[MY_TOWN_LANE_OUT, MY_TOWN_LANE_OUT],
            incoming: &[MY_TOWN_LANE_IN, MY_TOWN_LANE_IN],
        },
        sceneries: Sceneries { left: TOWN_SCENERY, right: TOWN_SCENERY },
        shoulders: Shoulders {
            left: &[
                Shoulder { style: theme::SHOULDER_HARD_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
                Shoulder { style: theme::SHOULDER_PARKED_CAR, repeat: 4 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
            ],
            right: &[
                Shoulder { style: theme::SHOULDER_HARD_EDGE, repeat: 0 },
                Shoulder { style: theme::SHOULDER_SIDEWALK, repeat: 0 },
                Shoulder { style: theme::SHOULDER_EMPTY, repeat: 0 },
            ]
        },
    },
};

// TRACK

pub const TRACK: Track = Track {
    name: "Batin",
    author: "odd",
    description: "A road trip I used to make when I was a kid. From the city to our countryside house. Starts decently and ends with extremely rugged roads.",
    stages: &[
        S01_CLUJ,
        S02_SANNICOARA,
        S03_APAHIDA,
        S04_JUCU,
        S05_E576_1,
        S06_RASCRUCI,
        S07_BONTIDA,
        S08_FUNDATURA,
        S09_ICLOD,
        S10_LIVADA,
        S11_BAITA,
        S12_GHERLA,
    ],
    distance_scale: 0.2,
    speed_scale: 2.0,
};
