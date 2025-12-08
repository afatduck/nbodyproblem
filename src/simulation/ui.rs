use crate::simulation::{Simulation, ui::{globals::SimulationUIGlobals, start_stop::SimulationUIStartStop}};

mod start_stop;
mod globals;

pub trait SimulationUI {
    fn draw_ui(&mut self);
}

impl SimulationUI for Simulation {
    fn draw_ui(&mut self) {
        self.draw_start_stop_button();
        self.draw_global_sliders();
    }
}

