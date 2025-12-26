use macroquad::{input::{MouseButton, is_mouse_button_pressed, mouse_position}, math::Vec2};

use crate::simulation::Simulation;

impl Simulation {
    pub fn stop_mouse_propagation(&mut self) {
        self._click_handled = true;
    }

    pub fn register_capture_window(&mut self, position: &Vec2, size: &Vec2) {
        let x_min = position.x;
        let y_min = position.y;
        let x_max = x_min + size.x;
        let y_max = y_min + size.y;
        let m_pos = mouse_position();
        if is_mouse_button_pressed(MouseButton::Left) &&
            m_pos.0 >= x_min && m_pos.0 <= x_max &&
            m_pos.1 >= y_min && m_pos.1 <= y_max {
                self.stop_mouse_propagation();
        }
    }
} 