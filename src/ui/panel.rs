use macroquad::prelude::*;

use crate::camera::Camera;

pub fn draw_panel(
    energy: f32,
    paused: bool,
    sim_speed: f32,
    auto_zoom: bool,
    sim_time: f64,
    today_positions: bool,
    camera: &Camera,
    font: Option<&Font>,
) {
    fn draw_text_ui(text: &str, x: f32, y: f32, font_size: u16, color: Color, font: Option<&Font>) {
        if let Some(f) = font {
            draw_text_ex(
                text,
                x,
                y,
                TextParams {
                    font_size,
                    font: Some(f),
                    color,
                    ..Default::default()
                },
            );
        } else {
            draw_text(text, x, y, font_size as f32, color);
        }
    }

    draw_text_ui(&format!("Energy: {:.2}", energy), 20.0, 25.0, 22, WHITE, font);
    draw_text_ui(
        &format!("Speed: {:.1}x  {}", sim_speed, if paused { "[PAUSED]" } else { "" }),
        20.0,
        50.0,
        18,
        if paused { YELLOW } else { GRAY },
        font,
    );
    draw_text_ui(
        "R reset | P panel | Z autozoom | Esc deselect | Space pause | [ ] speed | Wheel zoom | RMB pan",
        20.0,
        75.0,
        15,
        DARKGRAY,
        font,
    );
    draw_text_ui(
        &format!(
            "Zoom: {:.2}x ({})",
            camera.scale,
            if auto_zoom { "AUTO" } else { "MANUAL" }
        ),
        20.0,
        100.0,
        16,
        DARKGRAY,
        font,
    );

    // Таймер симуляции
    let total_seconds = sim_time as i64;
    let seconds = total_seconds % 60;
    let minutes = (total_seconds / 60) % 60;
    let hours = (total_seconds / 3600) % 24;
    let days = total_seconds / 86_400;

    let time_str = format!("T+ {}d {:02}:{:02}:{:02}", days, hours, minutes, seconds);
    draw_text_ui(&time_str, 20.0, 125.0, 16, DARKGRAY, font);

    let mode_str = if today_positions {
        "Планеты: положение на сегодня (T)"
    } else {
        "Планеты: стандартное положение (T)"
    };
    draw_text_ui(mode_str, 20.0, 145.0, 14, DARKGRAY, font);
}
