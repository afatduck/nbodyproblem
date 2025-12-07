use macroquad::{prelude::*, ui::{Skin, root_ui}};

use crate::simulation::Simulation;

pub trait SimulationUIStartStop {
    fn draw_start_stop_button(&mut self);
}

impl SimulationUIStartStop for Simulation {
    fn draw_start_stop_button(&mut self) {
        let button_width = 150.0;
        let button_height = 40.0;

        let margin = 20.0;
        let button_x = screen_width() - button_width - margin;
        let button_y = screen_height() - button_height - margin;
        
        let button_position = vec2(button_x, button_y);

        if self._running {
            if root_ui().button(Some(button_position), "Stop") {
                self.stop();
            }
            return;
        }
        if root_ui().button(Some(button_position), "Start") {
            self.start();
        }

    }
}