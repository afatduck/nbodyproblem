use macroquad::{color::Color, math::Vec2, shapes::draw_circle};

#[derive(Clone)]
pub struct Body {
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f32,
    pub radius: f32,
}

impl Body {
    pub fn draw(&self, translate: &Vec2) {
        let translated_pos = self.position + *translate;
        draw_circle(translated_pos.x, translated_pos.y, self.radius, Color::from_hex(0x225588));
    }
}