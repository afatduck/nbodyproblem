use macroquad::{input::{MouseButton, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, mouse_position}, math::Vec2};

use crate::simulation::Simulation;

pub trait SimulationFrameMove {
    fn handle_frame_move(&mut self);
}

impl SimulationFrameMove for Simulation {
    fn handle_frame_move(&mut self) {
        if is_mouse_button_pressed(MouseButton::Middle) {
            self._drag_start_position = Some(mouse_position().into());  
        }
        if is_mouse_button_released(MouseButton::Middle) {
            self._drag_start_position = None;
        }
        if is_mouse_button_down(MouseButton::Middle) {
            if let Some(last_pos) = self._drag_start_position {
                let mouse_pos: Vec2 = mouse_position().into();
                let delta = mouse_pos - last_pos;
                self._position += delta;
                self._drag_start_position = Some(mouse_pos);  
            }
        }
    }
}