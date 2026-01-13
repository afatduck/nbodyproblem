use macroquad::math::Vec2;

use crate::{body::Body};

static G: f32 = 1e2;

pub trait GravitySimulation {
    fn grav_update_positions(&mut self, dt: f32);

    fn grav_update_velocities(&mut self, dt: f32, gravity: f32);

    fn calculate_gravitational_acceleration(&self, body: &Body, gravity: f32) -> Vec2;
}

impl GravitySimulation for Vec<Body> {
    fn grav_update_positions(&mut self, dt: f32) {
        for body in self{
            body.position += body.velocity * dt;
        }
    }

    fn grav_update_velocities(&mut self, dt: f32, gravity: f32) {
        for i in 0..self.len() {
            let acc = self[i].acceleration;
            let acc_new = self.calculate_gravitational_acceleration(&self[i], gravity);
            self[i].velocity += (acc + acc_new) * dt * 0.5;
            self[i].acceleration = acc_new; 
        }
    }

    fn calculate_gravitational_acceleration(&self, body: &Body, gravity: f32) -> Vec2 {
        let mut aggregated_acceleration = Vec2::ZERO;
        for other in self {
            if std::ptr::eq(body, other) { continue; }
            let force = G * gravity * other.mass / body.position.distance_squared(other.position);
            let acceleration = (other.position - body.position).normalize() * force;
            aggregated_acceleration += acceleration;
        }
        aggregated_acceleration
    }
}