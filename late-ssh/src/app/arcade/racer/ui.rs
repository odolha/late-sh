use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
};

use super::state::{Config, Lane, Phase, State, TrafficDir};
use crate::app::arcade::ui::{
    GameBottomBar, draw_game_frame, draw_game_overlay, keys_line, status_line,
};
use crate::app::common::theme;

// ─── Road colors ─────────────────────────────────────────────────────────────

const ROAD_BG: Color = Color::Rgb(18, 18, 18);
const BORDER_FG: Color = Color::Rgb(80, 80, 80);
const DIVIDER_FG: Color = Color::Rgb(200, 160, 0);
const LANE_MARKING_FG: Color = Color::Rgb(60, 60, 60);
/// Dashed lane divider between adjacent lanes in the same direction group.
const LANE_DIVIDER_FG: Color = Color::Rgb(120, 120, 120);

const PLAYER_FG: Color = Color::Cyan;
const SAME_DIR_FG: Color = Color::Rgb(180, 180, 180);
const ONCOMING_FG: Color = Color::Rgb(230, 60, 60);

// ─── Verge / tree colors ──────────────────────────────────────────────────────

const VERGE_BG: Color = Color::Rgb(12, 42, 12);
const TREE_HI: Color = Color::Rgb(55, 175, 55);
const TREE_LO: Color = Color::Rgb(25, 105, 25);
const TRUNK_FG: Color = Color::Rgb(110, 70, 30);

// ─── Minimap colors (muted so they don't distract) ───────────────────────────

const MINI_BG: Color = Color::Rgb(14, 14, 14);
const MINI_BORDER: Color = Color::Rgb(55, 55, 55);
const MINI_DIVIDER: Color = Color::Rgb(70, 58, 20);
const MINI_SAME: Color = Color::Rgb(85, 85, 85);
const MINI_ONCOMING: Color = Color::Rgb(130, 50, 50);
const MINI_PLAYER: Color = Color::Rgb(0, 100, 100);

// ─── Car rendering helpers ────────────────────────────────────────────────────

/// True if `col` (0-based within the lane) is part of the car body.
/// Car occupies the middle 3 cols of the 5-wide lane, with 1-char padding each side.
fn is_car_col(col: u16) -> bool {
    col >= 1 && col <= 3
}

/// Leftmost screen column of lane `lane_idx`, given the road's left-border x.
fn lane_screen_start(road_x: u16, lane_idx: usize) -> u16 {
    let base = road_x + 1 + (lane_idx as u16) * Config::LANE_WIDTH;
    // Add 1 for the group divider once we're in the same-dir group.
    if lane_idx >= Config::LANES_ONCOMING { base + 1 } else { base }
}

// ─── Road draw ────────────────────────────────────────────────────────────────

struct CarOnScreen {
    /// Topmost screen row occupied by the car.
    top_row: i32,
    /// Height in rows.
    height: i32,
    lane: Lane,
    fg: Color,
}

fn collect_cars(state: &State) -> Vec<CarOnScreen> {
    let mut cars = Vec::with_capacity(state.ai_cars.len() + 1);

    cars.push(CarOnScreen {
        top_row: Config::PLAYER_TOP_ROW as i32,
        height: Config::CAR_HEIGHT_ROWS as i32,
        lane: state.player_lane,
        fg: PLAYER_FG,
    });

    for ai in &state.ai_cars {
        let center = state.track_to_screen_row(ai.pos_m);
        let h = ai.size.height_rows() as i32;
        cars.push(CarOnScreen {
            top_row: center - h / 2,
            height: h,
            lane: ai.lane,
            fg: match ai.direction {
                TrafficDir::Same => SAME_DIR_FG,
                TrafficDir::Oncoming => ONCOMING_FG,
            },
        });
    }
    cars
}

/// Multiply factor applied to every RGB channel in fade rows.
/// Index 0 = innermost (lightly dimmed), index 3 = outermost (near-black).
const FADE_ROWS: u16 = 4;
const FADE_FACTORS: [f32; 4] = [0.80, 0.50, 0.25, 0.06];

fn darken(c: Color, factor: f32) -> Color {
    match c {
        Color::Rgb(r, g, b) => Color::Rgb(
            (r as f32 * factor) as u8,
            (g as f32 * factor) as u8,
            (b as f32 * factor) as u8,
        ),
        other => other,
    }
}

// ─── Minimap ─────────────────────────────────────────────────────────────────

/// X column of lane `lane_idx` within the minimap (relative to minimap's left edge).
fn mini_lane_offset(lane_idx: usize) -> u16 {
    let base = 1 + lane_idx as u16;
    if lane_idx >= Config::LANES_ONCOMING { base + 1 } else { base }
}

fn draw_minimap(frame: &mut Frame, area: Rect, state: &State) {
    // 1 minimap row = CAR_HEIGHT_ROWS game rows = one "car height" of road
    let scale_m = Config::CAR_HEIGHT_ROWS as f32 * Config::METERS_PER_ROW;
    let rows = ((Config::MINIMAP_RANGE_M / scale_m) as u16).min(area.height);
    let buf = frame.buffer_mut();
    let divider_x = area.x + 1 + Config::LANES_ONCOMING as u16;
    let right_border_x = area.x + Config::MINI_W - 1;

    // Road outline — draw every row first, cars painted on top below
    for mr in 0..rows {
        let sy = area.y + mr;
        for x in area.x..area.x + Config::MINI_W {
            if let Some(c) = buf.cell_mut((x, sy)) {
                if x == area.x || x == right_border_x {
                    c.set_symbol("│").set_fg(MINI_BORDER).set_bg(MINI_BG);
                } else if x == divider_x {
                    c.set_symbol("·").set_fg(MINI_DIVIDER).set_bg(MINI_BG);
                } else {
                    c.set_symbol(" ").set_fg(MINI_BG).set_bg(MINI_BG);
                }
            }
        }
    }

    // AI cars — mr=0 is furthest ahead, mr=rows-1 is nearest to player
    for car in &state.ai_cars {
        let ahead_m = car.pos_m - state.player_pos_m;
        if ahead_m < 0.0 {
            continue;
        }
        let idx = (ahead_m / scale_m) as u16;
        if idx >= rows { continue; }
        let mr = (rows - 1) - idx;
        let sy = area.y + mr;
        let x = area.x + mini_lane_offset(car.lane.0);
        let fg = match car.direction {
            TrafficDir::Same => MINI_SAME,
            TrafficDir::Oncoming => MINI_ONCOMING,
        };
        if let Some(c) = buf.cell_mut((x, sy)) {
            c.set_symbol("▪").set_fg(fg).set_bg(MINI_BG);
        }
    }

    // Player marker at the bottom
    let x = area.x + mini_lane_offset(state.player_lane.0);
    if let Some(c) = buf.cell_mut((x, area.y + rows.saturating_sub(1))) {
        c.set_symbol("▲").set_fg(MINI_PLAYER).set_bg(MINI_BG);
    }
}

// ─── Verge (trees) ───────────────────────────────────────────────────────────

// (dist_from_road_edge, period_rows, phase_offset)
// Sprite: row_in_period 2=▲hi (top, rendered highest on screen),
//                       1=▲lo, 0=│ trunk (bottom, rendered lowest).
const LEFT_TREES: &[(u16, i32, i32)] = &[
    (2, 22, 0),
    (5, 18, 9),
    (8, 26, 4),
];
const RIGHT_TREES: &[(u16, i32, i32)] = &[
    (2, 22, 11),
    (5, 18, 3),
    (8, 26, 17),
];

fn tree_sym(row_in_period: i32) -> Option<(&'static str, Color)> {
    match row_in_period {
        0 => Some(("│", TRUNK_FG)),
        1 => Some(("▲", TREE_LO)),
        2 => Some(("▲", TREE_HI)),
        _ => None,
    }
}

fn draw_verge(frame: &mut Frame, road_area: Rect, left_x: u16, right_end: u16, state: &State) {
    let buf = frame.buffer_mut();
    let track_base = (state.player_pos_m / Config::METERS_PER_ROW) as i32;

    for r in 0..Config::VISIBLE_ROWS {
        let screen_y = road_area.y + r;
        if screen_y >= road_area.y + road_area.height {
            break;
        }
        let ri = r as i32;
        let track_row = track_base - (ri - Config::PLAYER_TOP_ROW as i32);

        // Fade factor for top/bottom edges (matches road fade).
        let bottom_fade_start = road_area.height.saturating_sub(FADE_ROWS);
        let fade: Option<f32> = if r < FADE_ROWS {
            Some(FADE_FACTORS[(FADE_ROWS - 1 - r) as usize])
        } else if r >= bottom_fade_start {
            Some(FADE_FACTORS[(r - bottom_fade_start) as usize])
        } else {
            None
        };
        let bg = fade.map_or(VERGE_BG, |f| darken(VERGE_BG, f));

        // Fill entire verge band with background.
        for x in left_x..road_area.x {
            if let Some(c) = buf.cell_mut((x, screen_y)) {
                c.set_symbol(" ").set_bg(bg);
            }
        }
        let road_right = road_area.x + road_area.width;
        for x in road_right..right_end {
            if let Some(c) = buf.cell_mut((x, screen_y)) {
                c.set_symbol(" ").set_bg(bg);
            }
        }

        // Skip trees in fade zones — they can't be faded gracefully.
        if fade.is_some() {
            continue;
        }

        // Left tree lanes.
        for &(dist, period, phase) in LEFT_TREES {
            let x = match road_area.x.checked_sub(dist) {
                Some(x) if x >= left_x => x,
                _ => continue,
            };
            let row_in_period = (track_row - phase).rem_euclid(period);
            if let Some((sym, fg)) = tree_sym(row_in_period) {
                if let Some(c) = buf.cell_mut((x, screen_y)) {
                    c.set_symbol(sym).set_fg(fg).set_bg(VERGE_BG);
                }
            }
        }

        // Right tree lanes.
        for &(dist, period, phase) in RIGHT_TREES {
            let x = road_right + dist - 1;
            if x >= right_end { continue; }
            let row_in_period = (track_row - phase).rem_euclid(period);
            if let Some((sym, fg)) = tree_sym(row_in_period) {
                if let Some(c) = buf.cell_mut((x, screen_y)) {
                    c.set_symbol(sym).set_fg(fg).set_bg(VERGE_BG);
                }
            }
        }
    }
}

fn draw_road(frame: &mut Frame, area: Rect, state: &State) {
    let cars = collect_cars(state);
    let buf = frame.buffer_mut();

    // Base track-row index at the player's front so road markings scroll with speed.
    let player_track_row = (state.player_pos_m / Config::METERS_PER_ROW) as i32;

    for r in 0..Config::VISIBLE_ROWS {
        let screen_y = area.y + r;
        if screen_y >= area.y + area.height {
            break;
        }
        let ri = r as i32;

        // Track-space row for this screen row: scrolls as the player moves.
        let track_row = player_track_row - (ri - Config::PLAYER_TOP_ROW as i32);

        // Determine fade level for this row (applied after normal render).
        let bottom_fade_start = area.height.saturating_sub(FADE_ROWS);
        let fade_idx: Option<usize> = if r < FADE_ROWS {
            Some((FADE_ROWS - 1 - r) as usize) // r=0 → idx 3 (darkest), r=3 → idx 0
        } else if r >= bottom_fade_start {
            Some((r - bottom_fade_start) as usize) // r=h-4 → idx 0, r=h-1 → idx 3 (darkest)
        } else {
            None
        };

        // Left border
        if let Some(cell) = buf.cell_mut((area.x, screen_y)) {
            cell.set_symbol("│").set_fg(BORDER_FG).set_bg(ROAD_BG);
        }

        // Each lane is rendered left-to-right; oncoming group then group divider then same-dir group.
        let divider_x = area.x + 1 + (Config::LANES_ONCOMING as u16) * Config::LANE_WIDTH;

        for lane_idx in 0..Config::TOTAL_LANES {
            let lane = Lane(lane_idx);
            let lane_x_start = lane_screen_start(area.x, lane_idx);
            let car_hit = cars.iter().find(|c| {
                c.lane == lane && ri >= c.top_row && ri < c.top_row + c.height
            });

            for col in 0..Config::LANE_WIDTH {
                let screen_x = lane_x_start + col;
                if let Some(cell) = buf.cell_mut((screen_x, screen_y)) {
                    if let Some(car) = car_hit && is_car_col(col) {
                        cell.set_symbol("█").set_fg(car.fg).set_bg(ROAD_BG);
                    } else {
                        let (sym, fg) = lane_bg_cell(track_row, col, lane);
                        cell.set_symbol(sym).set_fg(fg).set_bg(ROAD_BG);
                    }
                }
            }
        }

        // Group divider — dashed yellow line between oncoming and same-dir sides.
        let divider_sym = if track_row.rem_euclid(4) < 2 { "│" } else { " " };
        if let Some(cell) = buf.cell_mut((divider_x, screen_y)) {
            cell.set_symbol(divider_sym).set_fg(DIVIDER_FG).set_bg(ROAD_BG);
        }

        // Right border
        let right_border_x = area.x + Config::TOTAL_ROAD_WIDTH - 1;
        if let Some(cell) = buf.cell_mut((right_border_x, screen_y)) {
            cell.set_symbol("│").set_fg(BORDER_FG).set_bg(ROAD_BG);
        }

        // Darken every cell in fade rows so road content fades to black at edges.
        if let Some(fi) = fade_idx {
            let factor = FADE_FACTORS[fi];
            for x in 0..Config::TOTAL_ROAD_WIDTH {
                if let Some(cell) = buf.cell_mut((area.x + x, screen_y)) {
                    let new_bg = darken(cell.bg, factor);
                    let new_fg = darken(cell.fg, factor);
                    cell.set_bg(new_bg).set_fg(new_fg);
                }
            }
        }
    }
}

/// Empty lane cell.
/// - At the very outer road edges: subtle dotted shoulder markings.
/// - At boundaries between two lanes in the same direction group: dashed white divider.
/// - Everywhere else: blank road.
fn lane_bg_cell(track_row: i32, col: u16, lane: Lane) -> (&'static str, Color) {
    let lane_idx = lane.0;
    let last_lane_w = Config::LANE_WIDTH - 1;

    // Dashed white divider on the shared edge between two same-group lanes.
    let next_same_group = lane_idx + 1 < Config::TOTAL_LANES
        && Lane(lane_idx + 1).direction() == lane.direction();
    let prev_same_group = lane_idx > 0
        && Lane(lane_idx - 1).direction() == lane.direction();

    if col == last_lane_w && next_same_group {
        if track_row.rem_euclid(4) < 2 {
            return ("│", LANE_DIVIDER_FG);
        }
        return (" ", ROAD_BG);
    }
    if col == 0 && prev_same_group {
        // The previous lane already drew the divider on its right edge; leave blank.
        return (" ", ROAD_BG);
    }

    // Outer road shoulders only (leftmost lane's col 0, rightmost lane's col 4).
    let is_outer_left = lane_idx == 0 && col == 0;
    let is_outer_right = lane_idx == Config::TOTAL_LANES - 1 && col == last_lane_w;
    if (is_outer_left || is_outer_right) && track_row.rem_euclid(6) < 3 {
        return ("·", LANE_MARKING_FG);
    }
    (" ", ROAD_BG)
}

// ─── Stats panel ─────────────────────────────────────────────────────────────

fn draw_stats(frame: &mut Frame, area: Rect, state: &State) {
    if area.width < 14 || area.height < 6 {
        return;
    }

    let pct = state.progress_pct();
    let km_done = state.player_pos_m * Config::WORLD_DISTANCE_SCALE / 1000.0;
    let km_total = Config::TRACK_LENGTH_M * Config::WORLD_DISTANCE_SCALE / 1000.0;

    let mut lines: Vec<Line<'static>> = vec![
        Line::from(""),
        Line::from(Span::styled(
            " SHIT I'M LATE",
            Style::default()
                .fg(theme::AMBER())
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(" Speed  ", Style::default().fg(theme::TEXT_DIM())),
            Span::styled(
                format!("{:.0} km/h", state.player_speed_kmh),
                Style::default()
                    .fg(speed_color(state.player_speed_kmh))
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(" Track  ", Style::default().fg(theme::TEXT_DIM())),
            Span::styled(
                format!("{:.1} / {:.0} km", km_done, km_total),
                Style::default().fg(theme::TEXT_BRIGHT()),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(" Time   ", Style::default().fg(theme::TEXT_DIM())),
            Span::styled(
                state.elapsed_formatted(),
                Style::default().fg(theme::TEXT_BRIGHT()),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(" Score  ", Style::default().fg(theme::TEXT_DIM())),
            Span::styled(
                format_score(state.score),
                Style::default()
                    .fg(theme::AMBER_GLOW())
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(" Best   ", Style::default().fg(theme::TEXT_DIM())),
            Span::styled(
                if state.best_score > 0 {
                    format_score(state.best_score)
                } else {
                    "no runs yet".to_string()
                },
                Style::default().fg(theme::SUCCESS()),
            ),
        ]),
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            " Progress",
            Style::default().fg(theme::TEXT_DIM()),
        )),
    ];

    // Progress bar placeholder text (Gauge won't fit nicely in a list, so use text bar)
    let bar_width = (area.width as usize).saturating_sub(4).min(20);
    let filled = ((pct / 100.0) * bar_width as f32) as usize;
    let empty = bar_width.saturating_sub(filled);
    let bar = format!(" [{}{}] {:.0}%", "█".repeat(filled), "░".repeat(empty), pct);
    lines.push(Line::from(Span::styled(
        bar,
        Style::default().fg(theme::AMBER()),
    )));

    // Lane indicator
    lines.push(Line::from(""));
    let (lane_str, lane_color) = if state.player_lane.is_oncoming() {
        (
            format!(" Lane    {} of {} (ONCOMING!)",
                state.player_lane.0 + 1, Config::TOTAL_LANES),
            Color::Red,
        )
    } else {
        let same_idx = state.player_lane.0 - Config::LANES_ONCOMING + 1;
        (
            format!(" Lane    {} of {} (your side)", same_idx, Config::LANES_SAME_DIR),
            theme::SUCCESS(),
        )
    };
    lines.push(Line::from(Span::styled(
        lane_str,
        Style::default().fg(lane_color),
    )));

    let para = Paragraph::new(lines).wrap(Wrap { trim: false });
    frame.render_widget(para, area);
}

fn speed_color(kmh: f32) -> Color {
    if kmh >= 150.0 {
        Color::Red
    } else if kmh >= 100.0 {
        Color::Yellow
    } else if kmh >= 50.0 {
        theme::SUCCESS()
    } else {
        theme::TEXT_DIM()
    }
}

fn format_score(s: i64) -> String {
    // Insert thousands separators
    let s = s.to_string();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    out.chars().rev().collect()
}

// ─── Public draw entry ────────────────────────────────────────────────────────

pub fn draw_game(frame: &mut Frame, area: Rect, state: &State, show_bottom_bar: bool) {
    let bottom = GameBottomBar {
        status: status_line(vec![
            (
                "speed",
                format!("{:.0} km/h", state.player_speed_kmh),
                speed_color(state.player_speed_kmh),
            ),
            (
                "score",
                format_score(state.score),
                theme::AMBER_GLOW(),
            ),
            (
                "progress",
                format!("{:.1}%", state.progress_pct()),
                theme::SUCCESS(),
            ),
        ]),
        keys: keys_line(vec![
            ("w/up", "accel"),
            ("s/dn", "brake"),
            ("spc", "handbrake"),
            ("a/d/lr", "lane"),
            ("p", "pause"),
            ("r", "restart"),
            ("Esc", "exit"),
        ]),
        tip: Some(crate::app::arcade::ui::tip_line(
            "Stay in your lane. Switch left only to overtake - oncoming traffic won't stop for you.",
        )),
    };

    let content_area = draw_game_frame(frame, area, "Shit I'm Late", bottom, show_bottom_bar);

    // Terminal size check
    if content_area.height < Config::MIN_TERMINAL_HEIGHT
        || content_area.width < Config::MIN_TERMINAL_WIDTH
    {
        let msg = Paragraph::new(format!(
            "Terminal too small. Need {}x{} (have {}x{}).",
            Config::MIN_TERMINAL_WIDTH,
            Config::MIN_TERMINAL_HEIGHT,
            content_area.width,
            content_area.height,
        ))
        .alignment(Alignment::Center)
        .style(Style::default().fg(theme::ERROR()));
        frame.render_widget(msg, content_area);
        return;
    }

    // Center the road both horizontally and vertically; stats panel to its right.
    let road_width = Config::TOTAL_ROAD_WIDTH;
    let road_height = content_area.height.min(Config::VISIBLE_ROWS);
    let mini_gap: u16 = 2; // gap between minimap and left tree zone
    let tree_l: u16 = 6;
    let tree_r: u16 = 8;
    let stats_gap: u16 = 2;
    let stats_min: u16 = 28;
    let block_w = Config::MINI_W + mini_gap + tree_l + road_width + tree_r + stats_gap + stats_min;
    let block_x = content_area.x + content_area.width.saturating_sub(block_w) / 2;
    let mini_x = block_x;
    let tree_left_x = block_x + Config::MINI_W + mini_gap;
    let road_x = tree_left_x + tree_l;
    let road_y = content_area.y + content_area.height.saturating_sub(road_height) / 2;
    let right_tree_end = road_x + road_width + tree_r;
    let stats_x = right_tree_end + stats_gap;
    let stats_width = (content_area.x + content_area.width).saturating_sub(stats_x);

    let road_area = Rect {
        x: road_x,
        y: road_y,
        width: road_width,
        height: road_height,
    };
    let stats_area = Rect {
        x: stats_x,
        y: road_y,
        width: stats_width,
        height: road_height,
    };

    let minimap_rows =
        (Config::MINIMAP_RANGE_M / (Config::CAR_HEIGHT_ROWS as f32 * Config::METERS_PER_ROW))
            as u16;
    let mini_area = Rect {
        x: mini_x,
        y: road_y,
        width: Config::MINI_W,
        height: minimap_rows.min(road_height),
    };

    draw_minimap(frame, mini_area, state);
    draw_verge(frame, road_area, tree_left_x, right_tree_end, state);
    draw_road(frame, road_area, state);
    draw_stats(frame, stats_area, state);

    // Overlays
    match &state.phase {
        Phase::Dead => {
            draw_game_overlay(
                frame,
                road_area,
                "CRASH!",
                "Press r to restart",
                theme::ERROR(),
            );
        }
        Phase::Finished { elapsed_s, score } => {
            let mins = (*elapsed_s as u32) / 60;
            let secs = (*elapsed_s as u32) % 60;
            let tenth = ((*elapsed_s - *elapsed_s as u32 as f32) * 10.0) as u32;
            draw_game_overlay(
                frame,
                road_area,
                "FINISHED!",
                &format!(
                    "{}:{:02}.{}  score {}",
                    mins,
                    secs,
                    tenth,
                    format_score(*score)
                ),
                theme::SUCCESS(),
            );
        }
        Phase::Playing if state.is_paused => {
            draw_game_overlay(
                frame,
                road_area,
                "PAUSED",
                "Press p to resume",
                theme::AMBER(),
            );
        }
        _ => {}
    }
}
