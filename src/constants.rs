// constants.rs
use macroquad::prelude::*;
use crate::ui_helper;

//wellness
pub const WELLNESS_THRIVING: f64 = 80.0;
pub const WELLNESS_HEALTHY: f64 = 60.0;
pub const WELLNESS_NEUTRAL: f64 = 40.0;
pub const WELLNESS_SICK: f64 = 20.0;
pub const WELLNESS_PENALTY_SEVERITY: f64 = 2.0; // main tuning knob

//UI scale
pub const WINDOWS_DEFAULT_WIDTH: i32 = 1280;
pub const WINDOWS_DEFAULT_LENGTH: i32 = 720;

pub const TITLE_BOX_SCALE_WIDTH: f32 = 0.25;
pub const TITLE_BOX_SCALE_HEIGHT: f32 = 0.15;

pub const BUTTON_BOX_SCALE_WIDTH: f32 = 0.2;
pub const BUTTON_BOX_SCALE_HEIGHT: f32 = 0.1;

pub const SETTING_BUTTON_BOX_SCALE_WIDTH: f32 = 0.1;
pub const SETTING_BUTTON_BOX_SCALE_HEIGHT: f32 = 0.1;

pub const PRESTIGE_BOX_SCALE_WIDTH: f32 = 0.15;
pub const PRESTIGE_BOX_SCALE_HEIGHT: f32 = 0.1;

pub const BORDER_COLOUR: Color = WHITE;

pub const STAT_WIDTH: f32 = 0.1;
pub const STAT_HEIGHT: f32 = 0.1;