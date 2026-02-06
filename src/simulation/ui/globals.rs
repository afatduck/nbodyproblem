use macroquad::{math::vec2, ui::{hash, root_ui}, window::screen_width};

use crate::simulation::Simulation;

static MIN_SPEED: f32 = 1.0;
static MAX_SPEED: f32 = 100.0;
static MIN_GRAVITY: f32 = 0.0;
static MAX_GRAVITY: f32 = 10.0;

impl Simulation {
    pub fn draw_global_sliders(&mut self) {
        let position = vec2(screen_width() - 220.0, 20.0);
        let size = vec2(200.0, 100.0);
        self.register_capture_window(&position, &size);
        root_ui().window(hash!(), position, size, |ui| {
            ui.label(None, "Simulation Speed");

            ui.slider(
                hash!(), 
                "Speed", 
                MIN_SPEED..MAX_SPEED, 
                &mut self._speed
            );

            ui.label(None, "Gravity Factor");
            ui.slider(
                hash!(), 
                "Gravity", 
                MIN_GRAVITY..MAX_GRAVITY, 
                &mut self._gravity
            );
        });
    }
}

