use aseprite_loader::{load_aseprite_atlas, ImageAtlas, draw_frame, find_first_frame_in_tag};
use component::{IsBook, Demon};
use system::world_clickbox_from_id;
use std::path::Path;
use comfy::*;

use crate::component::Pickup;
mod aseprite_loader;

mod system;
mod component;

simple_game!("Asmodeus Web Summons", GameState, config, setup, update);

pub struct GameState {
    atlas: Option<ImageAtlas>,
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

    let atlas = load_aseprite_atlas(_c, Path::new("target/atlas.json"));

    let frame = find_first_frame_in_tag(&atlas, &"background/background".to_string()).unwrap();
    spawn_level(frame);

    gs.atlas = Some(atlas);

    commands().spawn(
        (
            component::GlobalGameState{
                show_recipe_book: false,
                pickup: None,
                recipe_stack: vec![]
            },
        )
    );
}

fn properties_from_aseprite_data(
        world: &mut  AtomicRefMut<'static, World>, id: Entity, value: &serde_json::Value
) {
    let _pickup_res = match value["pickup"].as_str() {
        None => Ok(()),
        Some(s) => match s {
            "web" => world.insert_one(id, Pickup::Web {  }),
            "skull" => world.insert_one(id, Pickup::Skull {  }),
            "fire" => world.insert_one(id, Pickup::Fire {  }),
            "knife" => world.insert_one(id, Pickup::Knife {  } ),
            "booze" => world.insert_one(id, Pickup::Booze {  }),
            "eye" => world.insert_one(id, Pickup::Eye {  }),
            "tentacle" => world.insert_one(id, Pickup::Tentacle {  }),
            _ => Ok(())
        }
    };

    let _recipe_book = match value["recipe_book"].as_bool() {
        None => Ok(()),
        Some(s) => match s {
            true => world.insert_one(id, component::IsBook{} ),
            false => Ok(())
        }
    };

    let _recipe_dropoff = match value["recipe_dropoff"].as_bool() {
        None => Ok(()),
        Some(s) => world.insert_one(id, component::IsDropoff {})
    };

    let _summon_circle = match value["summon_circle"].as_bool() {
        None => Ok(()),
        Some(s) => world.insert_one(id, component::IsSummonCircle{ })
    };
}

fn spawn_level(frame: &aseprite_loader::Frame) {
    let mut w = world_mut();
    for s in frame.slices.iter() {
        let id = w.reserve_entity();
        let _entity_result = w.insert(
            id,
            (
                component::ClickBox{
                    pos: vec2(0.0, 0.0),
                    size: vec2(s.bound.w as f32, s.bound.h as f32)
                },
                component::Position{
                    pos: vec2(
                        (s.bound.x - frame.source_size.w / 2) as f32,
                        (-s.bound.y + frame.source_size.h / 2 - s.bound.h ) as f32
                    )
                }
            )
        );
        
        properties_from_aseprite_data(&mut w, id, &s.data);
    }
}

fn should_show_book() -> bool {
    for (id, gs) in world().query::<&component::GlobalGameState>().iter() {
        if gs.show_recipe_book {
            return true;
        }
    }

    return false;
}

fn should_draw_pickup() -> bool {
    for (id, gs) in world().query::<&component::GlobalGameState>().iter() {
        if gs.pickup.is_some() {
            return true;
        }
    }

    return false;
}

fn pickup_frame_string_from_state() -> Option<String> {
    for (id, gs) in world().query::<& component::GlobalGameState>().iter() {
        return match &gs.pickup {
            None => None,
            Some(p) => pickup_frame_string_from_enum(p)
        };
    }

    return None;
}

fn pickup_frame_string_from_enum(p: &component::Pickup) -> Option<String> {
    return match p {
        Pickup::Web {  } => Some("pickups/web".to_string()),
        Pickup::Tentacle {  } => Some("pickups/tentacle".to_string()),
        Pickup::Eye {  } => Some("pickups/eye".to_string()),
        Pickup::Fire {  } => Some("pickups/fire".to_string()),
        Pickup::Skull {  } => Some("pickups/skull".to_string()),
        Pickup::Knife {  } => Some("pickups/knife".to_string()),
        Pickup::Booze {  } => Some("pickups/booze".to_string()),
        _ => None
    };
}

fn demon_frame_string_from_enum(d: &component::Demon) -> Option<String> {
    return match d {
        Demon::ToothImp {  } => Some("demons/tooth_imp".to_string()),
        _ => None
    };
}

fn draw_icon(gs: &GameState, pos: Vec2) {
    let atlas = gs.atlas.as_ref().unwrap();
    let maybe_framekey = pickup_frame_string_from_state();
    if maybe_framekey.is_none() {
        return;
    }
    let frame = find_first_frame_in_tag(atlas, &maybe_framekey.unwrap()).unwrap();
    draw_frame(atlas, frame, pos, 10);
}

fn find_draw_circle_location() -> Option<Vec2> {
    for (id, _) in world().query::<&component::IsSummonCircle>().iter() {
        let maybe_box = world_clickbox_from_id(id);
        if (maybe_box.is_some()) {
            let b = maybe_box.unwrap();
            return Some(vec2(b.pos.x + b.size.x * 0.5, b.pos.y + b.size.y * 0.5));
        }
    }

    return None;
}

fn draw_recipe_stack_from_stack(gs: &GameState, recipe_stack: &Vec<component::Pickup>) {
    let maybe_p = find_draw_circle_location();
    if maybe_p.is_none() {
        return;
    }
    let p = maybe_p.unwrap();
    let atlas = gs.atlas.as_ref().unwrap();
    for (i, r) in recipe_stack.iter().enumerate() {
        let maybe_frame_key = pickup_frame_string_from_enum(r);
        if maybe_frame_key.is_none() {
            continue;
        }
        let o = vec2(0.0, 10.0 * (i + 1) as f32);
        let frame = find_first_frame_in_tag(atlas, &maybe_frame_key.unwrap());
        draw_frame(atlas, frame.unwrap(), p + o, 30);
    }
}

fn draw_recipe_stack(gs: &GameState) {
    for (id, global_gs) in world().query::<&component::GlobalGameState>().iter() {
        draw_recipe_stack_from_stack(gs, &global_gs.recipe_stack);
    }
}

fn draw_demons(gs: &GameState) {
    let atlas = gs.atlas.as_ref().unwrap();
    let w = world();
    for (id, demon) in w.query::<&component::Demon>().iter() {
        let maybe_demon_string = demon_frame_string_from_enum(demon);
        if maybe_demon_string.is_none() {
            continue;
        }

        let pos = match w.get::<&component::Motion>(id) {
            Err(e) => vec2(0.0, 0.0),
            Ok(m) => m.position
        };

        let maybe_frame = find_first_frame_in_tag(atlas, &maybe_demon_string.unwrap());
        if maybe_frame.is_none() {
            continue;
        }
        draw_frame(atlas, maybe_frame.unwrap(), pos, 40);
    }
}

fn should_draw_no() -> bool {
    for (id, no) in world().query::<&component::DrawNo>().iter() {
        return true;
    }

    return false;
}

fn should_draw_win() -> bool {
    for (id, win) in world().query::<&component::DrawWin>().iter() {
        return true;
    }
    return false;
}

fn update(gs: &mut GameState, _c: &mut EngineContext) {
    if is_key_pressed(KeyCode::Escape) {
        *_c.quit_flag = true;
    }
    
    let atlas = gs.atlas.as_ref().unwrap();

    let frame = find_first_frame_in_tag(atlas, &"background/background".to_string()).unwrap();
    draw_frame(atlas, frame, vec2(0.0, 0.0), 0);

    let ui_frame = find_first_frame_in_tag(atlas, &"GUI/ui".to_string()).unwrap();
    //draw_frame(atlas, ui_frame, vec2(0.0, 0.0), 1);

    // Update 
    system::clickable_spin();
    system::pickup_spin();
    system::interactable_spin();
    system::recipe_book_spin();
    system::recipe_stack_spin();
    system::Lifetime_spin();
    system::motion_spin();

    if should_draw_pickup() {
        draw_icon(gs, mouse_world());
    }
    
    // Debug draw
    // system::clickable_draw();

    if should_show_book() {
        let book_frame = find_first_frame_in_tag(atlas, &"book/book".to_string()).unwrap();
        draw_frame(atlas, book_frame, vec2(0.0, 0.0), 2);
    }

    //draw_recipe_stack_from_stack(gs, &recipe_stack);
    draw_recipe_stack(gs);
    draw_demons(gs);

    if should_draw_no() {
        let no_frames = find_first_frame_in_tag(atlas, &"no/no".to_string()).unwrap();
        draw_frame(atlas, no_frames, vec2(0.0, 0.0), 100);
    }

    if should_draw_win() {
        let win_frames = find_first_frame_in_tag(atlas, &"no/win".to_string()).unwrap();
        draw_frame(atlas, win_frames, vec2(0.0, 0.0), 100);
    }
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