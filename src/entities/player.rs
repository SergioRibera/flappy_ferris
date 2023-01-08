use macroquad::prelude::*;
use macroquad_text::Fonts;

use crate::global::{FlappyState, GameMenu, GRID_SIZE_X, GRID_SIZE_Y, draw_element_with_bg};

const PLAYER_SPEED: f32 = 9.8;

pub struct Player {
    avatar: char,
    position: Vec2,
    initial_position: Vec2,
}

impl Player {
    pub fn new() -> Self {
        let height: f32 = screen_height();
        let initial_position = Vec2 {
            x: GRID_SIZE_X * 5.,
            y: (((height * 0.5) / GRID_SIZE_Y).round() * GRID_SIZE_Y),
        };
        Self {
            avatar: '🦀',
            // avatar: '🦀',
            // avatar: 'F',
            position: initial_position.clone(),
            initial_position,
        }
    }

    fn jump(&mut self) {
        self.position.y -= GRID_SIZE_Y * 3.;
    }

    pub fn draw(&mut self, state: &mut FlappyState, is_trigger_pressed: bool, font: &Fonts) {
        match state.menu {
            GameMenu::Main | GameMenu::GameOver => {
                if is_trigger_pressed {
                    self.jump();
                    self.position = self.initial_position;
                    state.score = 0;
                    state.menu = GameMenu::Playing;
                }
            }
            GameMenu::Pause => {
                if is_trigger_pressed {
                    self.jump();
                    state.menu = GameMenu::Playing;
                }
            }
            GameMenu::Playing => {
                if self.position.y < (screen_height() - GRID_SIZE_Y)
                    && self.position.y > GRID_SIZE_Y
                {
                    self.position.y += get_frame_time() * GRID_SIZE_Y * PLAYER_SPEED;
                } else {
                    state.menu = GameMenu::GameOver;
                }
                if is_trigger_pressed {
                    self.jump();
                }
            }
            GameMenu::Presentation  => return,
        }
        state.player_pos = self.position;

        // draw character
        draw_element_with_bg(font, self.position, self.avatar, RED);
    }
}
