use crate::super::vec2::Vec2;

#[derive(Clone)]
pub struct Body {
    pub mass: f64,
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub trail: Vec<Vec2>,
}
