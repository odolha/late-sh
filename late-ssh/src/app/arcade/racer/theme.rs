//! Standard style library for the Racer game.
//!
//! Every `pub const` in this module is a ready-made instance of one of the
//! style descriptor types defined in `track.rs`.  Track authors import these by
//! name and compose them via struct-update syntax or inline overrides.
//!
//! Adding a **new standard look** means adding a constant here.
//! Adding a **new [`Theme`] variant** means adding a branch to `tint` and to
//! `Theme::name` / `Theme::icon` in `track.rs`.
//!
//! Nothing in this file is mandatory.  A track can define its own style
//! instances inline without touching this file at all.

use ratatui::style::Color;

use super::track::{
    Cell, DividerStyle, LaneStyle, ObjectStyle, ObstacleStyle, SceneryStyle,
    ShoulderStyle, Sprite, Theme,
};

// ─── Theme tint ──────────────────────────────────────────────────────────────

/// Multiplicative colour tint for the active theme.  Applied inside each style
/// fn so callers never have to think about it.
pub fn tint(c: Color, theme: Theme) -> Color {
    let Color::Rgb(r, g, b) = c else { return c; };
    match theme {
        Theme::Standard => c,
        Theme::Winter => Color::Rgb(
            ((r as u16 + 60).min(255)) as u8,
            ((g as u16 + 60).min(255)) as u8,
            ((b as u16 + 80).min(255)) as u8,
        ),
        Theme::Desert => Color::Rgb(
            ((r as u16 * 11 / 10).min(255)) as u8,
            ((g as u16 *  9 / 10).min(255)) as u8,
            ((b as u16 *  7 / 10).min(255)) as u8,
        ),
    }
}

// ─── Trunk colour (shared by tree-bearing ObjectStyles) ─────────────────────

const TRUNK_BROWN: Color = Color::Rgb(110, 70, 30);

pub fn trunk_color(theme: Theme) -> Color {
    tint(TRUNK_BROWN, theme)
}

// ─── Lane styles ─────────────────────────────────────────────────────────────

const ASPHALT_BG: Color          = Color::Rgb(18, 18, 18);
const ASPHALT_PREMIUM_BG: Color  = Color::Rgb(28, 28, 32);
const ASPHALT_PATCHY_BG: Color   = Color::Rgb(22, 20, 18);
const ASPHALT_PATCH_FG: Color    = Color::Rgb(70, 65, 60);
const GRAVEL_BG: Color           = Color::Rgb(70, 60, 48);
const GRAVEL_FG: Color           = Color::Rgb(120, 105, 90);
const DIRT_BG: Color             = Color::Rgb(82, 56, 36);
const DIRT_FG: Color             = Color::Rgb(120, 90, 60);
const LANE_GRASS_BG: Color       = Color::Rgb(28, 60, 28);
const LANE_GRASS_FG: Color       = Color::Rgb(70, 110, 60);

pub const LANE_ASPHALT_PREMIUM: LaneStyle = LaneStyle {
    cell: |theme, _row, _col| {
        let bg = tint(ASPHALT_PREMIUM_BG, theme);
        Cell::new(" ", bg, bg)
    },
    bg: |theme| tint(ASPHALT_PREMIUM_BG, theme),
};

pub const LANE_ASPHALT_STANDARD: LaneStyle = LaneStyle {
    cell: |theme, _row, _col| {
        let bg = tint(ASPHALT_BG, theme);
        Cell::new(" ", bg, bg)
    },
    bg: |theme| tint(ASPHALT_BG, theme),
};

pub const LANE_ASPHALT_PATCHY: LaneStyle = LaneStyle {
    cell: |theme, row, _col| {
        let bg = tint(ASPHALT_PATCHY_BG, theme);
        if row.rem_euclid(7) == 0 {
            Cell::new(".", tint(ASPHALT_PATCH_FG, theme), bg)
        } else {
            Cell::new(" ", bg, bg)
        }
    },
    bg: |theme| tint(ASPHALT_PATCHY_BG, theme),
};

pub const LANE_GRAVEL: LaneStyle = LaneStyle {
    cell: |theme, row, _col| {
        let bg = tint(GRAVEL_BG, theme);
        if row.rem_euclid(3) == 0 {
            Cell::new(",", tint(GRAVEL_FG, theme), bg)
        } else {
            Cell::new(" ", bg, bg)
        }
    },
    bg: |theme| tint(GRAVEL_BG, theme),
};

pub const LANE_DIRT: LaneStyle = LaneStyle {
    cell: |theme, row, _col| {
        let bg = tint(DIRT_BG, theme);
        if row.rem_euclid(5) < 2 {
            Cell::new("·", tint(DIRT_FG, theme), bg)
        } else {
            Cell::new(" ", bg, bg)
        }
    },
    bg: |theme| tint(DIRT_BG, theme),
};

pub const LANE_GRASS: LaneStyle = LaneStyle {
    cell: |theme, row, _col| {
        let bg = tint(LANE_GRASS_BG, theme);
        if row.rem_euclid(6) == 0 {
            Cell::new("v", tint(LANE_GRASS_FG, theme), bg)
        } else {
            Cell::new(" ", bg, bg)
        }
    },
    bg: |theme| tint(LANE_GRASS_BG, theme),
};

// ─── Divider styles ──────────────────────────────────────────────────────────

const YELLOW: Color = Color::Rgb(220, 180, 30);
const WHITE:  Color = Color::Rgb(180, 180, 180);
const FAINT:  Color = Color::Rgb(85, 85, 85);

pub const DIV_YELLOW_DOUBLE: DividerStyle = DividerStyle {
    cell: |_row, bg| Cell::new("‖", YELLOW, bg),
};
pub const DIV_YELLOW_SINGLE: DividerStyle = DividerStyle {
    cell: |_row, bg| Cell::new("│", YELLOW, bg),
};
pub const DIV_YELLOW_DASH: DividerStyle = DividerStyle {
    cell: |row, bg| {
        let sym = if row.rem_euclid(4) < 2 { "│" } else { " " };
        Cell::new(sym, YELLOW, bg)
    },
};
pub const DIV_YELLOW_DOTS: DividerStyle = DividerStyle {
    cell: |row, bg| {
        let sym = if row.rem_euclid(3) == 0 { "·" } else { " " };
        Cell::new(sym, YELLOW, bg)
    },
};
pub const DIV_WHITE_SINGLE: DividerStyle = DividerStyle {
    cell: |_row, bg| Cell::new("│", WHITE, bg),
};
pub const DIV_WHITE_DASH: DividerStyle = DividerStyle {
    cell: |row, bg| {
        let sym = if row.rem_euclid(4) < 2 { "│" } else { " " };
        Cell::new(sym, WHITE, bg)
    },
};
pub const DIV_WHITE_DOTS: DividerStyle = DividerStyle {
    cell: |row, bg| {
        let sym = if row.rem_euclid(3) == 0 { "·" } else { " " };
        Cell::new(sym, WHITE, bg)
    },
};
pub const DIV_FAINT: DividerStyle = DividerStyle {
    cell: |_row, bg| Cell::new("│", FAINT, bg),
};
pub const DIV_NONE: DividerStyle = DividerStyle {
    cell: |_row, bg| Cell::new(" ", bg, bg),
};

// ─── Scenery background styles ───────────────────────────────────────────────

const CONCRETE_BG:  Color = Color::Rgb(60, 60, 60);
const SC_GRASS_BG:  Color = Color::Rgb(20, 55, 20);
const SC_DIRT_BG:   Color = Color::Rgb(70, 50, 30);
const VOID_BG:      Color = Color::Rgb(2, 2, 6);

pub const SCENERY_CONCRETE: SceneryStyle = SceneryStyle { bg: |theme| tint(CONCRETE_BG, theme) };
pub const SCENERY_GRASS:    SceneryStyle = SceneryStyle { bg: |theme| tint(SC_GRASS_BG, theme) };
pub const SCENERY_DIRT:     SceneryStyle = SceneryStyle { bg: |theme| tint(SC_DIRT_BG, theme) };
pub const SCENERY_VOID:     SceneryStyle = SceneryStyle { bg: |theme| tint(VOID_BG, theme) };

// ─── Object styles ───────────────────────────────────────────────────────────

const TREE_PINE_GREEN:  Color = Color::Rgb(55, 175, 55);
const TREE_OAK_GREEN:   Color = Color::Rgb(80, 160, 70);
const TREE_PALM_GREEN:  Color = Color::Rgb(120, 180, 70);
const FLOWER_PINK:      Color = Color::Rgb(200, 110, 160);
const BUSH_GREEN:       Color = Color::Rgb(70, 130, 60);
const GRASS_BLADE:      Color = Color::Rgb(70, 130, 70);
const BUILDING_GRAY:    Color = Color::Rgb(140, 140, 140);
const STAR_WHITE:       Color = Color::Rgb(220, 220, 220);

pub const OBJ_GRASS: ObjectStyle = ObjectStyle {
    sprite: |theme| Sprite { width: 1, glyphs: &[&["ʷ"]], fg: tint(GRASS_BLADE, theme) },
    has_trunk: false,
};
pub const OBJ_BUSH: ObjectStyle = ObjectStyle {
    sprite: |theme| Sprite { width: 1, glyphs: &[&["❀"]], fg: tint(BUSH_GREEN, theme) },
    has_trunk: false,
};
pub const OBJ_TREE_PINE: ObjectStyle = ObjectStyle {
    sprite: |theme| Sprite {
        width: 1,
        glyphs: &[&["▲"], &["▲"], &["│"]],
        fg: tint(TREE_PINE_GREEN, theme),
    },
    has_trunk: true,
};
pub const OBJ_TREE_OAK: ObjectStyle = ObjectStyle {
    sprite: |theme| Sprite {
        width: 1,
        glyphs: &[&["◉"], &["│"]],
        fg: tint(TREE_OAK_GREEN, theme),
    },
    has_trunk: true,
};
pub const OBJ_TREE_PALM: ObjectStyle = ObjectStyle {
    sprite: |theme| Sprite {
        width: 1,
        glyphs: &[&["✤"], &["│"]],
        fg: tint(TREE_PALM_GREEN, theme),
    },
    has_trunk: true,
};
pub const OBJ_BUILDING_HOUSE: ObjectStyle = ObjectStyle {
    sprite: |theme| Sprite {
        width: 3,
        glyphs: &[
            &["╱", "▔", "╲"],
            &["│", "░", "│"],
        ],
        fg: tint(BUILDING_GRAY, theme),
    },
    has_trunk: false,
};
pub const OBJ_BUILDING_APARTMENTS: ObjectStyle = ObjectStyle {
    sprite: |theme| Sprite {
        width: 3,
        glyphs: &[
            &["│", "▦", "│"],
            &["│", "▦", "│"],
            &["│", "░", "│"],
        ],
        fg: tint(BUILDING_GRAY, theme),
    },
    has_trunk: false,
};
pub const OBJ_SKYSCRAPER: ObjectStyle = ObjectStyle {
    sprite: |theme| Sprite {
        width: 3,
        glyphs: &[
            &[" ", "▲", " "],
            &["│", "▦", "│"],
            &["│", "▦", "│"],
            &["│", "▦", "│"],
            &["│", "▦", "│"],
            &["│", "░", "│"],
        ],
        fg: tint(BUILDING_GRAY, theme),
    },
    has_trunk: false,
};
pub const OBJ_FLOWER: ObjectStyle = ObjectStyle {
    sprite: |theme| Sprite { width: 1, glyphs: &[&["✿"]], fg: tint(FLOWER_PINK, theme) },
    has_trunk: false,
};
pub const OBJ_STAR: ObjectStyle = ObjectStyle {
    sprite: |theme| Sprite { width: 1, glyphs: &[&["✦"]], fg: tint(STAR_WHITE, theme) },
    has_trunk: false,
};

// ─── Shoulder styles ─────────────────────────────────────────────────────────

const SIDEWALK_BG:     Color = Color::Rgb(110, 110, 105);
const SIDEWALK_FG:     Color = Color::Rgb(170, 170, 165);
const HARDEDGE_FG:     Color = Color::Rgb(180, 180, 180);
const SOFTEDGE_FG:     Color = Color::Rgb(110, 110, 110);
const POLE_FG:         Color = Color::Rgb(160, 160, 160);
const RAIL_FG:         Color = Color::Rgb(150, 130, 110);
const RIVER_FG:        Color = Color::Rgb(70, 120, 200);
const PARKED_CAR_FG:   Color = Color::Rgb(120, 130, 180);
const COUNTRY_FG:      Color = Color::Rgb(90, 80, 60);
const TREE_PINE_DIM:   Color = Color::Rgb(25, 105, 25);

/// Helper: returns the transparent-background cell used when a shoulder is
/// off-row or intentionally empty.
#[inline]
fn blank(fallback_bg: Color) -> Cell {
    Cell::new(" ", fallback_bg, fallback_bg)
}

/// Helper: gate for the `repeat` field — `false` means render blank this row.
#[inline]
fn visible(row: i32, repeat: u8) -> bool {
    repeat == 0 || row.rem_euclid(repeat as i32) == 0
}

pub const SHOULDER_EMPTY: ShoulderStyle = ShoulderStyle {
    cell: |_theme, _row, _repeat, fallback_bg| blank(fallback_bg),
};
pub const SHOULDER_SIDEWALK: ShoulderStyle = ShoulderStyle {
    cell: |theme, row, repeat, fallback_bg| {
        if !visible(row, repeat) { return blank(fallback_bg); }
        Cell::new("▦", tint(SIDEWALK_FG, theme), tint(SIDEWALK_BG, theme))
    },
};
pub const SHOULDER_HARD_EDGE: ShoulderStyle = ShoulderStyle {
    cell: |theme, row, repeat, fallback_bg| {
        if !visible(row, repeat) { return blank(fallback_bg); }
        Cell::new("┃", tint(HARDEDGE_FG, theme), fallback_bg)
    },
};
pub const SHOULDER_SOFT_EDGE: ShoulderStyle = ShoulderStyle {
    cell: |theme, row, repeat, fallback_bg| {
        if !visible(row, repeat) { return blank(fallback_bg); }
        Cell::new("·", tint(SOFTEDGE_FG, theme), fallback_bg)
    },
};
pub const SHOULDER_PARKED_CAR: ShoulderStyle = ShoulderStyle {
    cell: |theme, row, repeat, fallback_bg| {
        if !visible(row, repeat) { return blank(fallback_bg); }
        Cell::new("▣", tint(PARKED_CAR_FG, theme), fallback_bg)
    },
};
pub const SHOULDER_POLES: ShoulderStyle = ShoulderStyle {
    cell: |theme, row, repeat, fallback_bg| {
        if !visible(row, repeat) { return blank(fallback_bg); }
        Cell::new("●", tint(POLE_FG, theme), fallback_bg)
    },
};
pub const SHOULDER_RAILROAD: ShoulderStyle = ShoulderStyle {
    cell: |theme, row, repeat, fallback_bg| {
        if !visible(row, repeat) { return blank(fallback_bg); }
        Cell::new("╫", tint(RAIL_FG, theme), fallback_bg)
    },
};
pub const SHOULDER_RIVER: ShoulderStyle = ShoulderStyle {
    cell: |theme, row, repeat, fallback_bg| {
        if !visible(row, repeat) { return blank(fallback_bg); }
        Cell::new("▚", tint(RIVER_FG, theme), fallback_bg)
    },
};
pub const SHOULDER_COUNTRY_ROAD: ShoulderStyle = ShoulderStyle {
    cell: |theme, row, repeat, fallback_bg| {
        if !visible(row, repeat) { return blank(fallback_bg); }
        Cell::new("░", tint(COUNTRY_FG, theme), fallback_bg)
    },
};
pub const SHOULDER_TREE_PINE: ShoulderStyle = ShoulderStyle {
    cell: |theme, row, repeat, fallback_bg| {
        if !visible(row, repeat) { return blank(fallback_bg); }
        Cell::new("▲", tint(TREE_PINE_DIM, theme), fallback_bg)
    },
};
pub const SHOULDER_TREE_OAK: ShoulderStyle = ShoulderStyle {
    cell: |theme, row, repeat, fallback_bg| {
        if !visible(row, repeat) { return blank(fallback_bg); }
        Cell::new("◉", tint(TREE_OAK_GREEN, theme), fallback_bg)
    },
};
pub const SHOULDER_TREE_PALM: ShoulderStyle = ShoulderStyle {
    cell: |theme, row, repeat, fallback_bg| {
        if !visible(row, repeat) { return blank(fallback_bg); }
        Cell::new("✤", tint(TREE_PALM_GREEN, theme), fallback_bg)
    },
};

// ─── Obstacle styles ─────────────────────────────────────────────────────────

const POTHOLE_FG:     Color = Color::Rgb(50, 50, 50);
const SPEEDBUMP_FG:   Color = Color::Rgb(240, 220, 0);
const SPIKES_FG:      Color = Color::Rgb(220, 40, 40);
const FALLEN_TREE_FG: Color = Color::Rgb(220, 40, 40);

pub const OBSTACLE_POTHOLE_SMALL: ObstacleStyle = ObstacleStyle {
    id: 1,
    label: "Pothole",
    glyphs: (["·", "◉", "·"], POTHOLE_FG),
};
pub const OBSTACLE_POTHOLE_BIG: ObstacleStyle = ObstacleStyle {
    id: 2,
    label: "BIG pothole",
    glyphs: (["◉", "◉", "◉"], POTHOLE_FG),
};
pub const OBSTACLE_POTHOLE_CRATER: ObstacleStyle = ObstacleStyle {
    id: 3,
    label: "CRATER",
    glyphs: (["╳", "◉", "╳"], POTHOLE_FG),
};
pub const OBSTACLE_SPEED_BUMP: ObstacleStyle = ObstacleStyle {
    id: 4,
    label: "Speed bump",
    glyphs: (["▁", "▂", "▁"], SPEEDBUMP_FG),
};
pub const OBSTACLE_SPIKES: ObstacleStyle = ObstacleStyle {
    id: 5,
    label: "SPIKES",
    glyphs: (["▲", "▲", "▲"], SPIKES_FG),
};
pub const OBSTACLE_FALLEN_TREE: ObstacleStyle = ObstacleStyle {
    id: 6,
    label: "Fallen tree",
    glyphs: (["≣", "≣", "≣"], FALLEN_TREE_FG),
};
