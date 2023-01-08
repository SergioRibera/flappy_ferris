use std::time::{Duration, Instant};

use macroquad::prelude::*;
use macroquad_text::Fonts;

use crate::global::{
    draw_element_with_bg, lerp_color, size, FlappyState, GameMenu, GRID_SIZE_X, GRID_SIZE_Y,
};

pub struct GameUI {
    curr_time: Instant,
    duration: Duration,
    presentation_showed: i8,
    a: Color,
    b: Color,
}

impl GameUI {
    pub fn new() -> Self {
        Self {
            curr_time: Instant::now(),
            duration: Duration::from_secs(5),
            presentation_showed: 0,
            a: Color::from_rgba(255, 255, 255, 0),
            b: WHITE,
        }
    }

    fn draw_text(&self, text: &str, position: Vec2, font: &Fonts, color: Color) {
        let width = screen_width();
        for (i, value) in text.chars().enumerate() {
            let elem_pos = position
                + Vec2 {
                    x: (GRID_SIZE_X * i as f32),
                    y: 0.,
                };
            let elem_pos = Vec2 {
                x: elem_pos.x % width,
                y: elem_pos.y + elem_pos.x / width,
            };
            draw_element_with_bg(font, elem_pos, value, color);
        }
    }

    fn draw_center_text(&self, text: &str, position: Vec2, font: &Fonts, color: Color) {
        let dims = font.measure_text(text, GRID_SIZE_Y as u16);
        let position = position
            - Vec2 {
                x: dims.width * 0.5f32,
                y: dims.height * 0.5f32,
            };
        self.draw_text(text, position, font, color);
    }

    fn draw_main(&self, size: Vec2, font: &Fonts) {
        let Vec2 { x: w, y: h } = size;
        let start_game_msg = if cfg!(target_os = "android") {
            "Tap to play"
        } else {
            self.draw_center_text(
                "Press q to close game",
                Vec2 {
                    x: w * 0.5,
                    y: (h * 0.5) + (GRID_SIZE_Y * 5.),
                },
                font,
                WHITE,
            );
            self.draw_center_text(
                "Press p to toggle pause",
                Vec2 {
                    x: w * 0.5,
                    y: (h * 0.5) + (GRID_SIZE_Y * 4.),
                },
                font,
                WHITE,
            );
            "Press SPACE to start play"
        };
        self.draw_center_text(
            start_game_msg,
            Vec2 {
                x: w * 0.5,
                y: (h * 0.5) + (GRID_SIZE_Y * 3.),
            },
            font,
            WHITE,
        );
        self.draw_center_text(
            " Flappy Ferris ",
            Vec2 {
                x: w * 0.5,
                y: h * 0.5,
            },
            font,
            WHITE,
        );
    }

    fn draw_stop(&self, title: &str, msg: &str, size: Vec2, font: &Fonts) {
        let Vec2 { x: w, y: h } = size;
        self.draw_center_text(
            msg,
            Vec2 {
                x: w * 0.5,
                y: (h * 0.5) + (GRID_SIZE_Y * 3.),
            },
            font,
            WHITE,
        );
        self.draw_center_text(
            title,
            Vec2 {
                x: w * 0.5,
                y: (h * 0.5),
            },
            font,
            WHITE,
        );
    }

    fn draw_game_ui(&self, score: u32, size: Vec2, font: &Fonts) {
        let score_str = format!(" Score: {} ", score);

        self.draw_center_text(
            &score_str,
            Vec2 {
                x: size.x * 0.5,
                y: GRID_SIZE_Y,
            },
            font,
            WHITE,
        );
    }

    fn draw_presentation(&mut self, state: &mut FlappyState, size: Vec2, font: &Fonts) {
        let Vec2 { x: w, y: h } = size;

        if self.curr_time.elapsed() >= self.duration {
            self.presentation_showed += 1;
            self.b = Color::from_rgba(255, 255, 255, 0);
            self.a = WHITE;
            self.curr_time = Instant::now();
        }

        if self.presentation_showed == 2 {
            state.menu = GameMenu::Main;
            return;
        }

        let color = lerp_color(
            self.a,
            self.b,
            get_frame_time() * self.curr_time.elapsed().as_secs_f32() * 16.,
        );

        self.draw_center_text(
            "Presents",
            Vec2 {
                x: w * 0.5,
                y: (h * 0.5) + (GRID_SIZE_Y * 3.),
            },
            font,
            color,
        );
        self.draw_center_text(
            "Sergio Ribera",
            Vec2 {
                x: w * 0.5,
                y: h * 0.5,
            },
            font,
            color,
        );
    }

    pub fn draw(&mut self, state: &mut FlappyState, last_keycode: &Option<KeyCode>, font: &Fonts) {
        if let Some(key) = last_keycode {
            match key {
                KeyCode::Q => state.stop(),
                KeyCode::P => {
                    if state.menu == GameMenu::Pause {
                        state.menu = GameMenu::Playing;
                    } else {
                        state.menu = GameMenu::Pause;
                    }
                }
                _ => (),
            }
        }
        let size = size();

        match state.menu {
            GameMenu::Presentation => self.draw_presentation(state, size, font),
            GameMenu::Main => self.draw_main(size, font),
            GameMenu::Playing => self.draw_game_ui(state.score, size, font),
            GameMenu::Pause => self.draw_stop("Pause", "Press SPACE to continue play", size, font),
            GameMenu::GameOver => self.draw_stop(
                "Game Over",
                &format!("Your Score: {}", state.score),
                size,
                font,
            ),
        }
    }
}
