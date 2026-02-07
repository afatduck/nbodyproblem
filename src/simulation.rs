use std::{collections::VecDeque, hash::{DefaultHasher, Hash, Hasher}};

use macroquad::{camera::{Camera2D, set_camera, set_default_camera}, math::{DVec2, Rect, Vec2, vec2}, window::{screen_height, screen_width}};

use crate::{body::Body, simulation::{collisions::CollsisionSimulation, frame_move::ScaleInterpolation, gravity::GravitySimulation, trajectory::TrajectoryVisibility}};

pub mod gravity;
pub mod collisions;
pub mod ui;
pub mod frame_move;
pub mod trajectory;
mod select;

static UPDATE_BUFFER_SIZE: usize = 1000;
static DEFAULT_MIN_BODY_RADIUS_PX: f32 = 12.0;
static VISUAL_SCALE: f64 = 100.0;

pub struct Simulation {
    _running: bool, 
    _bodies: Vec<Body>,
    _param_hash: u64,
    _update_buffer: VecDeque<Vec<Body>>,
    _time: f64,
    _timer: f32,
    _camera: Camera2D,
    _drag_start_position: Option<Vec2>,
    _scale_interpolation: Option<ScaleInterpolation>,
    _selected: Option<usize>,
    _camera_lock: Option<usize>,
    _click_handled: bool,
    _speed: f64,
    _gravity: f64,
    _min_body_radius_px: f32,
    _trajectory_visibility: TrajectoryVisibility,
    _show_names: bool
}

impl Simulation {
    pub const DT: f64 = 1e-4;

    pub fn new() -> Self {
        let mut camera = Camera2D::from_display_rect(
                Rect::new(
                    0.0, 
                    0.0, 
                    screen_width(), 
                    screen_height()
                )
            );

        camera.zoom.y = - camera.zoom.y;
        camera.target = Vec2::ZERO;

        Self { 
            _running: false,
            _bodies: Vec::new(),
            _update_buffer: VecDeque::new(),
            _param_hash: 0,
            _time: 0.0,
            _timer: 0.0,
            _camera: camera,
            _scale_interpolation: None,
            _drag_start_position: None,
            _selected: None,
            _camera_lock: None,
            _click_handled: false,
            _speed: 1.0,
            _gravity: 1.0,
            _min_body_radius_px: DEFAULT_MIN_BODY_RADIUS_PX,
            _trajectory_visibility: TrajectoryVisibility::SELECTED,
            _show_names: true
        }
    }

    pub fn add_body(&mut self, body: Body) {
        self._bodies.push(body);
    }

    pub fn remove_body(&mut self, index: usize) {
        self._selected = None;
        self._camera_lock = None;
        self._bodies.remove(index);
    }

    pub fn draw(&self) {
        self.draw_trajectories();
        for i in 0..self._bodies.len() {
            let visual_position = self.visual_position(self._bodies[i].position);
            let radius_world = self.visual_radius_world(self._bodies[i].radius);
            self._bodies[i].draw(
                &self._selected.unwrap_or(usize::MAX) == &i, 
                self._show_names,
                visual_position,
                radius_world
            );
        }
    }

    pub fn frame_update(&mut self, frame_time: f32) {
        self._click_handled = false;
        self.handle_frame_move(frame_time);

        if self.calculate_param_hash() != self._param_hash {
            self._update_buffer.clear();
        }
        self.fill_update_buffer();

        set_camera(&self._camera);
        self.draw();
        set_default_camera();

        if self._running {
            self.update_bodies(frame_time);
            self._timer += frame_time * self._speed as f32;
        }

        if let Some(lock_index) = self._camera_lock {
            let position = self.visual_position(self._bodies[lock_index].position);
            self._camera.target = vec2(position.x as f32, position.y as f32);
        }

        self._param_hash = self.calculate_param_hash();
        self.draw_ui();
        self.handle_select();
    }

    fn update_bodies(&mut self, frame_time: f32) {
        self._time += frame_time as f64 * self._speed * 0.01;
        while self._time >= Self::DT {
            self._time -= Self::DT;
            self._bodies = self._update_buffer.pop_front().unwrap_or(self._bodies.clone());
        }
    }

    fn fill_update_buffer(&mut self) {
        while self._update_buffer.len() < UPDATE_BUFFER_SIZE {
            let mut bodies = self._update_buffer.back().unwrap_or(&self._bodies).clone();
            bodies.grav_rk4_step(Self::DT, self._gravity);
            bodies.resolve_collisions();
            self._update_buffer.push_back(bodies);
        }
    }

    pub fn start(&mut self) {
        self._running = true;
    }

    pub fn stop(&mut self) {
        self._running = false;
    }

    fn calculate_param_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        (
            self._gravity.to_bits(),
            &self._bodies
        ).hash(&mut hasher);
        hasher.finish()
    }

    fn pixels_per_world(&self) -> f32 {
        let px_per_world_x = screen_width() * 0.5 * self._camera.zoom.x.abs();
        let px_per_world_y = screen_height() * 0.5 * self._camera.zoom.y.abs();
        let px_per_world = px_per_world_x.min(px_per_world_y);
        if px_per_world > 0.0 { px_per_world } else { 1.0 }
    }

    fn visual_radius_world(&self, radius: f64) -> f64 {
        let px_per_world = self.pixels_per_world();
        let screen_radius = (radius * VISUAL_SCALE) as f32 * px_per_world;
        let visible_px = screen_radius.max(self._min_body_radius_px);
        (visible_px / px_per_world) as f64
    }

    fn visual_position(&self, position: DVec2) -> DVec2 {
        position * VISUAL_SCALE
    }
}