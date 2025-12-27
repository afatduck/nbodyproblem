use macroquad::{input::{MouseButton, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, mouse_position, mouse_wheel}, math::Vec2};

use crate::simulation::Simulation;

static ZOOM_FACTOR: f32 = 1.2;

pub struct ScaleInterpolation {
    _start: Vec2,
    _end: Vec2,
    _time: f32,
    pub mouse_pos: Vec2
}

impl ScaleInterpolation {
    const END_TIME: f32 = 5e-1;

    fn f(x: f32) -> f32 {
        1.0 - (1.0 - x) * (1.0 - x)
    }

    pub fn from(start: Vec2, end: Vec2, mouse_pos: Vec2) -> Self {
        Self { _start: start, _end: end, _time: 0.0, mouse_pos }
    }

    pub fn update(&mut self, df: f32) -> (Vec2, bool) {
        self._time += df;
        let x = 1.0f32.min(self._time / Self::END_TIME);
        let y = ScaleInterpolation::f(x);
        let delta = self._end - self._start;
        let zoom = self._start + delta * y;
        (zoom, y == 1.0)
    }
}

impl Simulation {
    pub fn handle_frame_move(&mut self, frame_time: f32) {
        if is_mouse_button_pressed(MouseButton::Middle) {
            self._drag_start_position = Some(self._camera.screen_to_world(mouse_position().into()));  
        }
        if is_mouse_button_released(MouseButton::Middle) {
            self._drag_start_position = None;
        }
        if is_mouse_button_down(MouseButton::Middle) && self._camera_lock == None {
            if let Some(last_pos) = self._drag_start_position {
                let mouse_pos: Vec2 = self._camera.screen_to_world(mouse_position().into());
                let delta = mouse_pos - last_pos;
                self._camera.target -= delta;
                self._drag_start_position = Some(self._camera.screen_to_world(mouse_position().into()));  
            }
        }

        let zoom = mouse_wheel().1;
        if zoom != 0.0 {
            let mut end = self._scale_interpolation.as_ref().map_or(
                self._camera.zoom,
                |s| s._end
            );
            if zoom < 0.0 {
                end /= ZOOM_FACTOR;
            }
            else if zoom > 0.0 {
                end *= ZOOM_FACTOR;
            }
            self._scale_interpolation = Some(ScaleInterpolation::from(
                self._camera.zoom, 
                end,
                mouse_position().into()
            ));
        }

        if let Some(inp) = &mut self._scale_interpolation {
            let (zoom, done) = inp.update(frame_time);

            let before = self._camera.screen_to_world(inp.mouse_pos);
            self._camera.zoom = zoom;
            let after = self._camera.screen_to_world(inp.mouse_pos);
            let delta = after - before;
            if self._camera_lock == None {
                self._camera.target -= delta;
            }

            if done {
                self._scale_interpolation = None;
            }
        }
    }

}