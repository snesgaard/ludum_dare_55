use comfy::*;
use asefile::*;

 
simple_game!("Nice red circle", setup, update);


fn setup(_c: &mut EngineContext) {
}


fn update(_c: &mut EngineContext) {
    draw_rect(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0), Color::new(1.0, 0.0, 0.0, 1.0), 1);
    draw_rect(Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Color::new(1.0, 1.0, 0.0, 1.0), 1);

    if is_key_pressed(KeyCode::Escape) {
        *_c.quit_flag = true;
    }
}
