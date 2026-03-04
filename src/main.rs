use macroquad::prelude::*;
use chrono::{NaiveDate, Local};

mod camera;
mod nasa_data;
mod physics;
mod ui;

use camera::Camera;
use nasa_data::NasaData;
use physics::body::Body;
use physics::world::{World, G, MAX_PHYSICS_STEP};
use ui::body_panel::draw_body_panel;
use ui::panel::draw_panel;
use ui::control_panel::draw_control_panel;

fn orbital_velocity(mass_central: f32, r: f32) -> f32 {
    (G * mass_central / r).sqrt()
}

fn days_since_epoch() -> f64 {
    let today = Local::now().date_naive();
    let epoch = NaiveDate::from_ymd_opt(2000, 1, 1).expect("invalid epoch date");
    (today - epoch).num_days() as f64
}

fn create_solar_system(world: &mut World, use_today_positions: bool) {
    let mass_sun = 100000.0;

    world.add_body(
        Body::new(
            "Солнце",
            Vec2::ZERO,
            Vec2::ZERO,
            mass_sun,
            25.0,
            YELLOW,
        )
        .with_nasa(NasaData::sun()),
    );

    let names = [
        "Меркурий", "Венера", "Земля", "Марс",
        "Юпитер", "Сатурн", "Уран", "Нептун",
    ];
    let distances = [45.0, 65.0, 85.0, 110.0, 150.0, 200.0, 250.0, 300.0];
    let masses = [0.5, 1.2, 1.5, 0.8, 8.0, 6.0, 4.0, 3.5];
    let radii = [3.0, 5.0, 5.5, 4.0, 12.0, 10.0, 7.0, 7.0];
    let colors = [
        Color::new(0.7, 0.7, 0.7, 1.0),
        Color::new(1.0, 0.9, 0.6, 1.0),
        Color::new(0.3, 0.5, 1.0, 1.0),
        Color::new(1.0, 0.4, 0.3, 1.0),
        Color::new(0.9, 0.7, 0.4, 1.0),
        Color::new(0.95, 0.85, 0.5, 1.0),
        Color::new(0.6, 0.85, 1.0, 1.0),
        Color::new(0.3, 0.5, 1.0, 1.0),
    ];
    let nasa_data = [
        NasaData::mercury(),
        NasaData::venus(),
        NasaData::earth(),
        NasaData::mars(),
        NasaData::jupiter(),
        NasaData::saturn(),
        NasaData::uranus(),
        NasaData::neptune(),
    ];

    let days = if use_today_positions {
        Some(days_since_epoch())
    } else {
        None
    };

    // Небольшие фазовые сдвиги, чтобы в момент эпохи планеты не стояли на одной прямой.
    let phase_offsets = [
        0.0_f32,
        0.4,
        1.0,
        1.8,
        3.0,
        3.8,
        4.5,
        5.2,
    ];

    let mut earth_index: Option<usize> = None;

    for i in 0..names.len() {
        let r = distances[i];
        let v = orbital_velocity(mass_sun, r);

        let angle = if let Some(d) = days {
            if let Some(period_years) = nasa_data[i].orbital_period_years {
                let period_days = period_years * 365.25;
                let revolutions = d / period_days;
                let theta = (revolutions * std::f64::consts::TAU) as f32 + phase_offsets[i];
                theta
            } else {
                phase_offsets[i]
            }
        } else {
            phase_offsets[i]
        };

        let pos = Vec2::new(
            r * angle.cos(),
            r * angle.sin(),
        );
        let vel_dir = Vec2::new(
            -angle.sin(),
            angle.cos(),
        );
        let vel = vel_dir * v;

        world.add_body(
            Body::new(
                names[i],
                pos,
                vel,
                masses[i],
                radii[i],
                colors[i],
            )
            .with_nasa(nasa_data[i].clone()),
        );

        if names[i] == "Земля" {
            earth_index = Some(world.bodies.len() - 1);
        }
    }

    // Луна: орбита вокруг Земли, которая сама обращается вокруг Солнца.
    if let Some(e_idx) = earth_index {
        if let Some(earth) = world.bodies.get(e_idx).cloned() {
            let mut offset_dir = earth.position;
            if offset_dir.length_squared() < 1e-3 {
                offset_dir = Vec2::new(1.0, 0.0);
            }
            offset_dir = offset_dir.normalize();

            let moon_r = 12.0;
            let moon_pos = earth.position + offset_dir * moon_r;

            // Скорость Луны = скорость Земли + орбитальная скорость вокруг Земли
            let tangential = Vec2::new(-offset_dir.y, offset_dir.x);
            let v_rel = (G * earth.mass / moon_r).sqrt();
            let moon_vel = earth.velocity + tangential * v_rel;

            world.add_body(
                Body::new(
                    "Луна",
                    moon_pos,
                    moon_vel,
                    0.1,
                    2.0,
                    Color::new(0.8, 0.8, 0.9, 1.0),
                )
                .with_nasa(NasaData {
                    mass_kg: 7.342e22,
                    radius_km: 1_737.4,
                    semimajor_axis_au: None,
                    orbital_period_years: None,
                    orbital_velocity_km_s: None,
                }),
            );
        }
    }

    // Пара комет с вытянутыми орбитами вокруг Солнца (приближённо).
    let comet_distance = 400.0;
    let comet_speed = orbital_velocity(mass_sun, comet_distance) * 0.6;
    world.add_body(Body::new(
        "Комета 1",
        Vec2::new(-comet_distance, 0.0),
        Vec2::new(0.0, comet_speed),
        0.05,
        2.5,
        Color::new(0.7, 1.0, 1.0, 1.0),
    ));
    world.add_body(Body::new(
        "Комета 2",
        Vec2::new(0.0, -comet_distance * 0.8),
        Vec2::new(comet_speed * 0.8, 0.0),
        0.03,
        2.0,
        Color::new(0.9, 0.9, 1.0, 1.0),
    ));

    world.compute_accelerations();
}

/// Пытается загрузить шрифт с поддержкой кириллицы.
async fn load_cyrillic_font() -> Option<Font> {
    const FONT_PATHS: &[&str] = &[
        "/usr/share/fonts/Adwaita/AdwaitaSans-Regular.ttf",
        "/usr/share/fonts/TTF/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/noto/NotoSans-Regular.ttf",
        "assets/DejaVuSans.ttf",
    ];

    for path in FONT_PATHS {
        if let Ok(font) = load_ttf_font(path).await {
            return Some(font);
        }
    }
    None
}

#[macroquad::main("Solar System")]
async fn main() {
    let font = load_cyrillic_font().await;

    let mut world = World::new();
    let mut use_today_positions = true;
    create_solar_system(&mut world, use_today_positions);
    let mut selected_body: Option<usize> = None;
    let mut panel_visible = true;
    let mut camera = Camera::new();
    let mut paused = false;
    let mut sim_speed: f32 = 1.0;
    let mut pan_start: Option<Vec2> = None;
    let mut auto_zoom = false;
    let mut sim_time: f64 = 0.0;

    loop {
        clear_background(Color::new(0.02, 0.02, 0.05, 1.0));

        let dt = get_frame_time();
        let mouse_pos = mouse_position().into();
        let mouse_world = camera.screen_to_world(mouse_pos);

        // === INPUT ===
        if is_key_pressed(KeyCode::R) {
            world = World::new();
            create_solar_system(&mut world, use_today_positions);
            selected_body = None;
            sim_time = 0.0;
        }
        if is_key_pressed(KeyCode::Escape) {
            selected_body = None;
        }
        if is_key_pressed(KeyCode::P) {
            panel_visible = !panel_visible;
        }
        if is_key_pressed(KeyCode::Z) {
            auto_zoom = !auto_zoom;
        }
        if is_key_pressed(KeyCode::T) {
            use_today_positions = !use_today_positions;
            world = World::new();
            create_solar_system(&mut world, use_today_positions);
            selected_body = None;
            sim_time = 0.0;
        }
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::LeftBracket) {
            sim_speed = (sim_speed / 1.5).max(0.1);
        }
        if is_key_pressed(KeyCode::RightBracket) {
            sim_speed = (sim_speed * 1.5).min(50.0);
        }

        // Zoom (колёсико)
        let (_, wheel) = mouse_wheel();
        if wheel != 0.0 {
            auto_zoom = false;
            camera.zoom(wheel, mouse_pos);
        }

        // Pan (правый клик + drag)
        if is_mouse_button_pressed(MouseButton::Right) {
            pan_start = Some(mouse_pos);
        }
        if is_mouse_button_down(MouseButton::Right) {
            if let Some(start) = pan_start {
                let delta = mouse_pos - start;
                camera.pan(delta);
            }
            pan_start = Some(mouse_pos);
        } else {
            pan_start = None;
        }

        // Клик по телу (выбор/снятие) — не при панели
        const PANEL_WIDTH: f32 = 240.0;
        let on_panel = panel_visible && mouse_position().0 > screen_width() - PANEL_WIDTH;
        if is_mouse_button_pressed(MouseButton::Left)
            && !is_mouse_button_down(MouseButton::Right)
            && !on_panel
        {
            let click_world = mouse_world;
            let mut hit = false;
            for (i, body) in world.bodies.iter().enumerate() {
                let dist = (body.position - click_world).length();
                let hit_radius = body.radius.max(10.0);
                if dist < hit_radius {
                    selected_body = Some(i);
                    hit = true;
                    break;
                }
            }
            if !hit {
                selected_body = None; // снять выделение при клике по пустоте
            }
        }

        // === SIMULATION (субстеппинг — орбиты стабильны при любой скорости) ===
        if !paused {
            let total_dt = dt * sim_speed;
            let mut remaining = total_dt;
            while remaining > 0.0 {
                let step = remaining.min(MAX_PHYSICS_STEP);
                world.step(step);
                remaining -= step;
            }
            sim_time += total_dt as f64;
        }

        // === КАМЕРА ===
        if let Some(idx) = selected_body {
            if let Some(body) = world.bodies.get(idx) {
                // Следуем за выбранным телом.
                let target_offset = body.position;

                // Если включён автозум — масштаб подстраиваем под размеры системы,
                // иначе оставляем текущий (управляется колёсиком).
                let target_scale = if auto_zoom {
                    let (min, max) = world
                        .bodies
                        .iter()
                        .fold(
                            (Vec2::new(f32::INFINITY, f32::INFINITY), Vec2::new(f32::NEG_INFINITY, f32::NEG_INFINITY)),
                            |(min, max), b| {
                                (
                                    Vec2::new(min.x.min(b.position.x), min.y.min(b.position.y)),
                                    Vec2::new(max.x.max(b.position.x), max.y.max(b.position.y)),
                                )
                            },
                        );

                    let extents = max - min;
                    let view_w = screen_width() - if panel_visible { 240.0 } else { 0.0 };
                    let view_h = screen_height();
                    let margin = 0.2;
                    let scale_x = if extents.x.abs() < f32::EPSILON {
                        camera.scale
                    } else {
                        view_w * (1.0 - margin) / extents.x.abs()
                    };
                    let scale_y = if extents.y.abs() < f32::EPSILON {
                        camera.scale
                    } else {
                        view_h * (1.0 - margin) / extents.y.abs()
                    };

                    scale_x.min(scale_y).clamp(0.2, 10.0)
                } else {
                    camera.scale
                };

                camera.smooth_follow(target_offset, target_scale, 0.08);
            }
        } else if auto_zoom && !world.bodies.is_empty() {
            // Нет выбранного тела — центрируем и масштабируем так, чтобы
            // вся система помещалась в экран.
            let (min, max) = world
                .bodies
                .iter()
                .fold(
                    (Vec2::new(f32::INFINITY, f32::INFINITY), Vec2::new(f32::NEG_INFINITY, f32::NEG_INFINITY)),
                    |(min, max), b| {
                        (
                            Vec2::new(min.x.min(b.position.x), min.y.min(b.position.y)),
                            Vec2::new(max.x.max(b.position.x), max.y.max(b.position.y)),
                        )
                    },
                );

            let center = (min + max) * 0.5;
            let extents = max - min;
            let view_w = screen_width() - if panel_visible { 240.0 } else { 0.0 };
            let view_h = screen_height();
            let margin = 0.2;
            let scale_x = if extents.x.abs() < f32::EPSILON {
                camera.scale
            } else {
                view_w * (1.0 - margin) / extents.x.abs()
            };
            let scale_y = if extents.y.abs() < f32::EPSILON {
                camera.scale
            } else {
                view_h * (1.0 - margin) / extents.y.abs()
            };

            let target_scale = scale_x.min(scale_y).clamp(0.2, 10.0);
            camera.smooth_follow(center, target_scale, 0.06);
        }

        // === DRAW ===
        for (i, body) in world.bodies.iter().enumerate() {
            let pos_screen = camera.world_to_screen(body.position);
            let radius_screen = camera.scale_radius(body.radius).max(2.0);

            // Траектория (цвет тела с затуханием)
            for j in 1..body.trail.len() {
                let p0 = camera.world_to_screen(body.trail[j - 1]);
                let p1 = camera.world_to_screen(body.trail[j]);
                let alpha = (j as f32 / body.trail.len() as f32) * 0.6 + 0.2;
                let trail_color = Color::new(
                    body.color.r,
                    body.color.g,
                    body.color.b,
                    alpha,
                );
                draw_line(p0.x, p0.y, p1.x, p1.y, 1.5, trail_color);
            }

            // Тело
            let is_selected = selected_body == Some(i);
            let is_sun = i == 0;

            if is_sun {
                // Солнце с свечением
                for (r, a) in [(radius_screen * 2.5, 0.15), (radius_screen * 1.5, 0.4)] {
                    draw_circle(pos_screen.x, pos_screen.y, r, Color::new(1.0, 0.9, 0.3, a));
                }
            }

            draw_circle(pos_screen.x, pos_screen.y, radius_screen, body.color);

            if is_selected {
                draw_circle_lines(
                    pos_screen.x,
                    pos_screen.y,
                    radius_screen + 3.0,
                    2.0,
                    WHITE,
                );
            }

            // Название тела
            let label_y = pos_screen.y - radius_screen - 8.0;
            if let Some(f) = font.as_ref() {
                draw_text_ex(
                    &body.name,
                    pos_screen.x + radius_screen + 6.0,
                    label_y,
                    TextParams {
                        font_size: 14,
                        font: Some(f),
                        color: WHITE,
                        ..Default::default()
                    },
                );
            } else {
                draw_text(
                    &body.name,
                    pos_screen.x + radius_screen + 6.0,
                    label_y,
                    14.0,
                    WHITE,
                );
            }
        }

        draw_panel(
            world.total_energy(),
            paused,
            sim_speed,
            auto_zoom,
            sim_time,
            use_today_positions,
            &camera,
            font.as_ref(),
        );

        // Отдельная панель управления
        let prev_today = use_today_positions;
        draw_control_panel(
            &mut paused,
            &mut sim_speed,
            &mut auto_zoom,
            &mut use_today_positions,
            font.as_ref(),
        );
        if use_today_positions != prev_today {
            world = World::new();
            create_solar_system(&mut world, use_today_positions);
            selected_body = None;
            sim_time = 0.0;
        }

        if panel_visible {
            if let Some(idx) = draw_body_panel(&world.bodies, selected_body, font.as_ref()) {
                selected_body = Some(idx);
            }
        }

        next_frame().await;
    }
}
