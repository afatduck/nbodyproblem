use crate::simulation::{Simulation, ui::start_stop::SimulationUIStartStop};

mod start_stop;

pub trait SimulationUI {
    fn draw_ui(&mut self);
}

impl SimulationUI for Simulation {
    fn draw_ui(&mut self) {
        self.draw_start_stop_button();
    }
}

