use macroquad::prelude::*;

use super::body::Body;
use super::MAX_TRAIL_LENGTH;

/// Гравитационная постоянная в симуляции
pub const G: f32 = 0.1;

/// Максимальный шаг интегрирования — обеспечивает стабильность при любой скорости симуляции.
pub const MAX_PHYSICS_STEP: f32 = 0.008;

/// Минимальное расстояние для смягчения гравитации (избегаем деления на ноль)
const SOFTENING_DIST_SQ: f32 = 100.0;

/// Множитель гравитации Земли для Луны (для стабилизации орбиты)
const MOON_EARTH_GRAVITY_MULTIPLIER: f32 = 4.0;

pub struct World {
    pub bodies: Vec<Body>,
}

impl World {
    pub fn new() -> Self {
        Self { bodies: Vec::new() }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    /// Вычисляет ускорения для всех тел системы.
    /// Использует "мягчение" гравитации на малых расстояниях для стабильности.
    pub fn compute_accelerations(&mut self) {
        if self.bodies.is_empty() {
            return;
        }

        let sun_pos = self.bodies[0].position;
        let sun_mass = self.bodies[0].mass;
        
        // Находим Землю по индексу для оптимизации
        let earth_index = self.bodies.iter().position(|b| b.name == "Земля");
        let (earth_pos, earth_mass) = earth_index
            .map(|idx| {
                let b = &self.bodies[idx];
                (Some(b.position), Some(b.mass))
            })
            .unwrap_or((None, None));

        for (i, body) in self.bodies.iter_mut().enumerate() {
            if i == 0 {
                // Солнце не получает ускорения в этой модели
                body.acceleration = Vec2::ZERO;
                continue;
            }

            // Базовая гравитация от Солнца
            let mut acc = {
                let dir = sun_pos - body.position;
                let dist_sq = dir.length_squared().max(SOFTENING_DIST_SQ);
                dir.normalize() * (G * sun_mass / dist_sq)
            };

            // Дополнительная гравитация Земли на Луну:
            // Луна вращается вокруг Земли, но Земля при этом остаётся
            // на стабильной орбите вокруг Солнца (обратное влияние Луны
            // на Землю игнорируем ради устойчивости).
            if body.name == "Луна" {
                if let (Some(e_pos), Some(e_mass)) = (earth_pos, earth_mass) {
                    let dir_em = e_pos - body.position;
                    let dist_sq_em = dir_em.length_squared().max(SOFTENING_DIST_SQ * 0.1);
                    acc += dir_em.normalize() * (G * e_mass / dist_sq_em) * MOON_EARTH_GRAVITY_MULTIPLIER;
                }
            }

            body.acceleration = acc;
        }
    }

    /// Leapfrog (Verlet) — хорошо сохраняет энергию.
    pub fn step(&mut self, dt: f32) {
        // Половинный шаг для скорости
        for body in &mut self.bodies {
            body.velocity += body.acceleration * (dt * 0.5);
        }

        // Полный шаг для позиции
        for body in &mut self.bodies {
            body.position += body.velocity * dt;
        }

        // Пересчитываем ускорения на новых позициях
        self.compute_accelerations();

        // Вторая половина шага для скорости
        for body in &mut self.bodies {
            body.velocity += body.acceleration * (dt * 0.5);
        }

        // Обновляем следы траекторий с использованием кольцевого буфера
        for body in &mut self.bodies {
            if body.trail.len() >= MAX_TRAIL_LENGTH {
                body.trail.remove(0);
            }
            body.trail.push(body.position);
        }
    }

    /// Полная энергия системы в приближении "Солнце + планеты".
    /// Потенциальная энергия считается только между Солнцем и планетами,
    /// что совпадает с моделью в `compute_accelerations`.
    pub fn total_energy(&self) -> f32 {
        let mut energy = 0.0;

        // Кинетическая энергия всех тел
        for body in &self.bodies {
            energy += 0.5 * body.mass * body.velocity.length_squared();
        }

        if self.bodies.is_empty() {
            return energy;
        }

        let sun = &self.bodies[0];

        // Потенциальная энергия взаимодействия с Солнцем
        for planet in self.bodies.iter().skip(1) {
            let r = sun.position.distance(planet.position).max(5.0);
            energy -= G * sun.mass * planet.mass / r;
        }

        energy
    }
}
