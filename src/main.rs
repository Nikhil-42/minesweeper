mod minesweeper;

use macroquad::{miniquad::window::set_window_size, prelude::*};
use minesweeper::{GameState, Minesweeper, Point};

struct Config {
    header_size: f32,
    dimensions: (usize, usize),
    tile_size: f32,
}

impl Config {
    fn width(&self) -> f32 {
        self.dimensions.0 as f32 * self.tile_size
    }

    fn height(&self) -> f32 {
        self.dimensions.1 as f32 * self.tile_size + self.header_size
    }
}

fn from_screen(coords: (f32, f32), config: &Config) -> Option<Point> {
    let x = coords.0 / config.tile_size;
    let y = (coords.1 - config.header_size) / config.tile_size;
    if y < 0.0 || x >= config.dimensions.0 as f32 || y >= config.dimensions.1 as f32 {
        return None; // Out of bounds
    }
    let x = x.floor() as usize;
    let y = y.floor() as usize;
    Some((x, y))
}

fn to_screen(coords: Point, config: &Config) -> (f32, f32) {
    let x = coords.0 as f32 * config.tile_size;
    let y = config.header_size + coords.1 as f32 * config.tile_size;
    (x, y)
}

#[macroquad::main("Minesweeper")]
async fn main() {
    let config = Config {
        header_size: 50.0,
        dimensions: (24, 20),
        tile_size: 48.0,
    };

    let flag_tex = load_texture("res/flag.png").await.unwrap();
    let default_tex = load_texture("res/default.png").await.unwrap();
    let revealed_tex = load_texture("res/revealed.png").await.unwrap();
    let mine_tex = load_texture("res/mine.png").await.unwrap();

    let numbers_tex = [
        load_texture("res/1.png").await.unwrap(),
        load_texture("res/2.png").await.unwrap(),
        load_texture("res/3.png").await.unwrap(),
        load_texture("res/4.png").await.unwrap(),
        load_texture("res/5.png").await.unwrap(),
        load_texture("res/6.png").await.unwrap(),
        load_texture("res/7.png").await.unwrap(),
        load_texture("res/8.png").await.unwrap(),
    ];

    flag_tex.set_filter(FilterMode::Nearest);
    default_tex.set_filter(FilterMode::Nearest);
    revealed_tex.set_filter(FilterMode::Nearest);
    mine_tex.set_filter(FilterMode::Nearest);
    for tex in &numbers_tex {
        tex.set_filter(FilterMode::Nearest);
    }

    let mut minesweeper = Minesweeper::new(config.dimensions, 99);
    let mut start_time = macroquad::prelude::get_time();
    let mut end_time: Option<f64> = None;

    loop {
        set_window_size(config.width() as u32, config.height() as u32);
        let mouse_pos = mouse_position();
        let coords = from_screen(mouse_pos, &config);

        if let Some(coords) = coords {
            match minesweeper.current_state() {
                GameState::Playing => {
                    if is_mouse_button_released(MouseButton::Left) {
                        minesweeper.reveal_tile(coords);
                    }
                    if is_mouse_button_released(MouseButton::Right) {
                        minesweeper.toggle_flag(coords);
                    }
                }
                GameState::Win | GameState::Lose => {
                    if is_mouse_button_released(MouseButton::Left)
                        || is_mouse_button_released(MouseButton::Right)
                    {
                        minesweeper = Minesweeper::new(config.dimensions, 99);
                        start_time = macroquad::prelude::get_time();
                    }
                }
            }
        }

        // Spawn protection 
        if (minesweeper.total_revealed() == 0 && *minesweeper.current_state() == GameState::Lose) || (minesweeper.total_revealed() > 0 && minesweeper.total_revealed() < 9) {
            minesweeper = Minesweeper::new(config.dimensions, 99);
            start_time = macroquad::prelude::get_time();
            continue; 
        }

        draw_rectangle(0.0, 0.0, config.width(), config.header_size, GRAY);
        let total_flags = minesweeper.total_flags();
        let total_mines = minesweeper.total_mines();
        draw_text_ex(
            &format!("Flags: {}", if total_flags <= total_mines { total_mines - total_flags } else { 0 }),
            config.width() / 4.0,
            config.header_size * 0.75,
            TextParams {
                font_size: config.header_size as u16,
                ..Default::default()
            },
        );

        match *minesweeper.current_state() {
            GameState::Playing => {
                if end_time.is_some() {
                    end_time = None; // Reset end time when playing
                }
            }
            GameState::Win | GameState::Lose => {
                if end_time.is_none() {
                    end_time = Some(macroquad::prelude::get_time());
                }
            }
        }
        draw_text_ex(
            &format!("Time: {:.0}", if let Some(end_time) = end_time { end_time } else { macroquad::prelude::get_time() } - start_time),
            config.width() / 4.0 * 3.0 - 100.0,
            config.header_size * 0.75,
            TextParams {
                font_size: config.header_size as u16,
                ..Default::default()
            },
        );
        
        for x in 0..config.dimensions.0 {
            for y in 0..config.dimensions.1 {
                let (rect_x, rect_y) = to_screen((x, y), &config);

                draw_texture_ex(
                    &default_tex,
                    rect_x,
                    rect_y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some((config.tile_size, config.tile_size).into()),
                        ..Default::default()
                    },
                );
                if minesweeper.is_revealed((x, y)) {
                    draw_texture_ex(
                        &revealed_tex,
                        rect_x,
                        rect_y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some((config.tile_size, config.tile_size).into()),
                            ..Default::default()
                        },
                    );
                    if minesweeper.mine_count((x, y)) > 0 {
                        draw_texture_ex(
                            &numbers_tex[minesweeper.mine_count((x, y)) as usize - 1],
                            rect_x,
                            rect_y,
                            WHITE,
                            DrawTextureParams {
                                dest_size: Some((config.tile_size, config.tile_size).into()),
                                ..Default::default()
                            },
                        );
                    }
                }

                match minesweeper.current_state() {
                    GameState::Playing => {
                        if minesweeper.is_flagged((x, y)) {
                            draw_texture_ex(
                                &flag_tex,
                                rect_x,
                                rect_y,
                                WHITE,
                                DrawTextureParams {
                                    dest_size: Some((config.tile_size, config.tile_size).into()),
                                    ..Default::default()
                                },
                            );
                        }
                    }
                    GameState::Win => {
                        draw_text("You Win!", 10.0, 30.0, 20.0, GREEN);
                    }
                    GameState::Lose => {
                        if minesweeper.is_flagged((x, y)) && minesweeper.is_mine((x, y)) {
                            draw_texture_ex(
                                &flag_tex,
                                rect_x,
                                rect_y,
                                WHITE,
                                DrawTextureParams {
                                    dest_size: Some((config.tile_size, config.tile_size).into()),
                                    ..Default::default()
                                },
                            );
                        } else if minesweeper.is_flagged((x, y)) {
                            draw_texture_ex(
                                &default_tex,
                                rect_x,
                                rect_y,
                                RED,
                                DrawTextureParams {
                                    dest_size: Some((config.tile_size, config.tile_size).into()),
                                    ..Default::default()
                                },
                            );
                        } else if minesweeper.is_mine((x, y)) {
                            draw_texture_ex(
                                &mine_tex,
                                rect_x,
                                rect_y,
                                WHITE,
                                DrawTextureParams {
                                    dest_size: Some((config.tile_size, config.tile_size).into()),
                                    ..Default::default()
                                },
                            );
                        }
                    }
                }
            }
        }

        if let Some(coords) = coords {
            let tile_pos = to_screen(coords, &config);
            draw_rectangle(
                tile_pos.0,
                tile_pos.1,
                config.tile_size,
                config.tile_size,
                Color::new(0.0, 0.0, 0.0, 0.1),
            );
        }
        next_frame().await;
    }
}
