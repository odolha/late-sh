use std::time::{Duration, Instant};

use rand::Rng;

// ─── Configurable constants ──────────────────────────────────────────────────

pub struct Config;

impl Config {
    /// Fixed visible road height in terminal rows. Kept constant to prevent
    /// "wider terminal = longer sight distance" cheating.
    pub const VISIBLE_ROWS: u16 = 50;
    /// Visual height (in rows) per car size class.
    pub const CAR_HEIGHT_SM: u16 = 3;
    pub const CAR_HEIGHT_MD: u16 = 4;
    pub const CAR_HEIGHT_LG: u16 = 6;
    pub const CAR_HEIGHT_XL: u16 = 8;
    /// Player car height (always Sm).
    pub const CAR_HEIGHT_ROWS: u16 = Self::CAR_HEIGHT_SM;
    /// Spawn weights per size class. Should sum to 1.0.
    pub const SPAWN_WEIGHT_SM: f32 = 0.4;
    pub const SPAWN_WEIGHT_MD: f32 = 0.3;
    pub const SPAWN_WEIGHT_LG: f32 = 0.2;
    pub const SPAWN_WEIGHT_XL: f32 = 0.1;
    /// Width of each lane in chars.
    pub const LANE_WIDTH: u16 = 5;
    /// Number of oncoming-traffic lanes (rendered on the left side of the road).
    pub const LANES_ONCOMING: usize = 2;
    /// Number of same-direction lanes (rendered on the right side of the road).
    pub const LANES_SAME_DIR: usize = 2;
    /// Total number of lanes.
    pub const TOTAL_LANES: usize = Self::LANES_ONCOMING + Self::LANES_SAME_DIR;
    /// Total road width: borders + all lanes + center group divider.
    pub const TOTAL_ROAD_WIDTH: u16 =
        1 + (Self::TOTAL_LANES as u16) * Self::LANE_WIDTH + 1 + 1;
    /// Minimap width: borders + 1 col per lane + group divider.
    pub const MINI_W: u16 =
        1 + Self::LANES_ONCOMING as u16 + 1 + Self::LANES_SAME_DIR as u16 + 1;
    /// Total track length in meters (10 km).
    pub const TRACK_LENGTH_M: f32 = 10_000.0;
    /// Meters represented by one terminal row.
    pub const METERS_PER_ROW: f32 = 3.0;
    /// Rows of road visible behind (below) the player car.
    pub const PLAYER_BOTTOM_MARGIN: u16 = 4;
    /// Screen row of the top (front) of the player car.
    pub const PLAYER_TOP_ROW: u16 = Self::VISIBLE_ROWS - Self::CAR_HEIGHT_ROWS - Self::PLAYER_BOTTOM_MARGIN;
    /// Player starting speed in km/h.
    pub const PLAYER_START_SPEED_KMH: f32 = 50.0;
    /// Maximum player speed in km/h.
    pub const PLAYER_MAX_SPEED_KMH: f32 = 250.0;
    /// Minimum player speed in km/h (full stop allowed).
    pub const PLAYER_MIN_SPEED_KMH: f32 = 0.0;
    /// Acceleration in km/h per second while holding accelerate.
    pub const ACCEL_KMH_PER_S: f32 = 110.0;
    /// Braking deceleration in km/h per second.
    pub const DECEL_KMH_PER_S: f32 = 160.0;
    /// Passive coasting deceleration in km/h per second.
    pub const COAST_DECEL_KMH_PER_S: f32 = 0.0;
    /// Target number of AI cars on road at any time.
    pub const AI_CAR_COUNT: usize = 12;
    /// How far ahead the minimap shows (2 visible screens).
    /// Cars always spawn within this range so they appear on the minimap immediately.
    pub const MINIMAP_RANGE_M: f32 = 2.0 * Self::VISIBLE_ROWS as f32 * Self::METERS_PER_ROW;
    /// Fixed speed for same-direction NPC cars (km/h).
    pub const AI_SAME_DIR_SPEED_KMH: f32 = 90.0;
    /// Fixed speed for oncoming NPC cars (km/h).
    pub const AI_ONCOMING_SPEED_KMH: f32 = 50.0;
    /// Minimum center-to-center spawn gap between cars in the same lane/direction.
    /// Must exceed largest car length (XL = 8 rows × 3 m = 24 m) plus a small buffer.
    pub const AI_MIN_SEPARATION_M: f32 = 32.0;
    /// Distance behind player before an AI car is despawned.
    pub const AI_DESPAWN_BEHIND_M: f32 = 200.0;
    /// Meters visible ahead of player car front = PLAYER_TOP_ROW * METERS_PER_ROW.
    pub const VISIBLE_AHEAD_M: f32 = Self::PLAYER_TOP_ROW as f32 * Self::METERS_PER_ROW; // 111 m
    /// Initial score: 1000 * track length in meters.
    pub const INITIAL_SCORE: f32 = 1_000.0 * Self::TRACK_LENGTH_M; // 10 000 000
    /// Score decrease per second, calibrated so finishing at START_SPEED yields ~0.
    /// Time at start speed = TRACK_LEN / (START_SPEED / 3.6).
    /// Decay = INITIAL_SCORE / time_at_start_speed.
    pub const SCORE_DECAY_PER_S: f32 =
        Self::INITIAL_SCORE * (Self::PLAYER_START_SPEED_KMH / 3.6) / Self::TRACK_LENGTH_M;
    /// Duration of one world tick in seconds (15 FPS).
    pub const TICK_DT: f32 = 1.0 / 15.0;
    /// How long a held-key input stays active after the last key event.
    /// Key repeat fires every ~30ms, so 150ms gives ~5 repeat events of margin.
    pub const INPUT_HOLD_MS: u64 = 150;
    /// Minimum terminal width needed to render game + stats.
    pub const MIN_TERMINAL_WIDTH: u16 =
        Self::MINI_W + 2 + 6 + Self::TOTAL_ROAD_WIDTH + 8 + 2 + 28; // mini+gap+trees+road+trees+gap+stats
    /// Minimum terminal height needed (road + bottom bar).
    pub const MIN_TERMINAL_HEIGHT: u16 = Self::VISIBLE_ROWS + 5;
}

// ─── Types ───────────────────────────────────────────────────────────────────

/// Lane index. 0..LANES_ONCOMING are oncoming lanes (left side of road);
/// LANES_ONCOMING..TOTAL_LANES are same-direction lanes (right side).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Lane(pub usize);

impl Lane {
    pub fn direction(self) -> TrafficDir {
        if self.0 < Config::LANES_ONCOMING {
            TrafficDir::Oncoming
        } else {
            TrafficDir::Same
        }
    }
    /// First (leftmost) same-direction lane — player's starting lane.
    pub fn player_start() -> Self {
        Lane(Config::LANES_ONCOMING)
    }
    pub fn is_oncoming(self) -> bool {
        self.direction() == TrafficDir::Oncoming
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TrafficDir {
    /// Car moves in the same direction as the player (position increases).
    Same,
    /// Car moves toward the player (position decreases).
    Oncoming,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CarSize {
    Sm,
    Md,
    Lg,
    Xl,
}

impl CarSize {
    pub fn height_rows(self) -> u16 {
        match self {
            CarSize::Sm => Config::CAR_HEIGHT_SM,
            CarSize::Md => Config::CAR_HEIGHT_MD,
            CarSize::Lg => Config::CAR_HEIGHT_LG,
            CarSize::Xl => Config::CAR_HEIGHT_XL,
        }
    }

    pub fn pick_weighted<R: Rng>(rng: &mut R) -> Self {
        let r: f32 = rng.r#gen();
        let mut acc = Config::SPAWN_WEIGHT_SM;
        if r < acc { return CarSize::Sm; }
        acc += Config::SPAWN_WEIGHT_MD;
        if r < acc { return CarSize::Md; }
        acc += Config::SPAWN_WEIGHT_LG;
        if r < acc { return CarSize::Lg; }
        CarSize::Xl
    }
}

#[derive(Clone, Debug)]
pub struct AiCar {
    /// Center track position in meters.
    pub pos_m: f32,
    pub speed_kmh: f32,
    pub lane: Lane,
    pub direction: TrafficDir,
    pub size: CarSize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlayerInput {
    Accelerate,
    Brake,
    Handbrake,
    None,
}

pub enum Phase {
    Playing,
    Finished { elapsed_s: f32, score: i64 },
    Dead,
}

// ─── Game state ───────────────────────────────────────────────────────────────

pub struct State {
    /// Player's track position in meters (front of car).
    pub player_pos_m: f32,
    /// Current speed in km/h.
    pub player_speed_kmh: f32,
    /// Current lane.
    pub player_lane: Lane,
    /// Held input this tick.
    pub input: PlayerInput,
    /// When the current input was last refreshed by a key event.
    pub input_last_set: Option<Instant>,
    pub ai_cars: Vec<AiCar>,
    /// Ticks remaining before the next spawn event. Creates natural traffic gaps.
    pub spawn_cooldown_ticks: u32,
    pub elapsed_s: f32,
    pub score: i64,
    /// Best score this session (no DB persistence in v1).
    pub best_score: i64,
    pub phase: Phase,
    pub is_paused: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            player_pos_m: 0.0,
            player_speed_kmh: Config::PLAYER_START_SPEED_KMH,
            player_lane: Lane::player_start(),
            input: PlayerInput::None,
            input_last_set: None,
            ai_cars: Vec::new(),
            spawn_cooldown_ticks: 0,
            elapsed_s: 0.0,
            score: Config::INITIAL_SCORE as i64,
            best_score: 0,
            phase: Phase::Playing,
            is_paused: false,
        }
    }

    pub fn restart(&mut self) {
        let best = self.best_score;
        *self = Self::new();
        self.best_score = best;
    }

    pub fn is_playing(&self) -> bool {
        matches!(self.phase, Phase::Playing)
    }

    pub fn toggle_pause(&mut self) {
        if self.is_playing() {
            self.is_paused = !self.is_paused;
        }
    }

    pub fn move_left(&mut self) {
        if !self.is_playing() || self.is_paused {
            return;
        }
        if self.player_lane.0 > 0 {
            self.player_lane = Lane(self.player_lane.0 - 1);
        }
    }

    pub fn move_right(&mut self) {
        if !self.is_playing() || self.is_paused {
            return;
        }
        if self.player_lane.0 + 1 < Config::TOTAL_LANES {
            self.player_lane = Lane(self.player_lane.0 + 1);
        }
    }

    pub fn tick(&mut self) {
        if !self.is_playing() || self.is_paused {
            return;
        }
        let dt = Config::TICK_DT;

        // Expire held input if no key event arrived recently.
        // Key repeat fires every ~30ms while held, so 150ms gives ample margin.
        // When the key is released, repeat stops and input clears after INPUT_HOLD_MS.
        if let Some(t) = self.input_last_set {
            if t.elapsed() > Duration::from_millis(Config::INPUT_HOLD_MS) {
                self.input = PlayerInput::None;
                self.input_last_set = None;
            }
        }

        let delta = match self.input {
            PlayerInput::Accelerate => Config::ACCEL_KMH_PER_S * dt,
            PlayerInput::Brake => -(Config::DECEL_KMH_PER_S * dt),
            PlayerInput::Handbrake => -(Config::DECEL_KMH_PER_S * 2.0 * dt),
            PlayerInput::None => -(Config::COAST_DECEL_KMH_PER_S * dt),
        };
        self.player_speed_kmh = (self.player_speed_kmh + delta)
            .clamp(Config::PLAYER_MIN_SPEED_KMH, Config::PLAYER_MAX_SPEED_KMH);

        // Advance player
        self.player_pos_m += (self.player_speed_kmh / 3.6) * dt;
        self.elapsed_s += dt;
        self.score = (Config::INITIAL_SCORE - Config::SCORE_DECAY_PER_S * self.elapsed_s)
            .max(0.0) as i64;

        // Update AI cars
        self.update_ai(dt);

        // Collision check
        if self.check_collision() {
            self.phase = Phase::Dead;
            return;
        }

        // Finish check
        if self.player_pos_m >= Config::TRACK_LENGTH_M {
            let s = self.score;
            self.best_score = self.best_score.max(s);
            self.phase = Phase::Finished {
                elapsed_s: self.elapsed_s,
                score: s,
            };
            return;
        }

        self.manage_ai();
    }

    /// Returns true if spawning a car at `pos` in `lane`/`direction` won't immediately overlap.
    fn spawn_clear(&self, pos: f32, lane: Lane, direction: TrafficDir) -> bool {
        self.ai_cars.iter().all(|o| {
            o.lane != lane || o.direction != direction
                || (o.pos_m - pos).abs() >= Config::AI_MIN_SEPARATION_M
        })
    }

    fn update_ai(&mut self, dt: f32) {
        for car in &mut self.ai_cars {
            match car.direction {
                TrafficDir::Same => car.pos_m += (car.speed_kmh / 3.6) * dt,
                TrafficDir::Oncoming => car.pos_m -= (car.speed_kmh / 3.6) * dt,
            }
        }
    }

    fn check_collision(&self) -> bool {
        // Pixel-perfect: compute the exact screen row range each car occupies
        // (mirroring the render logic) and check for integer overlap.
        let p_top = Config::PLAYER_TOP_ROW as i32;
        let p_bot = p_top + Config::CAR_HEIGHT_ROWS as i32 - 1;

        for car in &self.ai_cars {
            if car.lane != self.player_lane {
                continue;
            }
            let center = self.track_to_screen_row(car.pos_m);
            let h = car.size.height_rows() as i32;
            let top = center - h / 2;
            let bot = top + h - 1;
            if top <= p_bot && bot >= p_top {
                return true;
            }
        }
        false
    }

    fn manage_ai(&mut self) {
        let player_pos = self.player_pos_m;

        self.ai_cars.retain(|car| {
            let offset = car.pos_m - player_pos;
            offset > -(Config::AI_DESPAWN_BEHIND_M)
                && car.pos_m < Config::TRACK_LENGTH_M + 200.0
        });

        // Cooldown creates natural gaps between spawn events.
        if self.spawn_cooldown_ticks > 0 {
            self.spawn_cooldown_ticks -= 1;
            return;
        }

        let current = self.ai_cars.len();
        let max = Config::AI_CAR_COUNT;
        let mut rng = rand::thread_rng();

        if current >= max {
            self.spawn_cooldown_ticks = rng.gen_range(15..90);
            return;
        }

        // Spawn 1–4 cars clustered near a random ahead position.
        let cluster = rng.gen_range(1..=((max - current).min(4)));
        let base_extra = rng.gen_range(
            0.0_f32..(Config::MINIMAP_RANGE_M - Config::VISIBLE_AHEAD_M).max(1.0),
        );
        for i in 0..cluster {
            let lane = Lane(rng.gen_range(0..Config::TOTAL_LANES));
            let direction = lane.direction();
            let speed = if direction == TrafficDir::Same {
                Config::AI_SAME_DIR_SPEED_KMH
            } else {
                Config::AI_ONCOMING_SPEED_KMH
            };
            let pos = player_pos + Config::VISIBLE_AHEAD_M + base_extra
                + i as f32 * rng.gen_range(25.0_f32..55.0);
            if pos > player_pos + Config::MINIMAP_RANGE_M {
                break;
            }
            if direction == TrafficDir::Same && pos >= Config::TRACK_LENGTH_M {
                continue;
            }
            if !self.spawn_clear(pos, lane, direction) {
                continue;
            }
            let size = CarSize::pick_weighted(&mut rng);
            self.ai_cars.push(AiCar { pos_m: pos, speed_kmh: speed, lane, direction, size });
        }

        // Random delay before next spawn event: 0–4 s at 15 fps.
        // Short cooldowns create back-to-back clusters; long ones open road.
        self.spawn_cooldown_ticks = rng.gen_range(0..60);
    }

    /// Convert a track position (meters) to a screen row.
    /// Row 0 = top (furthest ahead), VISIBLE_ROWS-1 = bottom (behind player).
    /// Returns i32; may be negative or ≥ VISIBLE_ROWS if out of view.
    pub fn track_to_screen_row(&self, track_pos_m: f32) -> i32 {
        let offset_m = self.player_pos_m - track_pos_m;
        Config::PLAYER_TOP_ROW as i32 + (offset_m / Config::METERS_PER_ROW) as i32
    }

    pub fn progress_pct(&self) -> f32 {
        (self.player_pos_m / Config::TRACK_LENGTH_M * 100.0).min(100.0)
    }

    pub fn elapsed_formatted(&self) -> String {
        let total = self.elapsed_s as u32;
        let mins = total / 60;
        let secs = total % 60;
        let tenth = ((self.elapsed_s - total as f32) * 10.0) as u32;
        format!("{}:{:02}.{}", mins, secs, tenth)
    }
}

// ─── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn track_to_screen_row_ahead_is_above_player() {
        let mut s = State::new();
        s.ai_cars.clear();
        s.player_pos_m = 500.0;
        // 99m ahead → above player_top_row
        let row = s.track_to_screen_row(599.0);
        assert!(row < Config::PLAYER_TOP_ROW as i32, "ahead car should be above player");
    }

    #[test]
    fn track_to_screen_row_player_front_is_player_top() {
        let mut s = State::new();
        s.ai_cars.clear();
        s.player_pos_m = 1000.0;
        let row = s.track_to_screen_row(1000.0);
        assert_eq!(row, Config::PLAYER_TOP_ROW as i32);
    }

    #[test]
    fn lane_direction_follows_index() {
        assert_eq!(Lane(0).direction(), TrafficDir::Oncoming);
        assert_eq!(
            Lane(Config::LANES_ONCOMING).direction(),
            TrafficDir::Same,
        );
    }

    #[test]
    fn score_starts_at_initial_and_decreases() {
        let mut s = State::new();
        s.ai_cars.clear();
        let initial = s.score;
        assert_eq!(initial, Config::INITIAL_SCORE as i64);
        s.tick();
        assert!(s.score < initial);
    }

    #[test]
    fn collision_detected_same_lane() {
        let mut s = State::new();
        s.ai_cars.clear();
        s.player_pos_m = 100.0;
        s.player_lane = Lane::player_start();
        // AI at player_pos_m + 3.0: diff = 3.0 < ahead_limit (6.0) → collision
        s.ai_cars.push(AiCar {
            pos_m: 103.0,
            speed_kmh: 60.0,
            lane: Lane::player_start(),
            direction: TrafficDir::Same,
            size: CarSize::Sm,
        });
        assert!(s.check_collision());
    }

    #[test]
    fn no_collision_ai_just_ahead() {
        let mut s = State::new();
        s.ai_cars.clear();
        s.player_pos_m = 100.0;
        s.player_lane = Lane::player_start();
        // AI at player_pos_m + 6.0: diff = 6.0, NOT < ahead_limit → no collision
        s.ai_cars.push(AiCar {
            pos_m: 106.0,
            speed_kmh: 60.0,
            lane: Lane::player_start(),
            direction: TrafficDir::Same,
            size: CarSize::Sm,
        });
        assert!(!s.check_collision());
    }

    #[test]
    fn no_collision_ai_just_behind() {
        let mut s = State::new();
        s.ai_cars.clear();
        s.player_pos_m = 100.0;
        s.player_lane = Lane::player_start();
        // AI at player_pos_m - 12.0: diff = -12.0, NOT > -behind_limit → no collision
        s.ai_cars.push(AiCar {
            pos_m: 88.0,
            speed_kmh: 60.0,
            lane: Lane::player_start(),
            direction: TrafficDir::Same,
            size: CarSize::Sm,
        });
        assert!(!s.check_collision());
    }

    #[test]
    fn no_collision_different_lane() {
        let mut s = State::new();
        s.ai_cars.clear();
        s.player_pos_m = 100.0;
        s.player_lane = Lane::player_start();
        s.ai_cars.push(AiCar {
            pos_m: 100.5,
            speed_kmh: 60.0,
            lane: Lane(0),
            direction: TrafficDir::Oncoming,
            size: CarSize::Sm,
        });
        assert!(!s.check_collision());
    }
}
