use std::ops::Add;

use comfy::*;
use crate::component::{self, IsHovered};

pub fn get_world_click_box(model_box: &component::ClickBox, pos: &Vec2) -> component::ClickBox {
    let x = model_box.pos.x + pos.x;
    let y = model_box.pos.y + pos.y;
    return component::ClickBox {pos: vec2(x, y), size: model_box.size};
}

fn is_point_inside_box(pos: &Vec2, size: &Vec2, point: &Vec2) -> bool {
    let is_outside = point.x < pos.x || point.y < pos.y || pos.x + size.x < point.x || pos.y + size.y < point.y;
    return !is_outside;
}

fn handle_mouse_pressed() {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return;
    }
    let m = mouse_world();
    println!("Click {}", m);

    let w = world();
    for (id, clickbox) in world().query::<&component::ClickBox>().iter() {
        let p = match w.get::<&component::Position>(id) {
            Ok(v) => v.pos,
            Err(e) => vec2(0.0, 0.0)
        };
        let world_clickbox = get_world_click_box(clickbox, &p);
        
        if is_point_inside_box(&world_clickbox.pos, &world_clickbox.size, &m) {
            let duration = 0.5;
            commands().insert_one(id, component::WasClicked{duration: duration, time: duration});
        }
    }
}

fn handle_was_clicked_cool_down() {
    for (id, was_clicked) in world().query::<&mut component::WasClicked>().iter() {
        was_clicked.time -= delta();
        if was_clicked.time <= 0.0 {
            commands().remove_one::<component::WasClicked>(id);
        }
    }
}

fn handle_mouse_hover() {
    let w = world();
    let m = mouse_world();
    for (id, clickbox) in world().query::<&component::ClickBox>().iter() {
        let p = match w.get::<&component::Position>(id) {
            Ok(v) => v.pos,
            Err(e) => vec2(0.0, 0.0)
        };
        let world_clickbox = get_world_click_box(clickbox, &p);

        if is_point_inside_box(&world_clickbox.pos, &world_clickbox.size, &m) {
            commands().insert_one(id, IsHovered{});
        } else {
            commands().remove_one::<IsHovered>(id);
        }
    }
}

pub fn clickable_spin() {
    handle_was_clicked_cool_down();
    handle_mouse_pressed();
    handle_mouse_hover();
}

fn get_click_box_color(id: Entity) -> Color {
    if world().get::<&component::WasClicked>(id).is_ok() {
        return Color::new(1.0, 0.0, 0.0, 0.5);
    }

    if world().get::<&component::IsHovered>(id).is_ok() {
        return Color::new(0.0, 1.0, 0.0, 0.5);
    }

    return Color::new(1.0, 1.0, 1.0, 0.25)
}

pub fn clickable_draw() {
    let t = 4.0;
    for (id, clickbox) in world().query::<&component::ClickBox>().iter() {
        let color = get_click_box_color(id);
        let o = vec2(clickbox.size.x * 0.5, clickbox.size.y * 0.5);
        draw_rect(clickbox.pos + o, clickbox.size, color, 0);
        draw_rect_outline(clickbox.pos + o, clickbox.size, t, WHITE, 0);
    }
}