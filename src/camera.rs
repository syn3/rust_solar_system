//! Камера: масштаб и смещение для отображения симуляции.

use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Camera {
    pub offset: Vec2,
    pub scale: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            offset: Vec2::ZERO,
            scale: 1.0,
        }
    }

    /// Мировые координаты -> экранные
    pub fn world_to_screen(&self, world: Vec2) -> Vec2 {
        let center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
        center + (world - self.offset) * self.scale
    }

    /// Экранные -> мировые
    pub fn screen_to_world(&self, screen: Vec2) -> Vec2 {
        let center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
        self.offset + (screen - center) / self.scale
    }

    /// Масштабировать радиус тела для отображения (не зависит от zoom камеры)
    pub fn scale_radius(&self, world_radius: f32) -> f32 {
        world_radius * self.scale
    }

    pub fn zoom(&mut self, delta: f32, at_screen: Vec2) {
        let world_before = self.screen_to_world(at_screen);
        self.scale = (self.scale * (1.0 + delta * 0.15)).clamp(0.1, 20.0);
        let world_after = self.screen_to_world(at_screen);
        self.offset += world_before - world_after;
    }

    pub fn pan(&mut self, delta: Vec2) {
        self.offset -= delta / self.scale;
    }

    /// Плавное приближение к целевой позиции и зуму (для следования за выбранным телом)
    pub fn smooth_follow(&mut self, target_offset: Vec2, target_scale: f32, t: f32) {
        self.offset = self.offset + (target_offset - self.offset) * t;
        self.scale = self.scale + (target_scale - self.scale) * t;
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
