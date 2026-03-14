use macroquad::prelude::*;

use crate::ui::draw_text_ui;

const PANEL_X: f32 = 20.0;
const PANEL_Y: f32 = 130.0;
const PANEL_WIDTH: f32 = 260.0;
const ROW_HEIGHT: f32 = 28.0;
const PADDING: f32 = 10.0;

fn button_row(
    row_y: &mut f32,
    mouse: Vec2,
    clicked: bool,
    label: &str,
    value: &str,
    state: bool,
    font: Option<&Font>,
) -> bool {
    let rect = Rect::new(PANEL_X, *row_y, PANEL_WIDTH, ROW_HEIGHT);
    let is_hover = rect.contains(mouse);

    if state {
        draw_rectangle(
            rect.x + 3.0,
            rect.y + 4.0,
            rect.w - 6.0,
            rect.h - 6.0,
            Color::new(0.25, 0.35, 0.5, 0.85),
        );
    } else if is_hover {
        draw_rectangle(
            rect.x + 3.0,
            rect.y + 4.0,
            rect.w - 6.0,
            rect.h - 6.0,
            Color::new(0.18, 0.2, 0.28, 0.85),
        );
    }

    draw_text_ui(
        label,
        rect.x + PADDING,
        rect.y + 19.0,
        16,
        if state { WHITE } else { LIGHTGRAY },
        font,
    );
    draw_text_ui(
        value,
        rect.x + rect.w - PADDING - 90.0,
        rect.y + 19.0,
        16,
        GRAY,
        font,
    );

    let activated = clicked && rect.contains(mouse);
    *row_y += ROW_HEIGHT;
    activated
}

/// Панель управления симуляцией (отдельное "окно" в левом верхнем углу).
///
/// Управляет состоянием через переданные &mut-параметры:
/// - paused — пауза/плей
/// - sim_speed — скорость симуляции
/// - auto_zoom — автозум камеры
/// - today_positions — старт планет по положению "на сегодня"
pub fn draw_control_panel(
    paused: &mut bool,
    sim_speed: &mut f32,
    auto_zoom: &mut bool,
    today_positions: &mut bool,
    font: Option<&Font>,
) {
    let (mx, my) = mouse_position();
    let mouse = Vec2::new(mx, my);
    let clicked = is_mouse_button_pressed(MouseButton::Left);

    // Фон панели
    draw_rectangle(
        PANEL_X,
        PANEL_Y,
        PANEL_WIDTH,
        ROW_HEIGHT * 5.0 + PADDING * 2.0,
        Color::new(0.07, 0.07, 0.12, 0.9),
    );
    draw_rectangle(
        PANEL_X,
        PANEL_Y,
        PANEL_WIDTH,
        24.0,
        Color::new(0.12, 0.15, 0.25, 1.0),
    );

    draw_text_ui(
        "Управление",
        PANEL_X + PADDING,
        PANEL_Y + 18.0,
        18,
        WHITE,
        font,
    );

    let mut row_y = PANEL_Y + 30.0;

    // Пауза
    if button_row(
        &mut row_y,
        mouse,
        clicked,
        "Пауза",
        if *paused { "ON" } else { "OFF" },
        *paused,
        font,
    ) {
        *paused = !*paused;
    }

    // Скорость
    let speed_rect = Rect::new(PANEL_X, row_y, PANEL_WIDTH, ROW_HEIGHT);
    let is_hover_speed = speed_rect.contains(mouse);
    if is_hover_speed {
        draw_rectangle(
            speed_rect.x + 3.0,
            speed_rect.y + 4.0,
            speed_rect.w - 6.0,
            speed_rect.h - 6.0,
            Color::new(0.18, 0.2, 0.28, 0.85),
        );
    }
    draw_text_ui(
        "Скорость",
        speed_rect.x + PADDING,
        speed_rect.y + 19.0,
        16,
        LIGHTGRAY,
        font,
    );
    draw_text_ui(
        &format!("{:.1}x", *sim_speed),
        speed_rect.x + speed_rect.w - PADDING - 60.0,
        speed_rect.y + 19.0,
        16,
        GRAY,
        font,
    );

    // Кнопки - и +
    let minus_rect = Rect::new(
        speed_rect.x + speed_rect.w - PADDING - 90.0,
        speed_rect.y + 6.0,
        18.0,
        ROW_HEIGHT - 12.0,
    );
    let plus_rect = Rect::new(
        speed_rect.x + speed_rect.w - PADDING - 30.0,
        speed_rect.y + 6.0,
        18.0,
        ROW_HEIGHT - 12.0,
    );

    let minus_hover = minus_rect.contains(mouse);
    let plus_hover = plus_rect.contains(mouse);

    let btn_bg = |hover: bool| {
        if hover {
            Color::new(0.35, 0.4, 0.6, 1.0)
        } else {
            Color::new(0.22, 0.26, 0.38, 1.0)
        }
    };

    draw_rectangle(
        minus_rect.x,
        minus_rect.y,
        minus_rect.w,
        minus_rect.h,
        btn_bg(minus_hover),
    );
    draw_rectangle(
        plus_rect.x,
        plus_rect.y,
        plus_rect.w,
        plus_rect.h,
        btn_bg(plus_hover),
    );

    draw_text_ui("-", minus_rect.x + 6.0, minus_rect.y + 17.0, 18, WHITE, font);
    draw_text_ui("+", plus_rect.x + 4.0, plus_rect.y + 17.0, 18, WHITE, font);

    if clicked && minus_rect.contains(mouse) {
        *sim_speed = (*sim_speed / 1.5).max(0.1);
    }
    if clicked && plus_rect.contains(mouse) {
        *sim_speed = (*sim_speed * 1.5).min(50.0);
    }

    row_y += ROW_HEIGHT;

    // Автозум
    if button_row(
        &mut row_y,
        mouse,
        clicked,
        "Автозум",
        if *auto_zoom { "AUTO" } else { "MANUAL" },
        *auto_zoom,
        font,
    ) {
        *auto_zoom = !*auto_zoom;
    }

    // Положение планет: "сегодня" / "по умолчанию"
    if button_row(
        &mut row_y,
        mouse,
        clicked,
        "Положение планет",
        if *today_positions { "СЕГОДНЯ" } else { "СТАНДАРТ" },
        *today_positions,
        font,
    ) {
        *today_positions = !*today_positions;
    }
}

