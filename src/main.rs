use entities::Game;
use macroquad::prelude::*;
use macroquad_text::Fonts;

use crate::global::{FlappyState, BACKGROUND_COLOR};

mod entities;
mod global;

const FIRA_CODE: &[u8] =
    include_bytes!("../assets/fonts/Fira Code Regular Nerd Font Complete Mono.ttf");
const NOTO_EMOJI: &[u8] = include_bytes!("../assets/fonts/NotoEmoji-VariableFont_wght.ttf");

fn window_conf() -> macroquad::window::Conf {
    macroquad::window::Conf {
        window_title: "Flappy Ferris".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    macroquad::file::set_pc_assets_folder("assets");
    let mut fonts = Fonts::default();
    fonts.load_font_from_bytes(FIRA_CODE).unwrap();
    fonts.load_font_from_bytes(NOTO_EMOJI).unwrap();
    println!("Font contain crap: {}", fonts.contains('🦀'));

    let mut state = FlappyState::default();
    let mut game = Game::new(&fonts);

    while state.is_running() {
        clear_background(BACKGROUND_COLOR);

        game.draw_entities(&mut state);

        next_frame().await
    }
}
