pub mod body;
pub mod world;

use std::collections::VecDeque;

/// Максимальная длина следа траектории
pub const MAX_TRAIL_LENGTH: usize = 500;

/// Создаёт новый кольцевой буфер для следа
pub fn new_trail() -> VecDeque<Vec2> {
    VecDeque::with_capacity(MAX_TRAIL_LENGTH)
}
