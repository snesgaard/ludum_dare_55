use comfy::*;

pub struct ClickBox {
    pub pos: Vec2,
    pub size: Vec2
}

pub struct DIE {
    
}

pub struct Position {
    pub pos: Vec2
}

pub struct WasClicked {
    pub time: f32,
    pub duration: f32
}

pub struct IsHovered {}

pub struct Owner {
    pub id: Entity
}

pub struct Level {
    pub frame: String
}

#[derive(Clone)]
pub enum Pickup {
    Skull {},
    Fire {},
    Web {},
    Tentacle {},
    Knife {},
    Eye {},
    Booze {}
}

pub struct IsBook {}

pub struct IsDropoff {}

pub struct IsSummonCircle {}

pub struct GlobalGameState {
    pub show_recipe_book: bool,
    pub pickup: Option<Pickup>,
    pub recipe_stack: Vec<Pickup>
}

pub struct Lifetime {
    pub time: f32,
    pub duration: f32
}

pub struct Motion {
    pub position: Vec2,
    pub velocity: Vec2,
    pub gravity: Vec2
}

#[derive(Clone)]
pub enum Demon {
    ToothImp {}
}