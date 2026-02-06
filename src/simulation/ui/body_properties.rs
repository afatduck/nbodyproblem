
use macroquad::{math::vec2, ui::{hash, root_ui}, window::{screen_height}};

use crate::simulation::Simulation;

static MARGIN: f32 = 20.0;
static WIDTH: f32 = 300.0;
static HEIGHT: f32 = 305.0;

impl Simulation {
    pub fn draw_body_properties(&mut self) {
        if let Some(body_index) = self._selected {
            let position = vec2(MARGIN, screen_height() - HEIGHT - MARGIN);
            let size = vec2(WIDTH, HEIGHT);
            self.register_capture_window(&position, &size);
            root_ui().window(hash!(), position, size, |ui| {
                let body = &mut self._bodies[body_index];
                ui.separator();
                ui.editbox(hash!(), vec2(WIDTH, 30.0), &mut body.name);
                ui.separator();

                ui.label(None, "Position");

                ui.separator();

                let mut position_x = body.position.x.to_string();
                let mut position_y = body.position.y.to_string();

                ui.label(None, "X: ");
                ui.same_line(20.0);
                ui.editbox(hash!(), vec2(115.0, 30.0), &mut position_x);
                ui.same_line(WIDTH / 2.0);
                ui.label(None, "Y: ");
                ui.same_line(WIDTH / 2.0 + 20.0);
                ui.editbox(hash!(), vec2(115.0, 30.0), &mut position_y);                
                
                ui.separator();

                ui.label(None, "Velocity");
                ui.separator();

                let mut velocity_x = body.velocity.x.to_string();
                let mut velocity_y = body.velocity.y.to_string();

                ui.label(None, "X: ");
                ui.same_line(20.0);
                ui.editbox(hash!(), vec2(115.0, 30.0), &mut velocity_x);
                ui.same_line(WIDTH / 2.0);
                ui.label(None, "Y: ");
                ui.same_line(WIDTH / 2.0 + 20.0);
                ui.editbox(hash!(), vec2(115.0, 30.0), &mut velocity_y);
                
                ui.separator();

                ui.label(None, "Mass and raius");
                ui.separator();

                let mut mass = body.mass.to_string();
                let mut radius = body.radius.to_string();

                ui.label(None, "M: ");
                ui.same_line(20.0);
                ui.editbox(hash!(), vec2(115.0, 30.0), &mut mass);
                ui.same_line(WIDTH / 2.0);
                ui.label(None, "R: ");
                ui.same_line(WIDTH / 2.0 + 20.0);
                ui.editbox(hash!(), vec2(115.0, 30.0), &mut radius);

                ui.separator();
                ui.label(None, "Restitution");
                ui.slider(
                    hash!(), 
                    "Restitution", 
                    0.0..1.0, 
                    &mut body.restitution
                );
                ui.separator();

                ui.separator();
                ui.separator();
                ui.separator();

                let mut locked = self._selected == self._camera_lock;
                let prev_locked = locked;
                ui.same_line(-105.0);
                ui.checkbox(body_index.try_into().unwrap(), "Lock camera", &mut locked);
                if locked != prev_locked {
                    self._camera_lock = if locked {
                        self._camera.target = body.position;
                        Some(body_index)
                    } else { None };
                }

                if !self._running {
                    let skin = ui.default_skin();
                    ui.pop_skin();
                    ui.same_line(240.0);
                    if ui.button(vec2(0.0, 0.0), "Remove") {
                        self.remove_body(body_index);
                        ui.push_skin(&skin);
                        return;
                    }
                    ui.push_skin(&skin);
                }

                if self._running { return; }

                if let Ok(px) = position_x.parse::<f32>() {
                    body.position.x = px;
                }

                if let Ok(py) = position_y.parse::<f32>() {
                    body.position.y = py;
                }

                if let Ok(vx) = velocity_x.parse::<f32>() {
                    body.velocity.x = vx;
                }

                if let Ok(vy) = velocity_y.parse::<f32>() {
                    body.velocity.y = vy;
                }

                if let Ok(m) = mass.parse::<f32>() {
                    body.mass = m;
                }

                if let Ok(r) = radius.parse::<f32>() {
                    body.radius = r;
                }
                

            });
        }
    }
}