use macroquad::prelude::*;

use crate::physics::body::Body;
use crate::ui::draw_text_ui;

const PANEL_WIDTH: f32 = 240.0;
const ROW_HEIGHT: f32 = 28.0;
const PADDING: f32 = 12.0;
const FONT_SIZE_LIST: u16 = 18;
const FONT_SIZE_TITLE: u16 = 20;
const FONT_SIZE_PARAM: u16 = 16;

/// Рисует панель выбора тел и возвращает индекс выбранного при клике.
/// selected — текущий выбранный индекс.
/// font — шрифт с поддержкой кириллицы (если None — используется стандартный).
pub fn draw_body_panel(
    bodies: &[Body],
    selected: Option<usize>,
    font: Option<&Font>,
) -> Option<usize> {
    let screen_w = screen_width();
    let panel_x = screen_w - PANEL_WIDTH;

    // Фон панели
    draw_rectangle(panel_x, 0.0, PANEL_WIDTH, screen_height(), Color::new(0.1, 0.1, 0.15, 0.95));
    draw_rectangle(panel_x, 0.0, 2.0, screen_height(), Color::new(0.3, 0.3, 0.4, 1.0));

    let mut clicked_index = None;
    let (mx, my) = mouse_position();

    // Заголовок
    draw_text_ui("ТЕЛА", panel_x + PADDING, 30.0, FONT_SIZE_TITLE, WHITE, font);

    let list_start_y = 50.0;

    for (i, body) in bodies.iter().enumerate() {
        let y = list_start_y + i as f32 * ROW_HEIGHT;
        let rect = Rect::new(panel_x, y - ROW_HEIGHT + 4.0, PANEL_WIDTH, ROW_HEIGHT);

        let is_hovered = rect.contains(Vec2::new(mx, my));
        let is_selected = selected == Some(i);

        if is_selected {
            draw_rectangle(
                panel_x + 4.0,
                y - ROW_HEIGHT + 6.0,
                PANEL_WIDTH - 8.0,
                ROW_HEIGHT - 4.0,
                Color::new(0.25, 0.35, 0.5, 0.8),
            );
        } else if is_hovered {
            draw_rectangle(
                panel_x + 4.0,
                y - ROW_HEIGHT + 6.0,
                PANEL_WIDTH - 8.0,
                ROW_HEIGHT - 4.0,
                Color::new(0.2, 0.2, 0.25, 0.8),
            );
        }

        if is_mouse_button_pressed(MouseButton::Left) && rect.contains(Vec2::new(mx, my)) {
            clicked_index = Some(i);
        }

        // Цветной кружок
        draw_circle(panel_x + PADDING + 8.0, y - 10.0, 6.0, body.color);

        // Название
        let text_color = if is_selected { WHITE } else { LIGHTGRAY };
        draw_text_ui(
            &body.name,
            panel_x + PADDING + 28.0,
            y - 2.0,
            FONT_SIZE_LIST,
            text_color,
            font,
        );
    }

    // Блок параметров выбранного тела (симуляция + реальные NASA)
    if let Some(idx) = selected {
        if let Some(body) = bodies.get(idx) {
            let detail_start = list_start_y + bodies.len() as f32 * ROW_HEIGHT + 20.0;

            draw_text_ui(
                "ПАРАМЕТРЫ (NASA)",
                panel_x + PADDING,
                detail_start,
                FONT_SIZE_TITLE,
                WHITE,
                font,
            );

            let mut y = detail_start + 25.0;

            if let Some(ref nasa) = body.nasa_data {
                draw_text_ui("Масса", panel_x + PADDING, y, FONT_SIZE_PARAM, GRAY, font);
                draw_text_ui(
                    &nasa.mass_display(),
                    panel_x + PANEL_WIDTH - PADDING - 100.0,
                    y,
                    FONT_SIZE_PARAM - 1,
                    Color::new(0.6, 1.0, 0.6, 1.0),
                    font,
                );
                y += 22.0;

                draw_text_ui(
                    "Радиус",
                    panel_x + PADDING,
                    y,
                    FONT_SIZE_PARAM,
                    GRAY,
                    font,
                );
                draw_text_ui(
                    &format!("{:.0} км", nasa.radius_km),
                    panel_x + PANEL_WIDTH - PADDING - 100.0,
                    y,
                    FONT_SIZE_PARAM - 1,
                    WHITE,
                    font,
                );
                y += 22.0;

                if let Some(au) = nasa.semimajor_axis_au {
                    draw_text_ui(
                        "Расст. от Солнца",
                        panel_x + PADDING,
                        y,
                        FONT_SIZE_PARAM,
                        GRAY,
                        font,
                    );
                    draw_text_ui(
                        &format!("{:.2} а.е.", au),
                        panel_x + PANEL_WIDTH - PADDING - 100.0,
                        y,
                        FONT_SIZE_PARAM - 1,
                        WHITE,
                        font,
                    );
                    y += 22.0;
                }

                if let Some(years) = nasa.orbital_period_years {
                    draw_text_ui(
                        "Орб. период",
                        panel_x + PADDING,
                        y,
                        FONT_SIZE_PARAM,
                        GRAY,
                        font,
                    );
                    let period_str = if years >= 1.0 {
                        format!("{:.1} лет", years)
                    } else {
                        format!("{:.0} дней", years * 365.25)
                    };
                    draw_text_ui(
                        &period_str,
                        panel_x + PANEL_WIDTH - PADDING - 100.0,
                        y,
                        FONT_SIZE_PARAM - 1,
                        WHITE,
                        font,
                    );
                    y += 22.0;
                }

                if let Some(v) = nasa.orbital_velocity_km_s {
                    draw_text_ui(
                        "Орб. скорость",
                        panel_x + PADDING,
                        y,
                        FONT_SIZE_PARAM,
                        GRAY,
                        font,
                    );
                    draw_text_ui(
                        &format!("{:.1} км/с", v),
                        panel_x + PANEL_WIDTH - PADDING - 100.0,
                        y,
                        FONT_SIZE_PARAM - 1,
                        WHITE,
                        font,
                    );
                    y += 22.0;
                }

                y += 8.0;
            }

            draw_text_ui(
                "--- симуляция ---",
                panel_x + PADDING,
                y,
                FONT_SIZE_PARAM - 2,
                DARKGRAY,
                font,
            );
            y += 20.0;

            let params = [
                ("Позиция X", format!("{:.1}", body.position.x)),
                ("Позиция Y", format!("{:.1}", body.position.y)),
                ("Скорость", format!("{:.2}", body.speed())),
            ];

            for (label, value) in params {
                draw_text_ui(label, panel_x + PADDING, y, FONT_SIZE_PARAM - 1, GRAY, font);
                draw_text_ui(
                    &value,
                    panel_x + PANEL_WIDTH - PADDING - 80.0,
                    y,
                    FONT_SIZE_PARAM - 1,
                    LIGHTGRAY,
                    font,
                );
                y += 20.0;
            }
        }
    }

    clicked_index
}
