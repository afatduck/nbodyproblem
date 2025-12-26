use macroquad::{prelude::*, ui::{root_ui}};

use crate::{body::Body, simulation::Simulation};

static BUTTON_HEIGHT: f32 = 30.0;
static MARGIN: f32 = 20.0;

impl Simulation {
    pub fn draw_add_body_button(&mut self) {

        let button_x = MARGIN;
        let button_y = screen_height() - BUTTON_HEIGHT - MARGIN;
        
        let button_position = vec2(button_x, button_y);

        if self._selected == None && !self._running {
            if root_ui().button(Some(button_position), "Add body") {
                let new_body: Body = Default::default();
                self.add_body(new_body);
            }
        }
    }
}