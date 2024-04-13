use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use comfy::{Color, WHITE, Itertools, load_texture_from_engine_bytes, epaint::TextureId, egui::Image};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use comfy::*;

use crate::main;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32
}

#[derive(Serialize, Deserialize)]
pub struct AsepriteSize {
    w: i32,
    h: i32
}

#[derive(Serialize, Deserialize)]
pub struct SliceKeys {
    frame: i32,
    bounds: Rect
}

#[derive(Serialize, Deserialize)]
pub struct AsepriteFrame {
    filename: String,
    frame: Rect,
    rotated: bool,
    trimmed: bool,
    spriteSourceSize: Rect,
    duration: i32
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AsepriteFrametag {
    name: String,
    to: i32,
    from: i32,
    direction: String
}

#[derive(Serialize, Deserialize)]
pub struct AspriteSliceInstance {
    frame: i32,
    bounds: Rect
}

#[derive(Serialize, Deserialize)]
pub struct AsepriteSlice {
    name: String,
    color: String,
    data: Option<String>,
    keys: Vec<AspriteSliceInstance>,
    from: i32,
    to: i32
}

#[derive(Serialize, Deserialize)]
pub struct AsepriteMeta {
    app: String,
    version: String,
    image: String,
    format: String,
    size: AsepriteSize,
    scale: String,
    frameTags: Vec<AsepriteFrametag>,
    slices: Vec<AsepriteSlice>
}

#[derive(Serialize, Deserialize)]
pub struct AsepriteAtlas {
    frames: Vec<AsepriteFrame>,
    meta: AsepriteMeta
}

pub struct Slice {
    bound: Rect,
    data: Value,
    color: Color,
    name: String
}

pub struct Frame {
    rect: Rect,
    slices: Vec<Slice>,
    duration: i32
}

fn is_frame_covered(frame_index: i32, ase: &AspriteSliceInstance) -> bool {
    return ase.frame <= frame_index;
}

fn find_single_slice_in_frames(frame_index: i32, slice: &AsepriteSlice) -> Option<Slice> {
    if frame_index < slice.from || slice.to < frame_index {
        return None;
    }

    let s = slice.keys.iter().rev().find_or_last (|&x| is_frame_covered(frame_index, x));

    if s.is_none() {
        return None;
    }

    return Some(Slice {
        bound: s.unwrap().bounds,
        data: match &slice.data {
            Some(s) => serde_json::from_str(s.as_str()).unwrap(),
            None => serde_json::from_str("{}").unwrap()
        }, 
        color: WHITE,
        name: slice.name.clone()
    })
}

fn find_slices_in_frame(frame_index: i32, slices: &Vec<AsepriteSlice>) -> Vec<Slice> {
    return slices
        .iter()
        .map(|s| find_single_slice_in_frames(frame_index, &s))
        .filter(|s| s.is_some())
        .map(|s| s.unwrap())
        .collect_vec();
}

pub struct ImageAtlas {
    pub texture_id: TextureHandle,
    pub frames: Vec<Frame>,
    pub tags: HashMap<String, AsepriteFrametag>,
    pub size: Vec2
}

pub fn find_first_frame_in_tag<'a>(atlas: &'a ImageAtlas, name: &'a String) -> Option<&'a Frame> {
    let maybe_tag = atlas.tags.get(name);
    if maybe_tag.is_none() { return None; }

    return atlas.frames.get(maybe_tag.unwrap().from as usize);
}

pub fn draw_frame(atlas: &ImageAtlas, frame: &Frame, pos: Vec2) {
    let params = DrawTextureProParams {
        source_rect: Some(IRect{
            offset: IVec2::new(frame.rect.x, frame.rect.y),
            size: ivec2(frame.rect.w, frame.rect.h)
        }),
        size: vec2(frame.rect.w as f32, frame.rect.h as f32),
        ..Default::default()
    };
    draw_sprite_pro(atlas.texture_id, pos, WHITE, 0, params)
}

pub fn load_aseprite_atlas( _c: &mut EngineContext, json_path: &Path) -> ImageAtlas {
    let file = File::open(json_path).unwrap();
    let root: AsepriteAtlas = serde_json::from_reader(file).unwrap();

    println!("Loading {}", json_path.to_str().unwrap());
    for s in find_slices_in_frame(11, &root.meta.slices) {
        println!(
            "Slice [{}, {}, {}, {}] = {}",
            s.bound.x, s.bound.y, s.bound.w, s.bound.h,
            s.data
        );
    }

    let mut formatter_frames: Vec<Frame> = vec![];
    for (frame_index, frame) in root.frames.iter().enumerate() {
        let slices = find_slices_in_frame(frame_index as i32, &root.meta.slices);
        let out_frame = Frame {
            duration: frame.duration,
            rect: frame.frame,
            slices: slices
        };
        formatter_frames.push(out_frame);
    }

    let image_path = json_path.parent().unwrap().join(root.meta.image);
    println!("Loading image file {}", image_path.to_str().unwrap());
    let file = File::open(json_path).unwrap();
    _c.load_texture_from_bytes("atlas", std::fs::read(image_path).unwrap().as_slice());

    let mut tags: HashMap<String, AsepriteFrametag> = hashmap! {};


    for ft in root.meta.frameTags.iter() {
        tags.insert(ft.name.clone(), ft.clone());
    }

    return ImageAtlas {
        texture_id: texture_id("atlas"), frames: formatter_frames,
        tags: tags,
        size: vec2(root.meta.size.w as f32, root.meta.size.h as f32)
    };
}