use macroquad::{prelude::*, ui::{root_ui}};

use crate::simulation::Simulation;

static BUTTON_WIDTH: f32 = 80.0;
static BUTTON_HEIGHT: f32 = 30.0;
static MARGIN: f32 = 20.0;

impl Simulation {
    pub fn draw_start_stop_button(&mut self) {

        let button_x = screen_width() - BUTTON_WIDTH - MARGIN;
        let button_y = screen_height() - BUTTON_HEIGHT - MARGIN;
        
        let button_position = vec2(button_x, button_y);

        if self._running {
            if root_ui().button(Some(button_position), "Stop ") {
                self.stop();
                self.stop_mouse_propagation();
            }
            return;
        }
        if root_ui().button(Some(button_position), "Start") {
            self.start();
            self.stop_mouse_propagation();
        }

    }
}