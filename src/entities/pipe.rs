use std::time::{Duration, Instant};

use crate::global::{draw_element, size, FlappyState, GameMenu, GRID_SIZE_X, GRID_SIZE_Y};
use macroquad::prelude::{rand::*, *};
use macroquad_text::Fonts;

const PIPE_CHAR: char = '#';
const PIPE_SPACE: i32 = 4;
const PIPE_SIZE: i32 = 3;
const PIPE_SPEED: f32 = 2.;

#[derive(Clone)]
pub struct GamePipe {
    pipes: Vec<Pipe>,
    last_state: GameMenu,
    curr_spawn_time: Instant,
    spawn_time: Duration,
}

impl GamePipe {
    pub fn new() -> Self {
        Self {
            pipes: Vec::new(),
            last_state: GameMenu::Main,
            curr_spawn_time: Instant::now(),
            spawn_time: Duration::from_secs_f32(5.),
        }
    }

    pub fn draw(&mut self, state: &mut FlappyState, font: &Fonts) {
        if self.last_state == GameMenu::GameOver && state.menu == GameMenu::Playing {
            self.pipes.clear();
            self.curr_spawn_time = Instant::now();
        }
        if self.last_state != state.menu {
            self.last_state = state.menu.clone();
        }
        if state.menu != GameMenu::Playing {
            return;
        }

        // Calculate duration for spawn pipe
        if self.curr_spawn_time.elapsed() >= self.spawn_time {
            self.pipes.push(Pipe::new());
            self.curr_spawn_time = Instant::now();
        }
        self.pipes.retain_mut(|pipe| {
            let pos_x = pipe.draw(state, font);
            !(pos_x + (GRID_SIZE_X * PIPE_SIZE as f32 * 2.) < 0.)
        });
    }
}

#[derive(Clone)]
pub struct Pipe {
    position: Vec2,
    top_size: i32,
    bottom_size: i32,
    bottom_start: i32,
    scored: bool,
}

impl Pipe {
    pub fn new() -> Self {
        let Vec2 { x, y } = size();

        let rnd_y = gen_range(5, 10) as f32 * GRID_SIZE_Y;
        let real_space = PIPE_SPACE as f32 * 2. * GRID_SIZE_Y;

        let top_size = rnd_y + real_space;
        let bottom_start = top_size + real_space;
        let mut position = Vec2::ZERO;
        position.x = (x + (PIPE_SIZE as f32 * GRID_SIZE_X * 3.)).round();

        Self {
            position,
            scored: false,
            top_size: top_size as i32,
            bottom_size: y as i32,
            bottom_start: bottom_start as i32,
        }
    }

    pub fn draw(&mut self, state: &mut FlappyState, font: &Fonts) -> f32 {
        self.position.x -= (get_frame_time() * GRID_SIZE_X * PIPE_SPEED).round();
        for x in 0..PIPE_SIZE {
            for t in 0..self.top_size {
                let pos = Vec2 {
                    x: self.position.x + (x as f32 * GRID_SIZE_X),
                    y: self.position.y + t as f32,
                };
                let pos = draw_element(font, pos, PIPE_CHAR, GREEN);
                if state.player_pos == pos {
                    state.menu = GameMenu::GameOver;
                }
            }
            for b in self.bottom_start..self.bottom_size as i32 {
                let pos = Vec2 {
                    x: self.position.x + (x as f32 * GRID_SIZE_X),
                    y: self.position.y + b as f32,
                };
                let pos = draw_element(font, pos, PIPE_CHAR, GREEN);
                if state.player_pos == pos {
                    state.menu = GameMenu::GameOver;
                }
            }
        }

        let x = (self.position.x / GRID_SIZE_X).round() * GRID_SIZE_X;

        let is_player_x = state.player_pos.x >= x && state.player_pos.x <= x + (3. * GRID_SIZE_X);
        if is_player_x && state.player_pos.y <= self.top_size as f32 || // top
            is_player_x && state.player_pos.y >= self.bottom_start as f32 { // bottom
            state.menu = GameMenu::GameOver;
        }

        if x + (4. * GRID_SIZE_X) == state.player_pos.x
            && state.menu == GameMenu::Playing
            && !self.scored
        {
            state.score += 1;
            self.scored = true;
        }

        x
    }
}
