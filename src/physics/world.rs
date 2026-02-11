use super::{body::Body, integrator::Integrator, vec2::Vec2};

pub struct World {
    pub bodies: Vec<Body>,
    g: f64,
    pub integrator: Integrator,
}

impl World {
    pub fn new(g: f64) -> Self {
        Self {
            bodies: Vec::new(),
            g,
            integrator: Integrator::Leapfrog,
        }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    /* ------------------ УСКОРЕНИЯ ------------------ */

    pub fn compute_accelerations(&mut self) {
        for i in 0..self.bodies.len() {
            let mut acc = Vec2::zero();

            for j in 0..self.bodies.len() {
                if i == j {
                    continue;
                }

                let r = self.bodies[j].position.sub(self.bodies[i].position);
                let dist = r.length() + 1e-6;

                acc = acc.add(r.mul(self.g * self.bodies[j].mass / (dist * dist * dist)));
            }

            self.bodies[i].acceleration = acc;
        }
    }

    /* ------------------ ШАГ СИМУЛЯЦИИ ------------------ */

    pub fn step(&mut self, dt: f64) {
        match self.integrator {
            Integrator::Euler => self.step_euler(dt),
            Integrator::Leapfrog => self.step_leapfrog(dt),
            Integrator::RK4 => self.step_rk4(dt),
        }
    }

    /* ------------------ EULER ------------------ */

    pub fn step_euler(&mut self, dt: f64) {
        self.compute_accelerations();

        for body in &mut self.bodies {
            body.position = body.position.add(body.velocity.mul(dt));
            body.velocity = body.velocity.add(body.acceleration.mul(dt));
        }
    }

    /* ------------------ LEAPFROG ------------------ */

    pub fn step_leapfrog(&mut self, dt: f64) {
        for body in &mut self.bodies {
            body.velocity = body.velocity.add(body.acceleration.mul(dt * 0.5));
        }

        for body in &mut self.bodies {
            body.position = body.position.add(body.velocity.mul(dt));
        }

        self.compute_accelerations();

        for body in &mut self.bodies {
            body.velocity = body.velocity.add(body.acceleration.mul(dt * 0.5));
        }
    }

    /* ------------------ RK4 ------------------ */

    pub fn step_rk4(&mut self, dt: f64) {
        let original = self.bodies.clone();

        self.compute_accelerations();
        let k1: Vec<_> = original
            .iter()
            .map(|b| (b.velocity, b.acceleration))
            .collect();

        self.apply_k(&k1, dt * 0.5);
        self.compute_accelerations();
        let k2: Vec<_> = self
            .bodies
            .iter()
            .map(|b| (b.velocity, b.acceleration))
            .collect();

        self.bodies = original.clone();
        self.apply_k(&k2, dt * 0.5);
        self.compute_accelerations();
        let k3: Vec<_> = self
            .bodies
            .iter()
            .map(|b| (b.velocity, b.acceleration))
            .collect();

        self.bodies = original.clone();
        self.apply_k(&k3, dt);
        self.compute_accelerations();
        let k4: Vec<_> = self
            .bodies
            .iter()
            .map(|b| (b.velocity, b.acceleration))
            .collect();

        self.bodies = original;

        for i in 0..self.bodies.len() {
            self.bodies[i].position = self.bodies[i].position.add(
                k1[i]
                    .0
                    .add(k2[i].0.mul(2.0))
                    .add(k3[i].0.mul(2.0))
                    .add(k4[i].0)
                    .mul(dt / 6.0),
            );

            self.bodies[i].velocity = self.bodies[i].velocity.add(
                k1[i]
                    .1
                    .add(k2[i].1.mul(2.0))
                    .add(k3[i].1.mul(2.0))
                    .add(k4[i].1)
                    .mul(dt / 6.0),
            );
        }
    }

    fn apply_k(&mut self, k: &Vec<(Vec2, Vec2)>, dt: f64) {
        for i in 0..self.bodies.len() {
            self.bodies[i].position = self.bodies[i].position.add(k[i].0.mul(dt));
            self.bodies[i].velocity = self.bodies[i].velocity.add(k[i].1.mul(dt));
        }
    }

    /* ------------------ ЭНЕРГИЯ ------------------ */

    pub fn total_energy(&self) -> f64 {
        let mut kinetic = 0.0;
        let mut potential = 0.0;

        for body in &self.bodies {
            kinetic += 0.5 * body.mass * body.velocity.length().powi(2);
        }

        for i in 0..self.bodies.len() {
            for j in (i + 1)..self.bodies.len() {
                let r = self.bodies[i]
                    .position
                    .sub(self.bodies[j].position)
                    .length();

                potential -= self.g * self.bodies[i].mass * self.bodies[j].mass / r;
            }
        }

        kinetic + potential
    }
}
