use std::time::{Duration, Instant};

use device_query::{Keycode, MouseState};
use eframe::{egui, epaint::FontId};
use rand::{prelude::ThreadRng, rng};
use rdev::Button;

use crate::{
    defines::*,
    types::{AppMode, ClickButton, ClickPosition, ClickType, InteractionMode},
    utils::{
        interval_ms, move_mouse_to, press_button, release_button, sanitize_i64_string,
        sanitize_string,
    },
};

pub struct RustyAutoClickerApp {
    // Text input strings
    pub hr_str: String,
    pub min_str: String,
    pub sec_str: String,
    pub ms_str: String,
    pub click_amount_str: String,
    pub click_x_str: String,
    pub click_y_str: String,
    pub speed_min_str: String,
    pub speed_max_str: String,

    // Time
    pub last_now: Instant,
    pub frame_start: Instant,

    // Counter
    pub click_counter: u64,

    // Hotkeys
    pub key_autoclick: Option<Keycode>,
    pub key_set_coord: Option<Keycode>,
    pub key_hold: Option<Keycode>,

    // Interaction state (mutually exclusive)
    pub mode: InteractionMode,

    // The button currently held down (click-and-hold), if any
    pub held_button: Option<ClickButton>,

    // App mode
    pub app_mode: AppMode,

    // Window state
    pub hotkey_window_open: bool,
    pub window_position: egui::Pos2,

    // Key states
    pub key_pressed_autoclick: bool,
    pub key_pressed_set_coord: bool,
    pub key_pressed_esc: bool,
    pub key_pressed_hold: bool,
    pub keys_pressed: Option<Vec<Keycode>>,

    // Mouse snapshot (polled in `logic`, displayed in `ui`)
    pub mouse: MouseState,

    // Enums
    pub click_btn: ClickButton,
    pub click_type: ClickType,
    pub click_position: ClickPosition,
    // RNG
    pub rng_thread: ThreadRng,
}

impl Default for RustyAutoClickerApp {
    fn default() -> Self {
        Self {
            // Text input strings
            hr_str: DEFAULT_HR_STR.to_owned(),
            min_str: DEFAULT_MIN_STR.to_owned(),
            sec_str: DEFAULT_SEC_STR.to_owned(),
            ms_str: DEFAULT_MS_STR.to_owned(),
            click_amount_str: DEFAULT_CLICK_AMOUNT_STR.to_owned(),
            click_x_str: DEFAULT_CLICK_X_STR.to_owned(),
            click_y_str: DEFAULT_CLICK_Y_STR.to_owned(),
            speed_min_str: MOUSE_TWEEN_SPEED_MIN_PX_S.to_string(),
            speed_max_str: MOUSE_TWEEN_SPEED_MAX_PX_S.to_string(),

            // Time
            last_now: Instant::now(),
            frame_start: Instant::now(),

            // Counter
            click_counter: 0u64,

            // Hotkeys
