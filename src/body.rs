use macroquad::{color::Color, math::Vec2, shapes::draw_circle};

#[derive(Clone)]
pub struct Body {
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f32,
    pub radius: f32,
}

impl Body {
    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, Color::from_hex(0x225588));
    }
}