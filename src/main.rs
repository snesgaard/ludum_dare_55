use aseprite_loader::{load_aseprite_atlas, ImageAtlas, draw_frame, find_first_frame_in_tag};
use std::path::Path;
use comfy::*;
mod aseprite_loader;

simple_game!("Nice red circle", GameState, config, setup, update);

pub struct GameState {
    atlas: Option<ImageAtlas>
}

impl GameState {
    pub fn new(_c: &mut EngineState) -> Self {
        Self { atlas: None }
    }
}

fn config(config: GameConfig) -> GameConfig {
    GameConfig {resolution: ResolutionConfig::Logical(1280, 720), ..config }
}

fn setup(gs: &mut GameState, _c: &mut EngineContext) {
    let mut m = main_camera_mut();
    m.zoom = screen_width() / 4.0;

    gs.atlas = Some(load_aseprite_atlas(_c, Path::new("target/atlas.json")));
}


fn update(gs: &mut GameState, _c: &mut EngineContext) {
    if is_key_pressed(KeyCode::Escape) {
        *_c.quit_flag = true;
    }
    
    let atlas = gs.atlas.as_ref().unwrap();

    let frame = find_first_frame_in_tag(atlas, &"background/background".to_string()).unwrap();

    draw_frame(
        atlas,
        frame,
        vec2(0.0, 0.0)
    );
}


/* 
fn main() {
    load_aseprite_atlas(Path::new("target/atlas.json"))
}
use aseprite_loader::{AsepriteAtlas, draw_atlas_frame, draw_atlas_tag};
use comfy::*;
use asefile::*;
use component::ClickBox;
mod aseprite_loader;
use std::path::Path;

mod component;
mod system;

 
simple_game!("Nice red circle", GameState, config, setup, update);

pub struct GameState {
    atlas: Option<AsepriteAtlas>
}

impl GameState {
    pub fn new(_c: &mut EngineState) -> Self {
        Self { atlas: None }
    }
}

fn config(config: GameConfig) -> GameConfig {
    GameConfig {resolution: ResolutionConfig::Logical(1280, 720), ..config }
}

fn setup(gs: &mut GameState, _c: &mut EngineContext) {
    let mut m = main_camera_mut();
    m.zoom = screen_width() / 4.0;
    gs.atlas = Some(aseprite_loader::load_aseprite_atlas(_c, Path::new("target/atlas.json"), Path::new("target/atlas.png")));

    commands().spawn(
        (ClickBox{pos: vec2(0.0, 0.0), size: vec2(100.0, 100.0)},)
    )
}


fn update(gs: &mut GameState, _c: &mut EngineContext) {
    //draw_rect(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0), Color::new(1.0, 0.0, 0.0, 1.0), 1);
    //draw_rect(Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Color::new(1.0, 1.0, 0.0, 1.0), 1);

    if is_key_pressed(KeyCode::Escape) {
        *_c.quit_flag = true;
    }


    /*
    let params = DrawTextureParams{
        source_rect: Some(IRect { offset: ivec2(0, 0), size: ivec2(100, 100) }),
        ..Default::default()
    };

    draw_sprite_ex(
        texture_id("atlas"), Vec2::ZERO, WHITE, 10, params
    );
    */


    system::clickable_spin();
    
    system::clickable_draw();
    draw_atlas_tag(gs.atlas.as_ref().unwrap(), "background".to_string(), 0.0, 0.0)
}
*/