use crate::simulation::{Simulation};

mod start_stop;
mod globals;
mod body_properties;
mod click_handler;
mod add_body;
mod top_switches;

impl Simulation {
    pub fn draw_ui(&mut self) {
        self.draw_start_stop_button();
        self.draw_global_sliders();
        self.draw_body_properties();
        self.draw_add_body_button();
        self.draw_top_switches();
    }
}

