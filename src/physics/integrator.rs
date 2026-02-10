#[derive(Debug, Clone, Copy)]
pub enum Integrator {
    Euler,
    Leapfrog,
    RK4,
}
