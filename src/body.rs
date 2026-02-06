use macroquad::{color::Color, math::{Vec2, vec2}, shapes::{draw_circle, draw_triangle}, text::{draw_text, measure_text}, window::{screen_height, screen_width}};

pub static COLOR_NORMAL: u32 = 0x225588;
pub static COLOR_SELECTED: u32 = 0xaa5560;
static ARROW_DISTANCE_TO_RADIUS: f32 = 1.3;
static NAME_DISTANCE_TO_RADIUS: f32 = 1.5;
static FONT_SIZE_TO_RADIUS: f32 = 0.5;
static MIN_FONT_SIZE: f32 = 14.;
static RADIUS_MULTIPLIER: f32 = 5e-2;
static ARROW_WIDTH_TO_RADIUS: f32 = 12.0 * RADIUS_MULTIPLIER;
static ARROW_HEIGHT_TO_RADIUS: f32 = 8.0 * RADIUS_MULTIPLIER;

#[derive(Clone)]
pub struct Body {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub name: String,
}

impl std::hash::Hash for Body {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.position.x.to_bits(), self.position.y.to_bits()).hash(state);
        (self.velocity.x.to_bits(), self.velocity.y.to_bits()).hash(state);
        self.mass.to_bits().hash(state);
        self.radius.to_bits().hash(state);
    }
}

impl Body {
    const COLOR_NORMAL: Color = Color::from_hex(COLOR_NORMAL); 
    const COLOR_SELECTED: Color = Color::from_hex(COLOR_SELECTED);

    fn draw_velocity_arrow(&self) {
        if self.velocity == Vec2::ZERO { return; }
        let velocity_normal = self.velocity.normalize();
        let normal_perpendicular = velocity_normal.perp();
        let arrow_base = self.position + velocity_normal * self.radius * ARROW_DISTANCE_TO_RADIUS;
        let arrow_top = arrow_base + velocity_normal * ARROW_HEIGHT_TO_RADIUS * self.radius;  
        let arrow_side1 = arrow_base + normal_perpendicular * ARROW_WIDTH_TO_RADIUS * 0.5 * self.radius;
        let arrow_side2 = arrow_base - normal_perpendicular * ARROW_WIDTH_TO_RADIUS * 0.5 * self.radius;
        draw_triangle(arrow_top, arrow_side1, arrow_side2, Self::COLOR_SELECTED);
    }

    fn draw_name(&self, color: Color) {
        let font_size = MIN_FONT_SIZE.max(self.radius * FONT_SIZE_TO_RADIUS);
        let text_size = measure_text(&self.name, None, font_size as u16, 1.);
        let text_pos = self.position - vec2(0.5 * text_size.width, 1. * self.radius * NAME_DISTANCE_TO_RADIUS);
        draw_text(&self.name, text_pos.x, text_pos.y, font_size, color);
    }

    pub fn draw(&self, selected: bool, draw_name: bool) {
        let color = if selected { Self::COLOR_SELECTED } else { Self::COLOR_NORMAL };
        let translated_pos = self.position;
        draw_circle(translated_pos.x, translated_pos.y, self.radius, color);
        if selected {
            self.draw_velocity_arrow();
        }
        if draw_name {
            self.draw_name(color);
        }
    }
}

impl Default for Body {
    fn default() -> Self {
        Self { 
            position: vec2(screen_width() / 2.0, screen_height() / 2.0), 
            velocity: Default::default(),
            acceleration: Vec2::ZERO,
            mass: 1e2, 
            radius: 5e1,
            name: String::from("untitled body")
        }
    }
}