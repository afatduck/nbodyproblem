use macroquad::{color::Color, math::Vec2, shapes::draw_circle};

use crate::{DT};
const GRAVITATIONAL_CONSTANT: f32 = 6.674e2;
const RESTITUTION_COEFFICIENT: f32 = 1.0;

#[derive(Clone)]
pub struct Body {
    pub position: Vec2,
    pub velocity: Vec2,
    // pub acceleration: Vec2,
    pub mass: f32,
    pub radius: f32,
}

impl Body {
    fn update_position(&mut self) {
        self.position += self.velocity * DT;
    }

    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, Color::from_hex(0x225588));
    }

    fn calculate_gravitational_acceleration(&self, bodies: &Vec<Body>) -> Vec2 {
        let mut aggregated_acceleration = Vec2::ZERO;
        for body in bodies {
            if std::ptr::eq(self, body) { continue; }
            // We excluse self.mass from the equation as it would have to be later devided anyways.
            let force = GRAVITATIONAL_CONSTANT * body.mass / self.position.distance(body.position);
            let acceleration = (body.position - self.position).normalize() * force;
            aggregated_acceleration += acceleration;
        }
        aggregated_acceleration
    }
}

pub trait BodyVec {
    fn update(&mut self);
    fn leapfrog(&mut self);
    fn draw(&self);
    fn resolve_collisions(&mut self);
}

impl BodyVec for Vec<Body> {
    fn update(&mut self) {
        for body in self.iter_mut() {
            body.update_position();
        }
        self.resolve_collisions();
        for i in 0..self.len() {
            let gravity_acceleration = self[i].calculate_gravitational_acceleration(self);
            self[i].velocity += gravity_acceleration * DT;
        }
    }

    fn draw(&self) {
        for body in self {
            body.draw();
        }
    }

    fn leapfrog(&mut self) {
        for i in 0..self.len() {
            let gravity_acceleration = self[i].calculate_gravitational_acceleration(self);
            self[i].velocity += gravity_acceleration * DT / 2.0;
        }
    }
    
    fn resolve_collisions(&mut self) {
        let len = self.len();
        for i in 0..len-1 {
            for j in i+1..len {
                let body_a = &self[i];
                let body_b = &self[j];

                let distance = body_a.position.distance(body_b.position);
                let penetration_depth = body_a.radius + body_b.radius - distance;

                if penetration_depth >= 0.0 {
                    let collision_normal = (body_a.position - body_b.position).normalize();
                    let total_mass = body_a.mass + body_b.mass;
                    let move_a = penetration_depth * body_b.mass / total_mass;
                    let move_b = penetration_depth * body_a.mass / total_mass;
                    let new_position_a = body_a.position + collision_normal * move_a;
                    let new_position_b = body_b.position - collision_normal * move_b;

                    let relative_velocity = body_a.velocity - body_b.velocity;
                    let normal_component = relative_velocity.dot(collision_normal);
                    let impulse_magnitude = - ((1.0 + RESTITUTION_COEFFICIENT) * normal_component) / ((1.0 / body_a.mass) + (1.0 / body_b.mass));
                    let new_velocity_a = body_a.velocity + collision_normal * impulse_magnitude / body_a.mass;
                    let new_velocity_b = body_b.velocity - collision_normal * impulse_magnitude / body_b.mass;

                    self[i].position = new_position_a;
                    self[j].position = new_position_b;
                    self[i].velocity = new_velocity_a;
                    self[j].velocity = new_velocity_b;

                }        

            }
        }
    }

    
}