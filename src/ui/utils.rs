//! Общие утилиты для UI модуля.

use macroquad::prelude::*;

/// Рисует текст с поддержкой пользовательского шрифта.
/// Если шрифт не предоставлен, используется стандартный.
#[inline]
pub fn draw_text_ui(text: &str, x: f32, y: f32, font_size: u16, color: Color, font: Option<&Font>) {
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
