use macroquad::math::DVec2;

use crate::{body::Body};

static G: f64 = 39.47841760435743;

pub trait GravitySimulation {
    fn grav_update_positions(&mut self, dt: f64);

    fn grav_rk4_step(&mut self, dt: f64, gravity: f64);

    fn calculate_gravitational_acceleration(&self, body: &Body, gravity: f64) -> DVec2;
}

impl GravitySimulation for Vec<Body> {
    fn grav_update_positions(&mut self, dt: f64) {
        for body in self{
            body.position += body.velocity * dt;
        }
    }

    fn grav_rk4_step(&mut self, dt: f64, gravity: f64) {
        let len = self.len();
        if len == 0 {
            return;
        }

        let masses: Vec<f64> = self.iter().map(|body| body.mass).collect();
        let positions: Vec<DVec2> = self.iter().map(|body| body.position).collect();
        let velocities: Vec<DVec2> = self.iter().map(|body| body.velocity).collect();

        let acc1 = compute_accelerations(&positions, &masses, gravity);
        let k1_p = velocities.clone();
        let k1_v = acc1;

        let positions2: Vec<DVec2> = positions
            .iter()
            .zip(k1_p.iter())
            .map(|(p, k)| *p + *k * (dt * 0.5))
            .collect();
        let velocities2: Vec<DVec2> = velocities
            .iter()
            .zip(k1_v.iter())
            .map(|(v, k)| *v + *k * (dt * 0.5))
            .collect();
        let k2_v = compute_accelerations(&positions2, &masses, gravity);
        let k2_p = velocities2;

        let positions3: Vec<DVec2> = positions
            .iter()
            .zip(k2_p.iter())
            .map(|(p, k)| *p + *k * (dt * 0.5))
            .collect();
        let velocities3: Vec<DVec2> = velocities
            .iter()
            .zip(k2_v.iter())
            .map(|(v, k)| *v + *k * (dt * 0.5))
            .collect();
        let k3_v = compute_accelerations(&positions3, &masses, gravity);
        let k3_p = velocities3;

        let positions4: Vec<DVec2> = positions
            .iter()
            .zip(k3_p.iter())
            .map(|(p, k)| *p + *k * dt)
            .collect();
        let velocities4: Vec<DVec2> = velocities
            .iter()
            .zip(k3_v.iter())
            .map(|(v, k)| *v + *k * dt)
            .collect();
        let k4_v = compute_accelerations(&positions4, &masses, gravity);
        let k4_p = velocities4;

        let dt_over_six = dt / 6.0;
        for i in 0..len {
            self[i].position = positions[i]
                + (k1_p[i] + k2_p[i] * 2.0 + k3_p[i] * 2.0 + k4_p[i]) * dt_over_six;
            self[i].velocity = velocities[i]
                + (k1_v[i] + k2_v[i] * 2.0 + k3_v[i] * 2.0 + k4_v[i]) * dt_over_six;
        }

        let acc_next = compute_accelerations(
            &self.iter().map(|body| body.position).collect::<Vec<DVec2>>(),
            &masses,
            gravity
        );
        for i in 0..len {
            self[i].acceleration = acc_next[i];
        }
    }

    fn calculate_gravitational_acceleration(&self, body: &Body, gravity: f64) -> DVec2 {
        let mut aggregated_acceleration = DVec2::ZERO;
        for other in self {
            if std::ptr::eq(body, other) { continue; }
            let distance = body.position.distance(other.position);
            let force = G * gravity * other.mass / distance.powi(2);
            let acceleration = (other.position - body.position).normalize() * force;
            aggregated_acceleration += acceleration;
        }
        aggregated_acceleration
    }
}

fn compute_accelerations(positions: &[DVec2], masses: &[f64], gravity: f64) -> Vec<DVec2> {
    let len = positions.len();
    let mut acc = vec![DVec2::ZERO; len];
    for i in 0..len {
        for j in 0..len {
            if i == j { continue; }
            let delta = positions[j] - positions[i];
            let distance = delta.length();
            if distance == 0.0 {
                continue;
            }
            let force = G * gravity * masses[j] / distance.powi(2);
            acc[i] += delta / distance * force;
        }
    }
    acc
}