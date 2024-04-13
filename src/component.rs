use comfy::*;

pub struct ClickBox {
    pub pos: Vec2,
    pub size: Vec2
}

pub struct Position {
    pub pos: Vec2
}

pub struct WasClicked {
    pub time: f32,
    pub duration: f32
}

pub struct IsHovered {}