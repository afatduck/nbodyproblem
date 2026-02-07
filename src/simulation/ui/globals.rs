use macroquad::{math::vec2, ui::{hash, root_ui}, window::screen_width};

use crate::simulation::Simulation;

static MIN_SPEED: f32 = 1.0;
static MAX_SPEED: f32 = 100.0;
static MIN_GRAVITY: f32 = 0.0;
static MAX_GRAVITY: f32 = 10.0;
static MIN_BODY_RADIUS_PX: f32 = 0.0;
static MAX_BODY_RADIUS_PX: f32 = 20.0;

impl Simulation {
    pub fn draw_global_sliders(&mut self) {
        let position = vec2(screen_width() - 220.0, 20.0);
        let size = vec2(200.0, 150.0);
        self.register_capture_window(&position, &size);
        root_ui().window(hash!(), position, size, |ui| {
            ui.label(None, "Simulation Speed");

            let mut speed = self._speed as f32;

            ui.slider(
                hash!(), 
                "Speed", 
                MIN_SPEED..MAX_SPEED, 
                &mut speed
            );

            self._speed = speed as f64;

            ui.label(None, "Gravity Factor");
            let mut gravity = self._gravity as f32;
            ui.slider(
                hash!(), 
                "Gravity", 
                MIN_GRAVITY..MAX_GRAVITY, 
                &mut gravity
            );
            self._gravity = gravity as f64;

            ui.label(None, "Min Body Radius (px)");
            ui.slider(
                hash!(),
                "Min Radius",
                MIN_BODY_RADIUS_PX..MAX_BODY_RADIUS_PX,
                &mut self._min_body_radius_px
            );
        });
    }
}

