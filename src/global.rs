use macroquad::prelude::*;
use macroquad_text::Fonts;

#[cfg(not(target_os = "android"))]
pub const GRID_SIZE_X: f32 = 20.0;
#[cfg(not(target_os = "android"))]
pub const GRID_SIZE_Y: f32 = 30.0;

#[cfg(target_os = "android")]
pub const GRID_SIZE_X: f32 = 40.0;
#[cfg(target_os = "android")]
pub const GRID_SIZE_Y: f32 = 50.0;
pub const BACKGROUND_COLOR: Color = BLACK;

#[derive(Clone, PartialEq)]
pub enum GameMenu {
    Presentation,
    Main,
    Pause,
    Playing,
    GameOver,
}

pub struct FlappyState {
    running: bool,
    pub score: u32,
    pub menu: GameMenu,
    pub player_pos: Vec2,
}

impl Default for FlappyState {
    fn default() -> Self {
        Self {
            running: true,
            score: 0,
            menu: GameMenu::Presentation,
            player_pos: Vec2::ZERO,
        }
    }
}

impl FlappyState {
    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}

pub fn size() -> Vec2 {
    let w = screen_width().round();
    let h = screen_height().round();

    Vec2 {
        x: w - (w % GRID_SIZE_X),
        y: h - (w % GRID_SIZE_Y),
    }
}

pub fn lerp_color(x: Color, y: Color, t: f32) -> Color {
    let r = x.r + (y.r - x.r) * t;
    let g = x.g + (y.g - x.g) * t;
    let b = x.b + (y.b - x.b) * t;
    let a = x.a + (y.a - x.a) * t;
    Color::from_vec(Vec4 {
        x: r,
        y: g,
        z: b,
        w: a,
    })
}

pub fn draw_element_with_bg(font: &Fonts, position: Vec2, value: char, fg: Color) {
    let Vec2 { x: raw_x, y: raw_y } = position;
    let x: f32 = (raw_x / GRID_SIZE_X).round() * GRID_SIZE_X;
    let y: f32 = (raw_y / GRID_SIZE_Y).round() * GRID_SIZE_Y;
    macroquad::prelude::draw_rectangle(x, y, GRID_SIZE_X, GRID_SIZE_Y, BACKGROUND_COLOR);
    font.draw_text(&value.to_string(), x, y, GRID_SIZE_Y as u16, fg);
}

pub fn draw_element(font: &Fonts, position: Vec2, value: char, fg: Color) -> Vec2 {
    let Vec2 { x: raw_x, y: raw_y } = position;
    let x: f32 = (raw_x / GRID_SIZE_X).round() * GRID_SIZE_X;
    let y: f32 = (raw_y / GRID_SIZE_Y).round() * GRID_SIZE_Y;

    font.draw_text(&value.to_string(), x, y, GRID_SIZE_Y as u16, fg);
    Vec2 { x, y }
}
