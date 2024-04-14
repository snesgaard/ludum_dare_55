use std::ops::Add;

use comfy::*;
use crate::component::{self, IsHovered, Pickup, WasClicked, Lifetime, Demon, Motion};
use crate::aseprite_loader::{ImageAtlas};

pub fn get_world_click_box(model_box: &component::ClickBox, pos: &Vec2) -> component::ClickBox {
    let x = model_box.pos.x + pos.x;
    let y = model_box.pos.y + pos.y;
    return component::ClickBox {pos: vec2(x, y), size: model_box.size};
}

pub fn world_clickbox_from_id(id: Entity) -> Option<component::ClickBox> {
    let w = world();
    let maybe_model_click_box = w.get::<&component::ClickBox>(id);
    if (maybe_model_click_box.is_err()) {
        return None;
    }

    let pos = match w.get::<&component::Position>(id) {
        Err(e) => vec2(0.0, 0.0),
        Ok(p) => p.pos
    };

    return Some(get_world_click_box(&maybe_model_click_box.unwrap(), &pos));
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
        if was_clicked.time <= 0.0 || true {
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

pub fn is_clicked(id: Entity) -> bool {
    return world().get::<&component::WasClicked>(id).is_ok();
} 

fn get_click_box_color(id: Entity) -> Color {
    if world().get::<&component::WasClicked>(id).is_ok() {
        return Color::new(1.0, 0.0, 0.0, 0.25);
    }

    if world().get::<&component::IsHovered>(id).is_ok() {
        return Color::new(0.0, 1.0, 0.0, 0.25);
    }

    return Color::new(1.0, 1.0, 1.0, 0.0)
}

pub fn clickable_draw() {
    let t = 4.0;
    let w = world();
    for (id, clickbox) in w.query::<&component::ClickBox>().iter() {
        let p = match w.get::<&component::Position>(id) {
            Ok(v) => v.pos,
            Err(e) => vec2(0.0, 0.0)
        };
        let world_box = get_world_click_box(clickbox, &p);
        let color = get_click_box_color(id);
        let o = vec2(world_box.size.x * 0.5, world_box.size.y * 0.5);
        draw_rect(world_box.pos + o, world_box.size, color, 0);
        draw_rect_outline(world_box.pos + o, world_box.size, t, WHITE, 0);
    }
}

pub fn was_something_picked_up() -> Option<component::Pickup> {
    for (id, pickup) in world().query::<&component::Pickup>().iter() {
        if (is_clicked(id)) {
            return Some(pickup.clone());
        }
    }

    return None;
}

pub fn is_mouse_inside_summon_circle() -> bool {
    for (id, summon) in world().query::<&component::IsDropoff>().iter() {
        let maybe_world_clickbox = world_clickbox_from_id(id);
        if maybe_world_clickbox.is_none() {
            continue;
        }

        let world_clickbox = maybe_world_clickbox.unwrap();
        let m = mouse_world();
        if is_point_inside_box(&world_clickbox.pos, &world_clickbox.size, &m) {
            return true;
        }
    }

    return false;
}

pub fn pickup_spin() {
    let maybe_pickup = was_something_picked_up();
    if maybe_pickup.is_some() {
        for (_, gs) in world_mut().query_mut::<&mut component::GlobalGameState>().into_iter() {
            gs.pickup = maybe_pickup.clone();
        }
    }

    if is_mouse_button_released(MouseButton::Left) {
        let maybe_pickup = get_pickup_from_state();
        let should_append = is_mouse_inside_summon_circle();

        for (_, gs) in world_mut().query_mut::<&mut component::GlobalGameState>().into_iter() {
            gs.pickup = None;
            if maybe_pickup.is_some() && should_append {
                gs.recipe_stack.push(maybe_pickup.clone().unwrap());
            }
        }
    }
}

pub fn get_pickup_from_state() -> Option<Pickup> {
    for (_, gs) in world().query::<&component::GlobalGameState>().iter() {
        return gs.pickup.clone();
    }

    return None;
} 

pub fn was_recipe_book_clicked() -> bool {
    for (id, book) in world().query::<&component::IsBook>().iter() {
        if is_clicked(id) {
            return true;
        }
    }

    return false;
}

pub fn recipe_book_spin() {
    if is_mouse_button_pressed(MouseButton::Left) {
        for (_, gs) in world_mut().query_mut::<&mut component::GlobalGameState>().into_iter() {
            gs.show_recipe_book = false;
        }    
    }

    if !was_recipe_book_clicked() {
        return;
    }

    for (_, gs) in world_mut().query_mut::<&mut component::GlobalGameState>().into_iter() {
        gs.show_recipe_book = true;
    }
}

pub fn interactable_spin() {
    pickup_spin();
    recipe_book_spin();
}

pub fn get_recipe_stack() -> Vec<component::Pickup> {
    for (_, gs) in world().query::<&component::GlobalGameState>().iter() {
        return gs.recipe_stack.clone();
    }

    return vec![];
}

pub fn recipe_stack_spin() {
    if 3 <= get_recipe_stack().len() {
        for (_, gs) in world_mut().query_mut::<&mut component::GlobalGameState>().into_iter() {
            gs.recipe_stack.clear();
        }

        commands().spawn((
            ParticleSystem::with_spawn_on_death(300, || {
                Particle {
                    texture: texture_id("atlas"),
                    position: random_circle(5.0),
                    size: splat(10.0),
                    size_curve: expo_out,
                    z_index: 30,
                    angular_velocity: random() * 10.0,
                    // Both size and color can be faded.
                    fade_type: FadeType::Both,
                    color_start: RED,
                    color_end: RED,
                    ..Default::default()
                }
            }),
            Transform::position(vec2(10.0, 0.0)),
            Lifetime {time: 0.0, duration: 2.0}
        ));

        let rx = random() * 2.0 - 1.0;
        let ry = random();

        commands().spawn(
            (
                Demon::ToothImp {},
                Lifetime {time: 0.0, duration: 2.0},
                Motion {
                    position: vec2(10.0, 0.0),
                    velocity: vec2(rx * 200.0, ry * 200.0),
                    gravity: vec2(0.0, -300.0)
                },
            )
        )
    }
}

pub fn Lifetime_spin() {
    for (id, lifetime) in world_mut().query::<&mut component::Lifetime>().iter() {
        lifetime.time += delta();

        if lifetime.duration <= lifetime.time {
            commands().despawn(id);
        }
    }
}

pub fn motion_spin() {
    let dt = delta();
    for (id, motion) in world_mut().query::<&mut component::Motion>().iter() {
        motion.velocity += motion.gravity * dt;
        motion.position += motion.velocity * dt
    }
}