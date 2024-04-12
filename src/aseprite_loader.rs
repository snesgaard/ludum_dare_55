use std::path::Path;
use serde_json::{Result, Value};
use std::fs::File;
use std::io::BufReader;
use comfy::*;
use std::collections::HashMap;

use crate::main;

struct Rectangle {
    x: f64,
    y: f64,
    w: f64,
    h: f64
}

fn rectangle(x: f64, y: f64, w: f64, h: f64) -> Rectangle {
    return Rectangle{x: x, y: y, w: w, h: h};
}

struct AsepriteFrame {
    rect: IRect,
    duration: i64
}

struct FrameTag {
    from: i64,
    to: i64
}

pub struct AsepriteAtlas {
    id: String,
    frames: Vec<AsepriteFrame>,
    frame_tags: HashMap<String, FrameTag>
}

pub fn draw_atlas_frame(atlas: &AsepriteAtlas, frame_index: i64, x: f32, y: f32) {
    let camera = main_camera();
    let f = &atlas.frames[frame_index as usize];
    let params = DrawTextureProParams{
        source_rect: Some(f.rect),
        size: Vec2::new((f.rect.size.x as f32) / camera.zoom, (f.rect.size.y as f32)  / camera.zoom),
        ..Default::default()
    };

    draw_sprite_pro(texture_id("atlas"), Vec2::new(x, y), WHITE, 0, params);
}

pub fn load_aseprite_atlas(c: &mut EngineContext, json_path: &Path, atlas: &Path) -> AsepriteAtlas {
    // Open the file in read-only mode with buffer.
    let file = File::open(json_path).unwrap();
    let reader = BufReader::new(file);
 
    // Read the JSON contents of the file as an instance of `User`.
    let root: Value = serde_json::from_reader(reader).unwrap();

    let mut frames: Vec<AsepriteFrame> = vec![];

    for frame in root["frames"].as_array().unwrap() {
        let quad_json = frame["frame"].as_object().unwrap();
        let quad = IRect{
            offset: ivec2(
                quad_json["x"].as_i64().unwrap() as i32,
                quad_json["y"].as_i64().unwrap() as i32
            ),
            size: ivec2(
                quad_json["w"].as_i64().unwrap() as i32,
                quad_json["h"].as_i64().unwrap() as i32
            )
        };
    
        let f = AsepriteFrame{rect: quad, duration: frame["duration"].as_i64().unwrap()};
        frames.push(f);
    }

    c.load_texture_from_bytes(
        // Every texture gets a string name later used to reference it.
        "atlas",
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/target/atlas.png")),
    );

    let mut frame_tags: HashMap<String, FrameTag> = HashMap::new();

    for frame_tag in root["meta"]["frameTags"].as_array().unwrap() {
        let name = frame_tag["name"].as_str().unwrap().to_string();
        let from = frame_tag["from"].as_i64().unwrap();
        let to = frame_tag["to"].as_i64().unwrap();

        let ft = FrameTag {from: from, to: to};
        frame_tags.insert(name, ft);
    }

    let atlas = AsepriteAtlas{
        id: "atlas".to_string(),
        frames: frames,
        frame_tags: frame_tags
    };
    return atlas
}