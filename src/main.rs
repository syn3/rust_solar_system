mod physics;
mod ui;

use macroquad::prelude::*;

use physics::integrator::Integrator;
use physics::vec2::Vec2;
use physics::world::World;
use src::physics::body::Body;

use ui::panel::PanelState;

#[macroquad::main("N-Body Physics")]
async fn main() {
    /* ------------------ СОЗДАНИЕ МИРА ------------------ */

    let mut world = World::new(1.0);

    // Солнце
    world.add_body(Body {
        mass: 1.0,
        position: Vec2::zero(),
        velocity: Vec2::zero(),
        acceleration: Vec2::zero(),
        trail: Vec::new(),
    });

    // Земля
    world.add_body(Body {
        mass: 0.000003,
        position: Vec2::new(1.0, 0.0),
        velocity: Vec2::new(0.0, 1.0),
        acceleration: Vec2::zero(),
        trail: Vec::new(),
    });

    // Марс
    world.add_body(Body {
        mass: 0.0000003,
        position: Vec2::new(1.5, 0.0),
        velocity: Vec2::new(0.0, (1.0_f64 / 1.5).sqrt()),
        acceleration: Vec2::zero(),
        trail: Vec::new(),
    });

    world.compute_accelerations();

    /* ------------------ UI ------------------ */

    let mut panel = PanelState::new();

    /* ------------------ ПАРАМЕТРЫ ------------------ */

    let dt = 0.002;
    let scale = 250.0;

    /* ------------------ ГЛАВНЫЙ ЦИКЛ ------------------ */

    loop {
        clear_background(BLACK);

        /* ---------- УПРАВЛЕНИЕ ИНТЕГРАТОРОМ ---------- */

        if is_key_pressed(KeyCode::Key1) {
            world.integrator = Integrator::Euler;
        }
        if is_key_pressed(KeyCode::Key2) {
            world.integrator = Integrator::Leapfrog;
        }
        if is_key_pressed(KeyCode::Key3) {
            world.integrator = Integrator::RK4;
        }

        /* ------------------ UI ------------------ */

        panel.draw(&mut world);

        /* ------------------ ФИЗИКА ------------------ */

        world.step(dt);

        for body in &mut world.bodies {
            body.trail.push(body.position);
            if body.trail.len() > 2000 {
                body.trail.remove(0);
            }
        }

        /* ------------------ ОТРИСОВКА СЛЕДОВ ------------------ */

        for body in &world.bodies {
            for p in &body.trail {
                let x = screen_width() as f64 / 2.0 + p.x * scale;
                let y = screen_height() as f64 / 2.0 + p.y * scale;
                draw_circle(x as f32, y as f32, 1.0, GRAY);
            }
        }

        /* ------------------ ОТРИСОВКА ТЕЛ ------------------ */

        for (i, body) in world.bodies.iter().enumerate() {
            let x = screen_width() as f64 / 2.0 + body.position.x * scale;
            let y = screen_height() as f64 / 2.0 + body.position.y * scale;

            let color = match i {
                0 => YELLOW,
                1 => BLUE,
                _ => RED,
            };

            draw_circle(x as f32, y as f32, 6.0 + i as f32 * 2.0, color);
        }

        /* ------------------ ОВЕРЛЕЙ ------------------ */

        draw_text(
            &format!(
                "Integrator: {:?}\nEnergy: {:.6}",
                world.integrator,
                world.total_energy()
            ),
            20.0,
            30.0,
            24.0,
            WHITE,
        );

        next_frame().await;
    }
}
