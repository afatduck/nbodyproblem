use macroquad::math::Vec2;

use crate::{body::Body, simulation::Simulation};

impl Simulation {
    pub fn grav_update_positions(&mut self) {
        for body in &mut self._bodies {
            body.position += body.velocity * Self::DT;
        }
    }

    pub fn grav_update_velocities(&mut self) {
        for i in 0..self._bodies.len() {
            let gravity_acceleration = self.calculate_gravitational_acceleration(&self._bodies[i], &self._bodies);
            self._bodies[i].velocity += gravity_acceleration * Self::DT;
        }
    }

    pub fn leapfrog(&mut self, back: bool) {
        // when starting and stopping the simulation we need to perform the leapfrog on all new or changed
        // bodies, so instead of keeping track of them we can just leapfrog back before stopping and then
        // leapfrog forward all bodies when starting again so starting and stopping doesnt influcene the simulation 
        let sign: f32 = if back { -1.0 } else { 1.0 };
        for i in 0..self._bodies.len() {
            let gravity_acceleration = self.calculate_gravitational_acceleration(&self._bodies[i], &self._bodies);
            self._bodies[i].velocity += gravity_acceleration * Self::DT * 0.5 * sign;
        }
    }

    fn calculate_gravitational_acceleration(&self, body: &Body, others: &Vec<Body>) -> Vec2 {
        let mut aggregated_acceleration = Vec2::ZERO;
        for other in others {
            if std::ptr::eq(body, other) { continue; }
            // We excluse self.mass from the equation as it would have to be later devided anyways.
            let force = self._gravity * other.mass / body.position.distance(other.position);
            let acceleration = (other.position - body.position).normalize() * force;
            aggregated_acceleration += acceleration;
        }
        aggregated_acceleration
    }
}