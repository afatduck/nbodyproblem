use macroquad::math::Vec2;

use crate::{body::Body};

pub mod gravity;
pub mod collisions;
pub mod ui;
pub mod frame_move;
mod select;

pub struct Simulation {
    _running: bool, 
    _bodies: Vec<Body>,
    _time: f32,
    _position: Vec2,
    _drag_start_position: Option<Vec2>,
    _selected: Option<usize>,
    _camera_lock: Option<usize>,
    _click_handled: bool,
    _speed: f32,
    _gravity: f32,
    _restitution: f32
}

impl Simulation {
    pub const DT: f32 = 2e-3;

    pub fn new() -> Self {
        Self { 
            _running: false,
            _bodies: Vec::new(),
            _time: 0.0,
            _position: Vec2::ZERO,
            _drag_start_position: None,
            _selected: None,
            _camera_lock: None,
            _click_handled: false,
            _speed: 1.0,
            _gravity: 1.0,
            _restitution: 1.0
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

    fn update_bodies(&mut self) {
        self.grav_update_positions();
        self.resolve_collisions();
        self.grav_update_velocities();
    }

    pub fn draw(&self) {
        for i in 0..self._bodies.len() {
            self._bodies[i].draw(
                &self._position, 
                &self._selected.unwrap_or(usize::MAX) == &i
            );
        }
    }

    pub fn frame_update(&mut self, frame_time: f32) {
        self._click_handled = false;
        self.handle_frame_move();
        self.draw();
        self.update(frame_time);
        self.handle_select();
    }

    fn update(&mut self, frame_time: f32) {
        if self._running {
            self._time += frame_time * self._speed;
            while self._time >= Self::DT {
                self._time -= Self::DT;
                self.update_bodies();
            }
        }
        self.draw_ui();
    }

    pub fn start(&mut self) {
        self.leapfrog(false);
        self._running = true;
    }

    pub fn stop(&mut self) {
        self._running = false;
        self.leapfrog(false);
    }
}