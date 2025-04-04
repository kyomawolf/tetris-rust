use std::{collections::btree_map::Range, process::exit};

use raylib::{ffi::WindowShouldClose, prelude::*};
use Logic::{check_if_game_ends, check_if_shape_is_set, TileType};

mod Inputhandler;
mod Logic;
use renderer::SceneState;
mod renderer;

// Todo create a renderer
//  Scenes, as base class
// todo create some game logic
// todo create a sound handler

fn draw_current_scene(mut state:SceneState, render:renderer::Renderer) -> renderer::Renderer {
        match state {
             SceneState::MAINMENU => {main_menu_loop(); return render},
             SceneState::GAME => return game_loop(render),
             SceneState::AFTERGAME => return render
        }
}


fn game_loop(mut render:renderer::Renderer) -> renderer::Renderer {
    let default_width: usize = 10;
    let default_height: usize = 25;
    let mut grid = Logic::GameGrid{field:vec![vec![TileType::NONE; default_width]; default_height], width:default_width, height:default_height, marked_tiles:vec![], current_shape:Logic::TileType::NONE};
    // todo fix on window should close
    while !render.handle.window_should_close() {
        
        // clear full lines
        let full_lines = Logic::check_lines(&grid);
        if !full_lines.is_empty() {
            grid = Logic::remove_and_shrink_lines(grid, full_lines);
        }
        // generate a new one 
        if grid.current_shape == TileType::NONE {
            grid.current_shape = Logic::get_new_tile();
            grid.marked_tiles = Logic::get_new_tile_shape(&grid.current_shape, &grid.width);
        } else {
            // is piece set
            if check_if_shape_is_set(&grid) {

                for marked_coordinate in &grid.marked_tiles {
                    grid.field[marked_coordinate.y][marked_coordinate.x] = grid.current_shape;
                }
                grid.current_shape = TileType::NONE;
                grid.marked_tiles.clear();

                if check_if_game_ends(&grid){
                    break;
                }
            } else {
                // if not, move one down
                for marked_coordinate in grid.marked_tiles.iter_mut() {
                    marked_coordinate.y += 1;
                }
            }

        }

        // todo draw game state
        renderer::Renderer::draw_game_scene(& mut render, &grid);

    }
    render.scene_state = SceneState::AFTERGAME;
    return render
}
fn main_menu_loop() {}


fn main() {
    let mut renderer = match renderer::Renderer::create(600, 800, "Tetris".to_owned()) {
        Ok(renderer) => renderer,
        Err(err_string) => {
            println!("Error while creating Renderer: {}", err_string); 
            exit(1)
        },
    };

    let state:SceneState = SceneState::GAME;
    while !renderer.handle.window_should_close() {
        renderer = draw_current_scene(state, renderer);
        
        // draw_handler.clear_background(Color::BLACK);
        // draw_handler.draw_text("Hey There! :3", 12, 12, 20, Color::WHITE);
    }

}
