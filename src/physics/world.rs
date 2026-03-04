use macroquad::prelude::*;

use super::body::Body;

pub const G: f32 = 0.1;

/// Максимальный шаг интегрирования — обеспечивает стабильность при любой скорости симуляции.
pub const MAX_PHYSICS_STEP: f32 = 0.008;

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

    pub fn compute_accelerations(&mut self) {
        const EPSILON_SQ: f32 = 100.0;

        if self.bodies.is_empty() {
            return;
        }

        let sun_pos = self.bodies[0].position;
        let sun_mass = self.bodies[0].mass;
        let earth_index = self.bodies.iter().position(|b| b.name == "Земля");
        let (earth_pos, earth_mass) = earth_index
            .map(|idx| {
                let b = &self.bodies[idx];
                (Some(b.position), Some(b.mass))
            })
            .unwrap_or((None, None));

        for (i, body) in self.bodies.iter_mut().enumerate() {
            if i == 0 {
                body.acceleration = Vec2::ZERO;
                continue;
            }

            // Базовая гравитация от Солнца
            let mut acc = {
                let dir = sun_pos - body.position;
                let dist_sq = dir.length_squared().max(EPSILON_SQ);
                dir.normalize() * (G * sun_mass / dist_sq)
            };

            // Дополнительная гравитация Земли на Луну:
            // Луна вращается вокруг Земли, но Земля при этом остаётся
            // на стабильной орбите вокруг Солнца (обратное влияние Луны
            // на Землю игнорируем ради устойчивости).
            if body.name == "Луна" {
                if let (Some(e_pos), Some(e_mass)) = (earth_pos, earth_mass) {
                    let dir_em = e_pos - body.position;
                    let dist_sq_em = dir_em.length_squared().max(EPSILON_SQ * 0.1);
                    acc += dir_em.normalize() * (G * e_mass / dist_sq_em) * 4.0;
                }
            }

            body.acceleration = acc;
        }
    }

    /// Leapfrog (Verlet) — хорошо сохраняет энергию.
    pub fn step(&mut self, dt: f32) {
        for body in &mut self.bodies {
            body.velocity += body.acceleration * (dt * 0.5);
        }

        for body in &mut self.bodies {
            body.position += body.velocity * dt;
        }

        self.compute_accelerations();

        for body in &mut self.bodies {
            body.velocity += body.acceleration * (dt * 0.5);
        }

        for body in &mut self.bodies {
            body.trail.push(body.position);
            if body.trail.len() > 500 {
                body.trail.remove(0);
            }
        }
    }

    /// Полная энергия системы в приближении "Солнце + планеты".
    /// Потенциальная энергия считается только между Солнцем и планетами,
    /// что совпадает с моделью в `compute_accelerations`.
    pub fn total_energy(&self) -> f32 {
        let mut energy = 0.0;

        for body in &self.bodies {
            energy += 0.5 * body.mass * body.velocity.length_squared();
        }

        if self.bodies.is_empty() {
            return energy;
        }

        let sun = &self.bodies[0];

        for planet in self.bodies.iter().skip(1) {
            let r = sun.position.distance(planet.position).max(5.0);
            energy -= G * sun.mass * planet.mass / r;
        }

        energy
    }
}
