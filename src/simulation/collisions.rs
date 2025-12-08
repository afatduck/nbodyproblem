use crate::simulation::Simulation;

impl Simulation {
    pub fn resolve_collisions(&mut self) {
        let len = self._bodies.len();
        for i in 0..len-1 {
            for j in i+1..len {
                let body_a = &self._bodies[i];
                let body_b = &self._bodies[j];

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
                    let impulse_magnitude = - ((1.0 + self._restitution) * normal_component) / ((1.0 / body_a.mass) + (1.0 / body_b.mass));
                    let new_velocity_a = body_a.velocity + collision_normal * impulse_magnitude / body_a.mass;
                    let new_velocity_b = body_b.velocity - collision_normal * impulse_magnitude / body_b.mass;

                    self._bodies[i].position = new_position_a;
                    self._bodies[j].position = new_position_b;
                    self._bodies[i].velocity = new_velocity_a;
                    self._bodies[j].velocity = new_velocity_b;

                }        

            }
        }
    }
}