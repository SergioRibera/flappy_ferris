use self::{pipe::GamePipe, player::Player, ui::GameUI};
use crate::global::FlappyState;
use macroquad::prelude::*;
use macroquad_text::Fonts;

mod pipe;
mod player;
mod ui;

pub struct Game<'a> {
    font: &'a Fonts,
    player: Player,
    ui: GameUI,
    pipe: GamePipe,
}

impl<'a> Game<'a> {
    pub fn new(font: &'a Fonts) -> Self {
        Self {
            font,
            player: Player::new(),
            ui: GameUI::new(),
            pipe: GamePipe::new(),
        }
    }

    pub fn draw_entities(&mut self, state: &mut FlappyState) {
        let last_keycode = get_last_key_pressed();
        let is_trigger_pressed = {
            let is_touch = touches().iter().any(|t| t.phase == TouchPhase::Started);
            let space_press = matches!(last_keycode, Some(KeyCode::Space));
            is_touch || space_press
        };
        // self.border.draw(state, self.font);
        self.pipe.draw(state, self.font);
        self.player.draw(state, is_trigger_pressed, self.font);
        self.ui.draw(state, &last_keycode, self.font);
    }
}
