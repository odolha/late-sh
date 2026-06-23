//! Racer game state and physics.
//!
//! Geometry, lane configuration, and speed ranges are no longer hard-coded —
//! they're read live from the active [`Track`] and current [`Stage`] (see
//! `track.rs` and `tracks/`).  What remains in [`Config`] is only the visual
//! frame (terminal rows, lane width in chars, FPS) and the global player
//! input feel (accel/decel/lane-transition speed).

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

use rand::{Rng, random};

use super::track::{ObstacleEffect, ObstacleStyle, Stage, Track};
use super::tracks::{ALL_TRACKS, DEFAULT_TRACK};

// ─── Static visual-frame configuration ──────────────────────────────────────

pub struct Config;

impl Config {
    pub const VISIBLE_ROWS: u16 = 50;
    pub const LANE_WIDTH: u16 = 5;
    pub const CAR_HEIGHT_ROWS: u16 = 3;
    pub const PLAYER_BOTTOM_MARGIN: u16 = 4;
    pub const PLAYER_TOP_ROW: u16 =
        Self::VISIBLE_ROWS - Self::CAR_HEIGHT_ROWS - Self::PLAYER_BOTTOM_MARGIN;
    pub const METERS_PER_ROW: f32 = 3.0;
    pub const VISIBLE_AHEAD_M: f32 = Self::PLAYER_TOP_ROW as f32 * Self::METERS_PER_ROW;
    pub const MINIMAP_RANGE_M: f32 = 2.0 * Self::VISIBLE_ROWS as f32 * Self::METERS_PER_ROW;
    pub const AI_MIN_SEPARATION_M: f32 = 32.0;
    pub const AI_DESPAWN_BEHIND_M: f32 = 200.0;
    pub const AI_FOLLOW_GAP_ROWS: u16 = 5;

    pub const PLAYER_START_SPEED_KMH: f32 = 50.0;
    pub const ACCEL_KMH_PER_S: f32 = 88.0;
    pub const DECEL_KMH_PER_S: f32 = 128.0;
    pub const SPEED_CLAMP_PER_S: f32 = 80.0;
    pub const TICK_DT: f32 = 1.0 / 15.0;
    pub const INPUT_HOLD_MS: u64 = 150;
    pub const LANE_TRANSITION_PER_S: f32 = 7.0;

    pub const RECENT_EFFECTS_CAPACITY: usize = 5;

    pub const INITIAL_SCORE_PER_DISPLAYED_KM: f32 = 100_000.0;
    pub const SCORE_DECAY_PER_S: f32 = 800.0;

    pub const MIN_TERMINAL_WIDTH_FLOOR: u16 = 70;
    pub const MIN_TERMINAL_HEIGHT: u16 = Self::VISIBLE_ROWS + 5;
}

// ─── Top-level state machine ─────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RacerScreen {
    Picker,
    Racing,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TrafficDir {
    Same,
    Oncoming,
}

#[derive(Clone, Copy, Debug)]
pub enum PlayerInput {
    Accelerate,
    Brake,
    Handbrake,
    None,
}

#[derive(Clone, Copy, Debug)]
pub enum Phase {
    Playing,
    Finished { elapsed_s: f32, score: i64 },
    Dead,
}

#[derive(Clone, Debug)]
pub struct AiCar {
    pub pos_m: f32,
    pub speed_kmh: f32,
    pub cruise_kmh: f32,
    pub lane_idx: usize,
    pub direction: TrafficDir,
    pub height_rows: u8,
}

/// Obstacle deterministically placed along a lane; cleared on crossing.
#[derive(Clone, Debug)]
pub struct SpawnedObstacle {
    pub style: ObstacleStyle,
    pub effects: &'static [ObstacleEffect],
    pub crash: bool,
    pub pos_m: f32,
    pub lane_idx: usize,
    pub triggered: bool,
}

#[derive(Clone, Debug)]
pub struct RecentEffect {
    pub label: &'static str,
    pub at: Instant,
}

/// All runtime racer state — drives both the picker and the race.
pub struct State {
    pub screen: RacerScreen,
    pub picker_selected_idx: usize,
    pub best_scores: HashMap<&'static str, i64>,
    pub best_score: i64,

    pub active_track: Option<&'static Track>,
    pub current_stage_idx: usize,
    pub stage_traveled_m: f32,
    pub player_pos_m: f32,
    pub player_speed_kmh: f32,
    pub player_lane_idx: usize,
    pub player_lane_display: f32,
    pub input: PlayerInput,
    pub input_last_set: Option<Instant>,
    pub ai_cars: Vec<AiCar>,
    pub spawn_cooldown_ticks: u32,
    pub obstacles: Vec<SpawnedObstacle>,
    pub obstacle_seed_m: f32,
    pub scenery_seed: u64,
    pub elapsed_s: f32,
    pub score: i64,
    pub phase: Phase,
    pub is_paused: bool,

    pub recent_effects: VecDeque<RecentEffect>,

    pub gas_blocked_until: Option<Instant>,
    pub brake_blocked_until: Option<Instant>,
    pub wheel_blocked_until: Option<Instant>,
}

impl State {
    pub fn new() -> Self {
        Self {
            screen: RacerScreen::Picker,
            picker_selected_idx: 0,
            best_scores: HashMap::new(),
            best_score: 0,
            active_track: None,
            current_stage_idx: 0,
            stage_traveled_m: 0.0,
            player_pos_m: 0.0,
            player_speed_kmh: Config::PLAYER_START_SPEED_KMH,
            player_lane_idx: 0,
            player_lane_display: 0.0,
            input: PlayerInput::None,
            input_last_set: None,
            ai_cars: Vec::new(),
            spawn_cooldown_ticks: 0,
            obstacles: Vec::new(),
            obstacle_seed_m: 0.0,
            scenery_seed: 0,
            elapsed_s: 0.0,
            score: 0,
            phase: Phase::Playing,
            is_paused: false,
            recent_effects: VecDeque::with_capacity(Config::RECENT_EFFECTS_CAPACITY),
            gas_blocked_until: None,
            brake_blocked_until: None,
            wheel_blocked_until: None,
        }
    }

    // ─── Picker ──────────────────────────────────────────────────────────────

    pub fn picker_move(&mut self, delta: i32) {
        let len = ALL_TRACKS.len() as i32;
        if len == 0 { return; }
        let cur = self.picker_selected_idx as i32;
        self.picker_selected_idx = ((cur + delta).rem_euclid(len)) as usize;
    }

    pub fn start_selected_track(&mut self) {
        let track = ALL_TRACKS.get(self.picker_selected_idx).copied().unwrap_or(DEFAULT_TRACK);
        self.start_track(track);
    }

    pub fn start_track(&mut self, track: &'static Track) {
        self.active_track = Some(track);
        self.current_stage_idx = 0;
        self.stage_traveled_m = 0.0;
        self.player_pos_m = 0.0;
        self.player_speed_kmh = Config::PLAYER_START_SPEED_KMH;
        self.player_lane_idx = track.stages[0].road.lanes.player_start_idx();
        self.player_lane_display = self.player_lane_idx as f32;
        self.input = PlayerInput::None;
        self.input_last_set = None;
        self.ai_cars.clear();
        self.spawn_cooldown_ticks = 0;
        self.obstacles.clear();
        self.obstacle_seed_m = 0.0;
        self.scenery_seed = random();
        self.elapsed_s = 0.0;
        self.score = self.initial_score_for(track);
        self.phase = Phase::Playing;
        self.is_paused = false;
        self.recent_effects.clear();
        self.gas_blocked_until = None;
        self.brake_blocked_until = None;
        self.wheel_blocked_until = None;
        self.screen = RacerScreen::Racing;
    }

    pub fn restart_current(&mut self) {
        if let Some(track) = self.active_track {
            self.start_track(track);
        }
    }

    pub fn return_to_picker(&mut self) {
        self.screen = RacerScreen::Picker;
    }

    fn initial_score_for(&self, track: &Track) -> i64 {
        (Config::INITIAL_SCORE_PER_DISPLAYED_KM * track.total_distance_km()) as i64
    }

    // ─── Active track / stage accessors ──────────────────────────────────────

    pub fn track(&self) -> Option<&'static Track> {
        self.active_track
    }

    pub fn current_stage(&self) -> Option<&'static Stage> {
        let track = self.active_track?;
        track.stages.get(self.current_stage_idx)
    }

    pub fn total_lanes(&self) -> usize {
        self.current_stage().map(|s| s.road.lanes.total()).unwrap_or(0)
    }

    pub fn lanes_incoming(&self) -> usize {
        self.current_stage().map(|s| s.road.lanes.incoming.len()).unwrap_or(0)
    }

    pub fn direction_of(&self, lane_idx: usize) -> TrafficDir {
        if lane_idx < self.lanes_incoming() {
            TrafficDir::Oncoming
        } else {
            TrafficDir::Same
        }
    }

    pub fn speed_scale(&self) -> f32 {
        self.active_track.map(|t| t.speed_scale).unwrap_or(1.0)
    }

    pub fn distance_scale(&self) -> f32 {
        self.active_track.map(|t| t.distance_scale).unwrap_or(1.0)
    }

    pub fn displayed_km_total(&self) -> f32 {
        self.player_pos_m / self.distance_scale() / 1000.0
    }

    pub fn displayed_km_stage(&self) -> f32 {
        self.stage_traveled_m / self.distance_scale() / 1000.0
    }

    pub fn track_total_km(&self) -> f32 {
        self.active_track.map(|t| t.total_distance_km()).unwrap_or(0.0)
    }

    pub fn current_stage_km(&self) -> f32 {
        self.current_stage().map(|s| s.distance_km).unwrap_or(0.0)
    }

    // ─── Player control ───────────────────────────────────────────────────────

    pub fn is_playing(&self) -> bool {
        matches!(self.phase, Phase::Playing) && self.screen == RacerScreen::Racing
    }

    pub fn toggle_pause(&mut self) {
        if matches!(self.phase, Phase::Playing) {
            self.is_paused = !self.is_paused;
        }
    }

    pub fn move_left(&mut self) {
        if !self.is_playing() || self.is_paused || self.wheels_blocked() { return; }
        if self.player_lane_idx > 0 {
            self.player_lane_idx -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if !self.is_playing() || self.is_paused || self.wheels_blocked() { return; }
        if self.player_lane_idx + 1 < self.total_lanes() {
            self.player_lane_idx += 1;
        }
    }

    fn wheels_blocked(&self) -> bool {
        self.wheel_blocked_until.map_or(false, |t| Instant::now() < t)
    }

    fn gas_blocked(&self) -> bool {
        self.gas_blocked_until.map_or(false, |t| Instant::now() < t)
    }

    fn brake_blocked(&self) -> bool {
        self.brake_blocked_until.map_or(false, |t| Instant::now() < t)
    }

    pub fn set_input(&mut self, input: PlayerInput) {
        let allowed = match input {
            PlayerInput::Accelerate => !self.gas_blocked(),
            PlayerInput::Brake | PlayerInput::Handbrake => !self.brake_blocked(),
            PlayerInput::None => true,
        };
        if !allowed { return; }
        self.input = input;
        self.input_last_set = Some(Instant::now());
    }

    // ─── Tick ─────────────────────────────────────────────────────────────────

    pub fn tick(&mut self) {
        if !self.is_playing() || self.is_paused { return; }
        let dt = Config::TICK_DT;

        if let Some(t) = self.input_last_set {
            if t.elapsed() > Duration::from_millis(Config::INPUT_HOLD_MS) {
                self.input = PlayerInput::None;
                self.input_last_set = None;
            }
        }

        let lane = self.current_lane_cfg();
        let (own_min, own_max, passive) = lane
            .map(|l| (l.own_min_speed, l.own_max_speed, l.passive_decel))
            .unwrap_or((0.0, 200.0, 0.0));

        match self.input {
            PlayerInput::Accelerate => {
                if self.player_speed_kmh < own_max {
                    self.player_speed_kmh =
                        (self.player_speed_kmh + Config::ACCEL_KMH_PER_S * dt).min(own_max);
                }
            }
            PlayerInput::Brake => {
                if self.player_speed_kmh > own_min {
                    self.player_speed_kmh =
                        (self.player_speed_kmh - Config::DECEL_KMH_PER_S * dt).max(own_min);
                }
            }
            PlayerInput::Handbrake => {
                if self.player_speed_kmh > own_min {
                    self.player_speed_kmh =
                        (self.player_speed_kmh - Config::DECEL_KMH_PER_S * 2.0 * dt).max(own_min);
                }
            }
            PlayerInput::None => {
                if passive > 0.0 && self.player_speed_kmh > own_min {
                    self.player_speed_kmh =
                        (self.player_speed_kmh - passive * dt).max(own_min);
                }
            }
        }

        // Ease speed back into the new lane's bounds after a lane change.
        let step = Config::SPEED_CLAMP_PER_S * dt;
        if self.player_speed_kmh > own_max {
            self.player_speed_kmh = (self.player_speed_kmh - step).max(own_max);
        } else if self.player_speed_kmh < own_min {
            self.player_speed_kmh = (self.player_speed_kmh + step).min(own_min);
        }
        if self.player_speed_kmh < 0.0 {
            self.player_speed_kmh = 0.0;
        }

        let speed_scale = self.speed_scale();
        let player_step = self.player_speed_kmh / 3.6 * speed_scale * dt;
        self.player_pos_m += player_step;
        self.stage_traveled_m += player_step;
        self.elapsed_s += dt;
        self.score = (self.score - (Config::SCORE_DECAY_PER_S * dt) as i64).max(0);

        let target = self.player_lane_idx as f32;
        let max_step = Config::LANE_TRANSITION_PER_S * dt;
        let diff = target - self.player_lane_display;
        if diff.abs() <= max_step {
            self.player_lane_display = target;
        } else {
            self.player_lane_display += diff.signum() * max_step;
        }

        self.update_ai(dt);
        self.manage_ai();
        self.spawn_obstacles_ahead();
        self.check_obstacle_crossings();

        if let Some(stage) = self.current_stage() {
            let stage_distance_m = stage.distance_km * 1000.0 * self.distance_scale();
            if self.stage_traveled_m >= stage_distance_m {
                self.advance_stage();
            }
        }

        if self.check_collision() {
            self.phase = Phase::Dead;
            self.record_best();
            return;
        }

        if let Some(track) = self.active_track {
            let total_m = track.total_distance_km() * 1000.0 * self.distance_scale();
            if self.player_pos_m >= total_m {
                self.phase = Phase::Finished { elapsed_s: self.elapsed_s, score: self.score };
                self.record_best();
            }
        }
    }

    fn current_lane_cfg(&self) -> Option<&'static super::track::Lane> {
        let stage = self.current_stage()?;
        stage.road.lanes.get(self.player_lane_idx)
    }

    fn record_best(&mut self) {
        let Some(track) = self.active_track else { return; };
        let s = self.score;
        let entry = self.best_scores.entry(track.name).or_insert(0);
        if s > *entry { *entry = s; }
        self.best_score = self.best_scores.values().copied().max().unwrap_or(0);
    }

    fn advance_stage(&mut self) {
        let Some(track) = self.active_track else { return; };
        let stage_m = track.stages[self.current_stage_idx].distance_km
            * 1000.0
            * self.distance_scale();
        let overflow = (self.stage_traveled_m - stage_m).max(0.0);
        let next_idx = self.current_stage_idx + 1;
        if next_idx >= track.stages.len() { return; }

        self.current_stage_idx = next_idx;
        self.stage_traveled_m = overflow;

        let new_total = track.stages[next_idx].road.lanes.total();
        self.ai_cars.retain(|c| c.lane_idx < new_total);

        let new_outgoing_start = track.stages[next_idx].road.lanes.incoming.len();
        if self.player_lane_idx >= new_total {
            self.player_lane_idx = new_outgoing_start;
            self.player_lane_display = self.player_lane_idx as f32;
        }
        self.obstacles.clear();
        self.obstacle_seed_m = self.player_pos_m;

        self.push_effect("new stage");
    }

    // ─── AI ──────────────────────────────────────────────────────────────────

    fn update_ai(&mut self, dt: f32) {
        let follow_gap_m = Config::AI_FOLLOW_GAP_ROWS as f32 * Config::METERS_PER_ROW;
        let speed_scale = self.speed_scale();

        let snap: Vec<(f32, usize, TrafficDir, f32, f32)> = self
            .ai_cars
            .iter()
            .map(|c| {
                let half_m = c.height_rows as f32 * Config::METERS_PER_ROW * 0.5;
                (c.pos_m, c.lane_idx, c.direction, c.speed_kmh, half_m)
            })
            .collect();

        for (i, car) in self.ai_cars.iter_mut().enumerate() {
            let my_half = car.height_rows as f32 * Config::METERS_PER_ROW * 0.5;
            let my_front = match car.direction {
                TrafficDir::Same => car.pos_m + my_half,
                TrafficDir::Oncoming => car.pos_m - my_half,
            };

            let mut nearest: Option<(f32, f32)> = None;
            for (j, &(jpos, jlane, jdir, jspeed, jhalf)) in snap.iter().enumerate() {
                if i == j || jlane != car.lane_idx { continue; }
                let j_back = match jdir {
                    TrafficDir::Same => jpos - jhalf,
                    TrafficDir::Oncoming => jpos + jhalf,
                };
                let gap = match car.direction {
                    TrafficDir::Same => j_back - my_front,
                    TrafficDir::Oncoming => my_front - j_back,
                };
                if gap <= 0.0 { continue; }
                if nearest.map_or(true, |(g, _)| gap < g) {
                    nearest = Some((gap, jspeed));
                }
            }

            car.speed_kmh = match nearest {
                Some((gap, lspeed)) if gap < follow_gap_m && lspeed < car.cruise_kmh => lspeed,
                _ => car.cruise_kmh,
            };

            let step = car.speed_kmh / 3.6 * speed_scale * dt;
            match car.direction {
                TrafficDir::Same => car.pos_m += step,
                TrafficDir::Oncoming => car.pos_m -= step,
            }
        }
    }

    fn check_collision(&self) -> bool {
        let p_top = Config::PLAYER_TOP_ROW as i32;
        let p_bot = p_top + Config::CAR_HEIGHT_ROWS as i32 - 1;
        for car in &self.ai_cars {
            if car.lane_idx != self.player_lane_idx { continue; }
            let center = self.track_to_screen_row(car.pos_m);
            let h = car.height_rows as i32;
            let top = center - h / 2;
            let bot = top + h - 1;
            if top <= p_bot && bot >= p_top { return true; }
        }
        false
    }

    fn manage_ai(&mut self) {
        let Some(stage) = self.current_stage() else { return; };
        let lanes = stage.road.lanes;
        let total_lanes = lanes.total();
        if total_lanes == 0 { return; }

        let player_pos = self.player_pos_m;
        self.ai_cars.retain(|car| {
            car.pos_m > player_pos - Config::AI_DESPAWN_BEHIND_M
                && car.lane_idx < total_lanes
        });

        if self.spawn_cooldown_ticks > 0 {
            self.spawn_cooldown_ticks -= 1;
            return;
        }

        let mut rng = rand::thread_rng();
        let mut per_lane_count = vec![0usize; total_lanes];
        for c in &self.ai_cars {
            per_lane_count[c.lane_idx] += 1;
        }

        let mut hungry_lanes: Vec<usize> = (0..total_lanes)
            .filter(|&i| {
                let lane = lanes.get(i).expect("idx in range");
                per_lane_count[i] < lane.traffic_size as usize
            })
            .collect();

        if hungry_lanes.is_empty() {
            self.spawn_cooldown_ticks = rng.gen_range(15..90);
            return;
        }

        let cluster_size = rng.gen_range(1..=3.min(hungry_lanes.len()));
        for _ in 0..cluster_size {
            let pick_idx = rng.gen_range(0..hungry_lanes.len());
            let lane_idx = hungry_lanes[pick_idx];
            let lane = lanes.get(lane_idx).expect("idx in range");
            let direction = self.direction_of(lane_idx);
            let cruise = rng.gen_range(
                lane.traffic_min_speed
                    ..=lane.traffic_max_speed.max(lane.traffic_min_speed + 1.0),
            );

            let extra = rng.r#gen::<f32>()
                * (Config::MINIMAP_RANGE_M - Config::VISIBLE_AHEAD_M).max(1.0);
            let pos = player_pos + Config::VISIBLE_AHEAD_M + extra;
            if !self.spawn_clear(pos, lane_idx, direction) { continue; }

            let height = pick_car_height(lane.traffic_cars, &mut rng);
            self.ai_cars.push(AiCar {
                pos_m: pos,
                speed_kmh: cruise,
                cruise_kmh: cruise,
                lane_idx,
                direction,
                height_rows: height,
            });
            per_lane_count[lane_idx] += 1;
            if per_lane_count[lane_idx] >= lane.traffic_size as usize {
                hungry_lanes.retain(|&i| i != lane_idx);
                if hungry_lanes.is_empty() { break; }
            }
        }
        self.spawn_cooldown_ticks = rng.gen_range(0..60);
    }

    fn spawn_clear(&self, pos: f32, lane_idx: usize, dir: TrafficDir) -> bool {
        self.ai_cars.iter().all(|o| {
            o.lane_idx != lane_idx
                || o.direction != dir
                || (o.pos_m - pos).abs() >= Config::AI_MIN_SEPARATION_M
        })
    }

    // ─── Obstacles ────────────────────────────────────────────────────────────

    fn spawn_obstacles_ahead(&mut self) {
        let Some(stage) = self.current_stage() else { return; };
        let lanes = stage.road.lanes;
        let target_max_m = self.player_pos_m + Config::MINIMAP_RANGE_M;

        const SLOT_M: f32 = 30.0;
        while self.obstacle_seed_m < target_max_m {
            let slot = (self.obstacle_seed_m / SLOT_M) as i64 ^ self.scenery_seed as i64;
            for (lane_idx, lane) in lanes
                .incoming
                .iter()
                .chain(lanes.outgoing.iter())
                .enumerate()
            {
                for ob in lane.obstacles {
                    let h = hash3(slot, lane_idx as i64, ob.style.id);
                    let p = (h % 10_000) as f32 / 10_000.0;
                    if p < ob.frequency {
                        let pos = self.obstacle_seed_m + (h as u64 % SLOT_M as u64) as f32;
                        self.obstacles.push(SpawnedObstacle {
                            style: ob.style,
                            effects: ob.effects,
                            crash: ob.has_crash(),
                            pos_m: pos,
                            lane_idx,
                            triggered: false,
                        });
                    }
                }
            }
            self.obstacle_seed_m += SLOT_M;
        }

        let cutoff = self.player_pos_m - Config::AI_DESPAWN_BEHIND_M;
        self.obstacles.retain(|o| o.pos_m > cutoff);
    }

    fn check_obstacle_crossings(&mut self) {
        let p_front = self.player_pos_m;
        let p_back = self.player_pos_m - Config::CAR_HEIGHT_ROWS as f32 * Config::METERS_PER_ROW;

        let mut triggered: Vec<(&'static str, &'static [ObstacleEffect])> = Vec::new();
        for obs in self.obstacles.iter_mut() {
            if obs.triggered || obs.lane_idx != self.player_lane_idx { continue; }
            if obs.pos_m >= p_back && obs.pos_m <= p_front {
                obs.triggered = true;
                triggered.push((obs.style.label, obs.effects));
            }
        }

        for (label, effects) in triggered {
            self.push_effect(label);
            for &eff in effects {
                self.apply_effect(eff);
            }
        }
    }

    fn apply_effect(&mut self, eff: ObstacleEffect) {
        match eff {
            ObstacleEffect::Crash => {
                self.phase = Phase::Dead;
                self.record_best();
            }
            ObstacleEffect::SpeedChange { affect } => {
                self.player_speed_kmh = (self.player_speed_kmh * (1.0 + affect)).max(0.0);
            }
            ObstacleEffect::BlockGas { cooldown_ms } => {
                self.gas_blocked_until =
                    Some(Instant::now() + Duration::from_millis(cooldown_ms as u64));
            }
            ObstacleEffect::BlockBrakes { cooldown_ms } => {
                self.brake_blocked_until =
                    Some(Instant::now() + Duration::from_millis(cooldown_ms as u64));
            }
            ObstacleEffect::BlockWheels { cooldown_ms } => {
                self.wheel_blocked_until =
                    Some(Instant::now() + Duration::from_millis(cooldown_ms as u64));
            }
        }
    }

    /// Push a labelled effect into the right-panel log, capping at capacity.
    fn push_effect(&mut self, label: &'static str) {
        self.recent_effects.push_front(RecentEffect { label, at: Instant::now() });
        self.recent_effects.truncate(Config::RECENT_EFFECTS_CAPACITY);
    }

    // ─── Geometry helpers (used by ui.rs) ────────────────────────────────────

    pub fn track_to_screen_row(&self, track_pos_m: f32) -> i32 {
        let offset_m = self.player_pos_m - track_pos_m;
        Config::PLAYER_TOP_ROW as i32 + (offset_m / Config::METERS_PER_ROW) as i32
    }

    pub fn progress_pct(&self) -> f32 {
        let Some(track) = self.active_track else { return 0.0; };
        let total_m = track.total_distance_km() * 1000.0 * self.distance_scale();
        if total_m == 0.0 { return 0.0; }
        (self.player_pos_m / total_m * 100.0).clamp(0.0, 100.0)
    }

    pub fn elapsed_formatted(&self) -> String {
        let total = self.elapsed_s as u32;
        let mins = total / 60;
        let secs = total % 60;
        let tenth = ((self.elapsed_s - total as f32) * 10.0) as u32;
        format!("{}:{:02}.{}", mins, secs, tenth)
    }
}

// ─── Helpers ────────────────────────────────────────────────────────────────

fn pick_car_height(cars: &[super::track::Car], rng: &mut impl Rng) -> u8 {
    if cars.is_empty() { return 3; }
    let total: f32 = cars.iter().map(|c| c.incidence).sum();
    if total <= 0.0 { return cars[0].height; }
    let mut r: f32 = rng.r#gen::<f32>() * total;
    for c in cars {
        if r < c.incidence { return c.height; }
        r -= c.incidence;
    }
    cars[cars.len() - 1].height
}

/// Cheap deterministic 3-input hash for obstacle placement seeding.
/// Exported to `ui.rs` for scenery placement (avoids duplicate).
pub(super) fn hash3(a: i64, b: i64, c: i64) -> u64 {
    let mut x = (a as u64).wrapping_mul(0x9E3779B97F4A7C15);
    x ^= (b as u64).wrapping_mul(0xC2B2AE3D27D4EB4F);
    x = x.rotate_left(31);
    x ^= (c as u64).wrapping_mul(0x165667B19E3779F9);
    x.wrapping_mul(0x94D049BB133111EB)
}

// ─── Unit tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::arcade::racer::tracks::DEFAULT_TRACK;

    #[test]
    fn picker_starts_with_zero_score() {
        let s = State::new();
        assert_eq!(s.screen, RacerScreen::Picker);
        assert_eq!(s.best_score, 0);
    }

    #[test]
    fn start_track_moves_to_racing() {
        let mut s = State::new();
        s.start_track(DEFAULT_TRACK);
        assert_eq!(s.screen, RacerScreen::Racing);
        assert_eq!(s.current_stage_idx, 0);
        assert_eq!(
            s.player_lane_idx,
            DEFAULT_TRACK.stages[0].road.lanes.player_start_idx()
        );
    }

    #[test]
    fn move_left_then_right_returns_to_origin() {
        let mut s = State::new();
        s.start_track(DEFAULT_TRACK);
        let start = s.player_lane_idx;
        s.move_left();
        assert!(s.player_lane_idx < start);
        s.move_right();
        assert_eq!(s.player_lane_idx, start);
    }

    #[test]
    fn cannot_drive_above_lane_max_for_long() {
        let mut s = State::new();
        s.start_track(DEFAULT_TRACK);
        let lane = s.current_lane_cfg().unwrap();
        s.player_speed_kmh = lane.own_max_speed + 50.0;
        for _ in 0..30 {
            s.tick();
        }
        let lane = s.current_lane_cfg().unwrap();
        assert!(s.player_speed_kmh <= lane.own_max_speed + 1.0);
    }
}
