use crate::physics::world::World;
use macroquad::ui::root_ui;

pub struct PanelState {
    pub selected_body: usize,
}

impl PanelState {
    pub fn new() -> Self {
        Self { selected_body: 0 }
    }

    pub fn draw(&mut self, world: &mut World) {
        root_ui().label(None, "Planet editor");
        // sliders позже
    }
}
