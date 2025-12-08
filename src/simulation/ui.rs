use crate::simulation::{Simulation};

mod start_stop;
mod globals;

impl Simulation {
    pub fn draw_ui(&mut self) {
        self.draw_start_stop_button();
        self.draw_global_sliders();
    }
}

