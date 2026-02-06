use std::{collections::VecDeque, hash::{DefaultHasher, Hash, Hasher}};

use macroquad::{camera::{Camera2D, set_camera, set_default_camera}, math::{Rect, Vec2}, window::{screen_height, screen_width}};

use crate::{body::Body, simulation::{collisions::CollsisionSimulation, frame_move::ScaleInterpolation, gravity::GravitySimulation, trajectory::TrajectoryVisibility}};

pub mod gravity;
pub mod collisions;
pub mod ui;
pub mod frame_move;
pub mod trajectory;
mod select;

static UPDATE_BUFFER_SIZE: usize = 50000;

pub struct Simulation {
    _running: bool, 
    _bodies: Vec<Body>,
    _param_hash: u64,
    _update_buffer: VecDeque<Vec<Body>>,
    _time: f32,
    _camera: Camera2D,
    _drag_start_position: Option<Vec2>,
    _scale_interpolation: Option<ScaleInterpolation>,
    _selected: Option<usize>,
    _camera_lock: Option<usize>,
    _click_handled: bool,
    _speed: f32,
    _gravity: f32,
    _restitution: f32,
    _trajectory_visibility: TrajectoryVisibility,
    _show_names: bool
}

impl Simulation {
    pub const DT: f32 = 2e-3;

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

        Self { 
            _running: false,
            _bodies: Vec::new(),
            _update_buffer: VecDeque::new(),
            _param_hash: 0,
            _time: 0.0,
            _camera: camera,
            _scale_interpolation: None,
            _drag_start_position: None,
            _selected: None,
            _camera_lock: None,
            _click_handled: false,
            _speed: 1.0,
            _gravity: 1.0,
            _restitution: 1.0,
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
            self._bodies[i].draw(
                &self._selected.unwrap_or(usize::MAX) == &i, 
                self._show_names
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

        self.update_bodies(frame_time);

        if let Some(lock_index) = self._camera_lock {
            self._camera.target = self._bodies[lock_index].position;
        }

        self._param_hash = self.calculate_param_hash();
        self.draw_ui();
        self.handle_select();
    }

    fn update_bodies(&mut self, frame_time: f32) {
        if self._running {
            self._time += frame_time * self._speed;
            while self._time >= Self::DT {
                self._time -= Self::DT;
                self._bodies = self._update_buffer.pop_front().unwrap_or(self._bodies.clone());
            }
        }
    }

    fn fill_update_buffer(&mut self) {
        while self._update_buffer.len() < UPDATE_BUFFER_SIZE {
            let mut bodies = self._update_buffer.back().unwrap_or(&self._bodies).clone();
            bodies.grav_update_positions(Self::DT);
            bodies.resolve_collisions(self._restitution);
            bodies.grav_update_velocities(Self::DT, self._gravity);
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
            self._restitution.to_bits(),
            &self._bodies
        ).hash(&mut hasher);
        hasher.finish()
    }
}