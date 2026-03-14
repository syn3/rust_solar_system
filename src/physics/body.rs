use macroquad::prelude::*;

use crate::nasa_data::NasaData;
use super::new_trail;

#[derive(Clone)]
pub struct Body {
    pub name: String,
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub color: Color,
    pub trail: Vec<Vec2>,
    pub nasa_data: Option<NasaData>,
}

impl Body {
    pub fn new(
        name: impl Into<String>,
        position: Vec2,
        velocity: Vec2,
        mass: f32,
        radius: f32,
        color: Color,
    ) -> Self {
        Self {
            name: name.into(),
            position,
            velocity,
            acceleration: Vec2::ZERO,
            mass,
            radius,
            color,
            trail: new_trail().into(),
            nasa_data: None,
        }
    }

    pub fn with_nasa(mut self, data: NasaData) -> Self {
        self.nasa_data = Some(data);
        self
    }

    /// Скорость (модуль вектора скорости)
    pub fn speed(&self) -> f32 {
        self.velocity.length()
    }
}
