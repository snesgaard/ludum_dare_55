use aseprite_loader::{AsepriteAtlas, draw_atlas_frame};
use comfy::*;
use asefile::*;
mod aseprite_loader;
use std::path::Path;

 
simple_game!("Nice red circle", setup, update);

pub struct GameState {
    atlas: AsepriteAtlas
}

fn setup(_c: &mut EngineContext) {
    commands().spawn(
        (aseprite_loader::load_aseprite_atlas(
            _c, Path::new("target/atlas.json"), Path::new("target/atlas.png")
        ),)
    );
}


fn update(_c: &mut EngineContext) {
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

    for (id, atlas) in world().query::<&AsepriteAtlas>().iter() {
        println!("Never!");
        draw_atlas_frame(&atlas, 2, 0.0, 0.0);
    }

}
