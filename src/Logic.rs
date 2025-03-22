use std::any::Any;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};


use rand;

#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) enum TileType { LRIGHT, LLEFT, SLEFT, SRIGHT, TSHAPE, ISHAPE, SQUARE, NONE }

impl Distribution<TileType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TileType {
        match rng.gen_range(0..=6) {
            0 => TileType::LRIGHT,
            1 => TileType::LLEFT,
            2 => TileType::SLEFT,
            3 => TileType::SRIGHT,
            4 => TileType::TSHAPE,
            5 => TileType::ISHAPE,
            _ => TileType::SQUARE,
        }
    }
}

struct Logic {}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub (crate) struct Coordinate {
    //height
    pub y: usize,
    //width
    pub x: usize
}

#[derive(Debug)]
pub (crate) struct GameGrid {
    // [height][width]
    pub field: Vec<Vec<TileType>>,
    pub width: usize,
    pub height: usize,
    pub marked_tiles: Vec<Coordinate>,
    pub current_shape: TileType
}

struct LogicError{

}

pub (crate) fn get_new_tile() -> TileType {
    return rand::random::<TileType>()
}

pub (crate) fn get_new_tile_shape(tile_type: &TileType, max_width: &usize) -> Vec<Coordinate> {
    let mut shape: Vec<Coordinate> = vec![];
    let mid = (max_width / 2) - 1;

    match tile_type {
        TileType::ISHAPE => {
            for idx in 0..4 {
                shape.push(Coordinate{y:idx, x:mid});
            }
        },
        TileType::LRIGHT => {
            for idx in 0..3 {
                shape.push(Coordinate{y:idx, x:mid});
            }
            shape.push(Coordinate{y:2, x:mid + 1});
        },
        TileType::LLEFT => {
            for idx in 0..3 {
                shape.push(Coordinate{y:idx, x:mid});
            }
            shape.push(Coordinate{y:2, x:mid - 1});

        },
        TileType::SLEFT => {
            shape.push(Coordinate{y:0, x:mid - 1});
            shape.push(Coordinate{y:0, x:mid});
            shape.push(Coordinate{y:1, x:mid});
            shape.push(Coordinate{y:1, x:mid + 1});

        },
        TileType::SRIGHT => {
            shape.push(Coordinate{y:0, x:mid + 1});
            shape.push(Coordinate{y:0, x:mid});
            shape.push(Coordinate{y:1, x:mid});
            shape.push(Coordinate{y:1, x:mid - 1});},
        TileType::TSHAPE => {
            shape.push(Coordinate{y:0, x:mid - 1});
            shape.push(Coordinate{y:0, x:mid});
            shape.push(Coordinate{y:0, x:mid + 1});
            shape.push(Coordinate{y:1, x:mid});
            shape.push(Coordinate{y:2, x:mid});
        },
        TileType::SQUARE => {
            shape.push(Coordinate{y:0, x:mid});
            shape.push(Coordinate{y:0, x:mid + 1});
            shape.push(Coordinate{y:1, x:mid});
            shape.push(Coordinate{y:1, x:mid + 1});
        },
        TileType::NONE => {},
    }
    return shape
}

pub (crate) fn check_lines (grid: &GameGrid) -> Vec<bool> {
    let mut ret_vec: Vec<bool> = vec![true; grid.height];

    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.field[y][x] != TileType::NONE {
                ret_vec[x] = false;
                break;
            }
        }
        
    }
    return ret_vec
}

pub (crate) fn remove_and_shrink_lines(mut grid: GameGrid, full_lines: Vec<bool>) -> GameGrid {
    // let mut ret_grid = grid.clone();
    for y in 0..grid.height {
        if full_lines[y] {
            for x in 0..grid.width {
                grid.field[y][x] = TileType::NONE;
            }
        }
    }

    for y in 0..grid.height {
        if y != 0 && full_lines[y - 1] {
            for x in 0..grid.width {
                grid.field[y][x] = grid.field[y - 1][x];
                grid.field[y - 1][x] = TileType::NONE;
            }
        }
    }
    return grid
}

pub (crate) fn check_if_shape_is_set(grid: &GameGrid) -> bool {
    let mut lowest :usize = grid.height - 1;
    lowest -= 1;
    if grid.current_shape == TileType::NONE {
        return true;
    }
    for idx in &grid.marked_tiles {
        // check if the shape is at bottom of the game
        if idx.y == lowest {
            return true;
        }
        // check if there are filled tiles below the shape 
        if grid.field[idx.y - 1][idx.x] != TileType::NONE {
            return true;
        }
    }

    return false;
}

pub (crate) fn check_if_game_ends(grid: &GameGrid) -> bool {
    for x in 0..grid.width {
        if grid.field[0][x] != TileType::NONE {
            return true;
        }
    }
    return false
}