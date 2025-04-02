use raylib::{ffi::TextToUpper, prelude::*};

use crate::{Logic::GameGrid, TileType};

const GAME_FIELD_RATIO:f32 = 0.6; 
const GAME_BORDER:f32 = 10.0; 
const GAME_BORDER_THICKNESS:f32 = 4.0;
const GAME_TILE_SIZE:f32 = 30.0;

const GAME_SPRITE_PATH:&str = "/home/kyomawolf/Code/rust/tetris/sprites";
const GAME_SPRITE_LRIGHT:&str = "sprite_tetris_0.png";
const GAME_SPRITE_LLEFT:&str = "sprite_tetris_1.png";
const GAME_SPRITE_SLEFT:&str = "sprite_tetris_2.png";
const GAME_SPRITE_SRIGHT:&str = "sprite_tetris_3.png";
const GAME_SPRITE_TSHAPE:&str = "sprite_tetris_4.png";
const GAME_SPRITE_ISHAPE:&str = "sprite_tetris_5.png";
const GAME_SPRITE_SQUARE:&str = "sprite_tetris_6.png";


#[derive(Debug, Clone, Copy)]
pub enum SceneState{ MAINMENU, GAME, AFTERGAME }


// todo fix
#[derive(Debug, Clone, Copy)]
pub (crate)struct GameWindowProperties {
    pub width:i32,
    pub height:i32,
    pub margin_game:Rectangle,
    pub margin_stats:Rectangle
}

#[derive(Debug)]
pub (crate) struct GameTextures {
    pub lright:Texture2D,
    pub lleft:Texture2D,
    pub sleft:Texture2D,
    pub sright:Texture2D,
    pub tshape:Texture2D,
    pub ishape:Texture2D,
    pub square:Texture2D
}

impl GameTextures {
    fn create(handle:& mut RaylibHandle, thread:&RaylibThread) -> Result<GameTextures, String> {
    let path = GAME_SPRITE_PATH.to_owned();
        return Ok(GameTextures{
            lright:(handle.load_texture(&thread, &(path.clone() + "/" + GAME_SPRITE_LRIGHT))?),
            lleft:(handle.load_texture(&thread, &(path.clone() + "/" + GAME_SPRITE_LLEFT))?),
            sleft:(handle.load_texture(&thread, &(path.clone() + "/" + GAME_SPRITE_SLEFT))?),
            sright:(handle.load_texture(&thread, &(path.clone() + "/" + GAME_SPRITE_SRIGHT))?),
            tshape:(handle.load_texture(&thread, &(path.clone() + "/" + GAME_SPRITE_TSHAPE))?),
            ishape:(handle.load_texture(&thread, &(path.clone() + "/" + GAME_SPRITE_ISHAPE))?),
            square:(handle.load_texture(&thread, &(path.clone() + "/" + GAME_SPRITE_SQUARE))?)
        })
    }
}

#[derive(Debug)]
pub (crate)struct Renderer {
    pub props:GameWindowProperties,
    pub scene_state:SceneState,

    pub textures:GameTextures,

    pub handle:RaylibHandle,
    pub thread:RaylibThread
}

fn draw_background(draw_handle: & mut RaylibDrawHandle, properties:GameWindowProperties) {
    // background
    
    draw_handle.clear_background(Color::BLACK);
    // borders
    // draw_handle.draw_rectangle_rec(properties.margin_game,raylib::color::Color::WHITE);
    // draw_handle.draw_rectangle_rec(properties.margin_stats,raylib::color::Color::RED);
    
    //draw_handle.draw_rectangle(5, 5, ((600 as f32) * 0.3) as i32, 795, raylib::color::Color::WHITE);

    // good to know lines go to the inside and dont split even
    draw_handle.draw_rectangle_lines_ex(properties.margin_game, GAME_BORDER_THICKNESS, Color::LIGHTGRAY);
    draw_handle.draw_rectangle_lines_ex(properties.margin_stats, GAME_BORDER_THICKNESS, Color::LIGHTGRAY);

    // draw test cube

    // draw_handle.draw_rectangle(50, 50, 30, 30, Color::RED);

    // let rect = Rectangle::new(50.0, 50.0, 30.0, 30.0);
    // draw_handle.draw_rectangle(50, 50, 30, 30, Color::RED);
    // draw_handle.draw_rectangle_lines_ex(rect, 5.0, Color::WHITE);
}
impl Renderer {
    pub fn create(new_width:i32, new_height:i32, new_title:String) -> Result<Renderer, String> {
        let (mut rl, thread) = raylib::init()
        .size(new_width, new_height)
        .title(&new_title)
        .build();
        let window_width = new_width as f32;
        let window_height = new_height as f32;

        let margin_game = Rectangle::new(GAME_BORDER, GAME_BORDER, window_width * GAME_FIELD_RATIO , window_height - GAME_BORDER);
        let margin_stats = Rectangle::new(window_width * GAME_FIELD_RATIO + GAME_BORDER, GAME_BORDER, window_width * (1.0 - GAME_FIELD_RATIO) - GAME_BORDER, window_height - GAME_BORDER);

        let all_textures = GameTextures::create(&mut rl, &thread)?;

        let properties:GameWindowProperties = GameWindowProperties{width:new_width, height:new_height,margin_game, margin_stats};
        return  Ok(Renderer{props:properties, scene_state:SceneState::MAINMENU, 
            textures: all_textures, handle:rl, thread:thread});
        }


    fn draw_statistics(self, game:&GameGrid) -> Renderer {
        return self;
    }

    // todo make it to be on self.
    fn draw_tile_coordinate(draw_handle:& mut RaylibDrawHandle,
                            properties:GameWindowProperties, 
                            textures:&GameTextures, 
                            grid_x:i32, grid_y:i32, 
                            tile_type: TileType) {

        let texture = match tile_type {
            TileType::LRIGHT => &textures.lright,
            TileType::LLEFT => &textures.lleft,
            TileType::SLEFT => &textures.sleft,
            TileType::SRIGHT => &textures.sright,
            TileType::TSHAPE => &textures.tshape,
            TileType::ISHAPE => &textures.ishape,
            TileType::SQUARE => &textures.square,
            TileType::NONE => return,
        };
        let x = (grid_x as f32 * GAME_TILE_SIZE) + properties.margin_game.x; 
        let y = (grid_y as f32 * GAME_TILE_SIZE) + properties.margin_game.y; 
        let pos =Vector2::new(x, y);
        draw_handle.draw_texture_ex(texture, pos,0.0, 3.0,  Color::WHITE);
    }

    // todo high performace, compare old vs new game state
    fn draw_tiles(draw_handle:& mut RaylibDrawHandle, properties:GameWindowProperties, textures:&GameTextures, game:&GameGrid) {
        for y in 0..game.height {
            for x in 0..game.width {
                Self::draw_tile_coordinate(draw_handle, properties, &textures, x as i32, y as i32, game.field[y][x]);
            } 
        }

        for idx in &game.marked_tiles {
            Self::draw_tile_coordinate(draw_handle, properties, &textures, idx.x as i32, idx.y as i32, game.current_shape);
        }
    }

    pub fn draw_game_scene(render:& mut Renderer, game:&GameGrid) {
        let mut draw_handle: RaylibDrawHandle = render.handle.begin_drawing(&render.thread);
    
        draw_background(&mut draw_handle, render.props);
        Self::draw_tiles(&mut draw_handle, render.props, &render.textures, game);
    }
}
