use macroquad::{input::{MouseButton, is_mouse_button_pressed, mouse_position}, math::Vec2};

use crate::simulation::Simulation;

impl Simulation {
    pub fn handle_select(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) && !self._click_handled {
            let mouse_pos: Vec2 = mouse_position().into();
            let rel_mouse_pos = self._camera.screen_to_world(mouse_pos);
            for i in 0..self._bodies.len() {
                if {
                    let body = &self._bodies[i];
                    body.radius.powi(2) >= body.position.distance_squared(rel_mouse_pos)
                } {
                    self._selected = Some(i);
                    break;
                }
                self._selected = None;
            }
        }  
    }

    pub fn select_last(&mut self) {
        self._selected = Some(self._bodies.len() - 1);
    }
}