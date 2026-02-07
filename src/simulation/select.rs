use macroquad::{input::{MouseButton, is_mouse_button_pressed, mouse_position}, math::{DVec2, Vec2}};

use crate::simulation::Simulation;

impl Simulation {
    pub fn handle_select(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) && !self._click_handled {
            let mouse_pos: Vec2 = mouse_position().into();
            let rel_mouse_pos = self._camera.screen_to_world(mouse_pos);
            let rel_mouse_pos = DVec2::new(rel_mouse_pos.x as f64, rel_mouse_pos.y as f64);
            for i in 0..self._bodies.len() {
                if {
                    let body = &self._bodies[i];
                    let radius = self.visual_radius_world(body.radius);
                    let position = self.visual_position(body.position);
                    radius.powi(2) >= position.distance_squared(rel_mouse_pos)
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