use macroquad::{input::{MouseButton, is_mouse_button_pressed, mouse_position}, math::Vec2};

use crate::simulation::Simulation;

impl Simulation {
    pub fn handle_select(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos: Vec2 = mouse_position().into();
            for i in 0..self._bodies.len() {
                if {
                    let body = &self._bodies[i];
                    body.radius.powi(2) >= body.position.distance_squared(mouse_pos)
                } {
                    self._selected = Some(i);
                    break;
                }
                self._selected = None;
            }
        }  
    }
}