//! Theme-aware rendering helpers.
//!
//! Centralises every `(*Aspect, Theme) -> (glyph, fg, bg)` lookup so the
//! draw loop in `ui.rs` stays geometry-only. Adding a new theme means
//! extending the `match` arms here; adding a new aspect adds a new arm.

use ratatui::style::Color;

use super::track::{
    DividerAspect, LaneAspect, ObjectAspect, ObstacleAspect, SceneryBackground,
    ShoulderAspect, StageIcon, Theme,
};

// ─── Themed cell ────────────────────────────────────────────────────────────

/// One rendered cell: glyph + foreground + background.
#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub sym: &'static str,
    pub fg: Color,
    pub bg: Color,
}

impl Cell {
    pub const fn new(sym: &'static str, fg: Color, bg: Color) -> Self {
        Self { sym, fg, bg }
    }
}

// ─── Stage / theme glyphs (used in the right-side HUD) ──────────────────────

pub fn stage_icon_glyph(icon: StageIcon) -> &'static str {
    match icon {
        StageIcon::Metropolis => "🏙",
        StageIcon::City => "🏢",
        StageIcon::CityOutskirts => "🏘",
        StageIcon::Village => "🏡",
        StageIcon::Highway => "🛣",
        StageIcon::WildPlains => "🌾",
        StageIcon::WildHills => "⛰",
        StageIcon::WildForest => "🌲",
        StageIcon::SlopeUp => "📈",
        StageIcon::SlopeDown => "📉",
        StageIcon::Special => "✨",
    }
}

pub fn theme_icon_glyph(theme: Theme) -> &'static str {
    match theme {
        Theme::Standard => "☀",
        Theme::Winter => "❄",
        Theme::Desert => "🌵",
    }
}

// ─── Lane backgrounds ───────────────────────────────────────────────────────

const ASPHALT_BG: Color = Color::Rgb(18, 18, 18);
const ASPHALT_PREMIUM_BG: Color = Color::Rgb(28, 28, 32);
const ASPHALT_PATCHY_BG: Color = Color::Rgb(22, 20, 18);
const ASPHALT_PATCH_FG: Color = Color::Rgb(70, 65, 60);
const GRAVEL_BG: Color = Color::Rgb(70, 60, 48);
const GRAVEL_FG: Color = Color::Rgb(120, 105, 90);
const DIRT_BG: Color = Color::Rgb(82, 56, 36);
const DIRT_FG: Color = Color::Rgb(120, 90, 60);
const LANE_GRASS_BG: Color = Color::Rgb(28, 60, 28);
const LANE_GRASS_FG: Color = Color::Rgb(70, 110, 60);

/// Themed tint applied multiplicatively to base colours.
fn tint(c: Color, theme: Theme) -> Color {
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
            ((g as u16 * 9 / 10).min(255)) as u8,
            ((b as u16 * 7 / 10).min(255)) as u8,
        ),
    }
}

/// One cell of a lane surface. `col` is the column within the lane (0..LANE_WIDTH);
/// `row` is the absolute track row so textures scroll with the road.
pub fn lane_aspect_cell(aspect: LaneAspect, theme: Theme, row: i32, _col: u16) -> Cell {
    let (sym, fg, bg) = match aspect {
        LaneAspect::AsphaltPremium => (" ", ASPHALT_PREMIUM_BG, ASPHALT_PREMIUM_BG),
        LaneAspect::AsphaltStandard => (" ", ASPHALT_BG, ASPHALT_BG),
        LaneAspect::AsphaltPatchy => {
            if row.rem_euclid(7) == 0 {
                (".", ASPHALT_PATCH_FG, ASPHALT_PATCHY_BG)
            } else {
                (" ", ASPHALT_PATCHY_BG, ASPHALT_PATCHY_BG)
            }
        }
        LaneAspect::GravelRoad => {
            if row.rem_euclid(3) == 0 {
                (",", GRAVEL_FG, GRAVEL_BG)
            } else {
                (" ", GRAVEL_BG, GRAVEL_BG)
            }
        }
        LaneAspect::DirtRoad => {
            if row.rem_euclid(5) < 2 {
                ("·", DIRT_FG, DIRT_BG)
            } else {
                (" ", DIRT_BG, DIRT_BG)
            }
        }
        LaneAspect::Grass => {
            if row.rem_euclid(6) == 0 {
                ("v", LANE_GRASS_FG, LANE_GRASS_BG)
            } else {
                (" ", LANE_GRASS_BG, LANE_GRASS_BG)
            }
        }
    };
    Cell { sym, fg: tint(fg, theme), bg: tint(bg, theme) }
}

/// Background colour for a lane (used for car bodies, overlays).
pub fn lane_aspect_bg(aspect: LaneAspect, theme: Theme) -> Color {
    let bg = match aspect {
        LaneAspect::AsphaltPremium => ASPHALT_PREMIUM_BG,
        LaneAspect::AsphaltStandard => ASPHALT_BG,
        LaneAspect::AsphaltPatchy => ASPHALT_PATCHY_BG,
        LaneAspect::GravelRoad => GRAVEL_BG,
        LaneAspect::DirtRoad => DIRT_BG,
        LaneAspect::Grass => LANE_GRASS_BG,
    };
    tint(bg, theme)
}

// ─── Dividers ───────────────────────────────────────────────────────────────

const YELLOW: Color = Color::Rgb(220, 180, 30);
const WHITE: Color = Color::Rgb(180, 180, 180);
const FAINT: Color = Color::Rgb(85, 85, 85);

/// Render one cell of a lane separator column at the given absolute track row.
pub fn divider_cell(aspect: DividerAspect, _theme: Theme, row: i32, bg: Color) -> Cell {
    let mod4 = row.rem_euclid(4);
    let mod3 = row.rem_euclid(3);
    let (sym, fg) = match aspect {
        DividerAspect::YellowDouble => ("‖", YELLOW),
        DividerAspect::YellowSingle => ("│", YELLOW),
        DividerAspect::YellowDash => (if mod4 < 2 { "│" } else { " " }, YELLOW),
        DividerAspect::YellowDots => (if mod3 == 0 { "·" } else { " " }, YELLOW),
        DividerAspect::WhiteSingle => ("│", WHITE),
        DividerAspect::WhiteDash => (if mod4 < 2 { "│" } else { " " }, WHITE),
        DividerAspect::WhiteDots => (if mod3 == 0 { "·" } else { " " }, WHITE),
        DividerAspect::Faint => ("│", FAINT),
        DividerAspect::None => (" ", bg),
    };
    Cell { sym, fg, bg }
}

// ─── Sceneries ──────────────────────────────────────────────────────────────

const CONCRETE_BG: Color = Color::Rgb(60, 60, 60);
const SC_GRASS_BG: Color = Color::Rgb(20, 55, 20);
const SC_DIRT_BG: Color = Color::Rgb(70, 50, 30);
const VOID_BG: Color = Color::Rgb(2, 2, 6);

pub fn scenery_bg(bg: SceneryBackground, theme: Theme) -> Color {
    let c = match bg {
        SceneryBackground::Concrete => CONCRETE_BG,
        SceneryBackground::Grass => SC_GRASS_BG,
        SceneryBackground::Dirt => SC_DIRT_BG,
        SceneryBackground::Void => VOID_BG,
    };
    tint(c, theme)
}

// ─── Objects (scenery) ─────────────────────────────────────────────────────

/// A small ASCII sprite. Rendered with its bottom-left anchor at the given cell;
/// extends upward by `glyphs.len() - 1` rows and rightward by `width - 1` cols.
#[derive(Clone, Copy, Debug)]
pub struct Sprite {
    pub width: u8,
    pub glyphs: &'static [&'static str],
    pub fg: Color,
}

const TREE_PINE_GREEN: Color = Color::Rgb(55, 175, 55);
const TREE_PINE_DIM: Color = Color::Rgb(25, 105, 25);
const TREE_OAK_GREEN: Color = Color::Rgb(80, 160, 70);
const TREE_PALM_GREEN: Color = Color::Rgb(120, 180, 70);
const TRUNK_BROWN: Color = Color::Rgb(110, 70, 30);
const FLOWER_PINK: Color = Color::Rgb(200, 110, 160);
const BUSH_GREEN: Color = Color::Rgb(70, 130, 60);
const GRASS_BLADE: Color = Color::Rgb(70, 130, 70);
const BUILDING_GRAY: Color = Color::Rgb(140, 140, 140);
const STAR_WHITE: Color = Color::Rgb(220, 220, 220);

/// Sprite for the given object. Theme tinting is applied.
pub fn object_sprite(aspect: ObjectAspect, theme: Theme) -> Sprite {
    let raw = match aspect {
        ObjectAspect::Grass => Sprite { width: 1, glyphs: &["v"], fg: GRASS_BLADE },
        ObjectAspect::Bush => Sprite { width: 1, glyphs: &["o"], fg: BUSH_GREEN },
        ObjectAspect::TreePine => Sprite {
            width: 1,
            glyphs: &["|", "A", "A"], // bottom: trunk, mid: lower crown, top: upper crown
            fg: TREE_PINE_GREEN,
        },
        ObjectAspect::TreeOak => Sprite {
            width: 1,
            glyphs: &["|", "O"],
            fg: TREE_OAK_GREEN,
        },
        ObjectAspect::TreePalm => Sprite {
            width: 1,
            glyphs: &["|", "Y"],
            fg: TREE_PALM_GREEN,
        },
        ObjectAspect::BuildingHouse => Sprite {
            width: 3,
            glyphs: &["[_]", "/-\\"], // bottom row first
            fg: BUILDING_GRAY,
        },
        ObjectAspect::BuildingApartments => Sprite {
            width: 3,
            glyphs: &["[#]", "[#]", "[#]", "/-\\"],
            fg: BUILDING_GRAY,
        },
        ObjectAspect::Skyscraper => Sprite {
            width: 3,
            glyphs: &["[#]", "[#]", "[#]", "[#]", "[#]", "/A\\"],
            fg: BUILDING_GRAY,
        },
        ObjectAspect::Flower => Sprite { width: 1, glyphs: &["*"], fg: FLOWER_PINK },
        ObjectAspect::Star => Sprite { width: 1, glyphs: &["*"], fg: STAR_WHITE },
    };
    // Special case: trunks should stay brown regardless of theme. We accept
    // a single foreground per sprite — to keep this simple, multi-row tree
    // sprites use the green fg and the consumer overrides the trunk row.
    Sprite { fg: tint(raw.fg, theme), ..raw }
}

/// True if this object aspect has a separate trunk colour that should override
/// the sprite's `fg` on the bottom row.
pub fn object_has_trunk(aspect: ObjectAspect) -> bool {
    matches!(
        aspect,
        ObjectAspect::TreePine | ObjectAspect::TreeOak | ObjectAspect::TreePalm
    )
}

pub fn trunk_color(theme: Theme) -> Color {
    tint(TRUNK_BROWN, theme)
}

// ─── Shoulders ─────────────────────────────────────────────────────────────

const SIDEWALK_BG: Color = Color::Rgb(110, 110, 105);
const SIDEWALK_FG: Color = Color::Rgb(170, 170, 165);
const HARDEDGE_FG: Color = Color::Rgb(180, 180, 180);
const SOFTEDGE_FG: Color = Color::Rgb(110, 110, 110);
const POLE_FG: Color = Color::Rgb(160, 160, 160);
const RAIL_FG: Color = Color::Rgb(150, 130, 110);
const RIVER_FG: Color = Color::Rgb(70, 120, 200);
const PARKED_CAR_FG: Color = Color::Rgb(120, 130, 180);
const COUNTRY_FG: Color = Color::Rgb(90, 80, 60);

/// Render a shoulder cell at the given track row.
/// `repeat = 0` → continuous; `repeat > 0` → glyph only every `repeat` rows.
pub fn shoulder_cell(
    aspect: ShoulderAspect,
    theme: Theme,
    row: i32,
    repeat: u8,
    fallback_bg: Color,
) -> Cell {
    let show = repeat == 0 || row.rem_euclid(repeat as i32) == 0;
    if !show {
        return Cell { sym: " ", fg: fallback_bg, bg: fallback_bg };
    }
    let (sym, fg, bg) = match aspect {
        ShoulderAspect::Sidewalk => ("=", SIDEWALK_FG, SIDEWALK_BG),
        ShoulderAspect::HardEdge => ("|", HARDEDGE_FG, fallback_bg),
        ShoulderAspect::SoftEdge => (":", SOFTEDGE_FG, fallback_bg),
        ShoulderAspect::ParkedCar => {
            // Render a 3-row block character so successive rows look like a car.
            ("#", PARKED_CAR_FG, fallback_bg)
        }
        ShoulderAspect::Poles => ("o", POLE_FG, fallback_bg),
        ShoulderAspect::Railroad => ("=", RAIL_FG, fallback_bg),
        ShoulderAspect::River => ("~", RIVER_FG, fallback_bg),
        ShoulderAspect::CountryRoad => (":", COUNTRY_FG, fallback_bg),
        ShoulderAspect::TreePine => ("A", TREE_PINE_DIM, fallback_bg),
        ShoulderAspect::TreeOak => ("O", TREE_OAK_GREEN, fallback_bg),
        ShoulderAspect::TreePalm => ("Y", TREE_PALM_GREEN, fallback_bg),
    };
    Cell { sym, fg: tint(fg, theme), bg: tint(bg, theme) }
}

// ─── Obstacles ─────────────────────────────────────────────────────────────

const POTHOLE_FG: Color = Color::Rgb(0, 0, 0);
const SPEEDBUMP_FG: Color = Color::Rgb(240, 220, 0);
const SPIKES_FG: Color = Color::Rgb(220, 40, 40);
const FALLEN_TREE_FG: Color = Color::Rgb(80, 50, 20);

/// 3-wide glyph row for the given obstacle (matches car body width).
pub fn obstacle_glyph(aspect: ObstacleAspect) -> (&'static str, Color) {
    match aspect {
        ObstacleAspect::PotholeSmall => ("·O·", POTHOLE_FG),
        ObstacleAspect::PotholeBig => ("OOO", POTHOLE_FG),
        ObstacleAspect::PotholeCrater => ("###", POTHOLE_FG),
        ObstacleAspect::SpeedBump => ("===", SPEEDBUMP_FG),
        ObstacleAspect::Spikes => ("^^^", SPIKES_FG),
        ObstacleAspect::FallenTree => ("===", FALLEN_TREE_FG),
    }
}

/// Short label for an obstacle effect — shown in the right-panel effects log.
pub fn obstacle_effect_label(aspect: ObstacleAspect) -> &'static str {
    match aspect {
        ObstacleAspect::PotholeSmall => "pothole",
        ObstacleAspect::PotholeBig => "BIG pothole",
        ObstacleAspect::PotholeCrater => "CRATER",
        ObstacleAspect::SpeedBump => "speed bump",
        ObstacleAspect::Spikes => "SPIKES",
        ObstacleAspect::FallenTree => "fallen tree",
    }
}
