use crate::{body::Body, simulation::{collisions::CollisionSimulation, gravity::GravitySimulation, ui::SimulationUI}};

pub mod gravity;
pub mod collisions;
pub mod ui;

pub struct Simulation {
    _running: bool, 
    _bodies: Vec<Body>,
    _time: f32,

    pub speed: f32,
    pub gravity: f32,
    pub restitution: f32
}

impl Simulation {
    pub const DT: f32 = 2e-3;

    pub fn new() -> Self {
        Self { 
            _running: false,
            _bodies: Vec::new(),
            _time: 0.0,
            speed: 1.0,
            gravity: 1.0,
            restitution: 1.0
        }
    }

    pub fn add_body(&mut self, body: Body) {
        self._bodies.push(body);
    }

    fn update_bodies(&mut self) {
        self.grav_update_positions();
        self.resolve_collisions();
        self.grav_update_velocities();
    }

    pub fn draw(&self) {
        for body in &self._bodies {
            body.draw();
        }
    }

    pub fn update(&mut self, frame_time: f32) {
        if self._running {
            self._time += frame_time * self.speed;
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