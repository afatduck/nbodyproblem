use macroquad::{color::Color, math::{DVec2, Vec2, vec2}, shapes::{draw_circle, draw_triangle}, text::{draw_text, measure_text}, window::{screen_height, screen_width}};

pub static COLOR_NORMAL: u32 = 0x225588;
pub static COLOR_SELECTED: u32 = 0xaa5560;
static ARROW_DISTANCE_TO_RADIUS: f32 = 1.3;
static NAME_DISTANCE_TO_RADIUS: f32 = 1.5;
static FONT_SIZE_TO_DIAMETER: f32 = 1.;
static RADIUS_MULTIPLIER: f32 = 5e-2;
static ARROW_WIDTH_TO_RADIUS: f32 = 12.0 * RADIUS_MULTIPLIER;
static ARROW_HEIGHT_TO_RADIUS: f32 = 8.0 * RADIUS_MULTIPLIER;

#[derive(Clone)]
pub struct Body {
    pub position: DVec2,
    pub velocity: DVec2,
    pub acceleration: DVec2,
    pub mass: f64,
    pub radius: f64,
    pub name: String,
    pub restitution: f64
}

impl std::hash::Hash for Body {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.position.x.to_bits(), self.position.y.to_bits()).hash(state);
        (self.velocity.x.to_bits(), self.velocity.y.to_bits()).hash(state);
        self.mass.to_bits().hash(state);
        self.radius.to_bits().hash(state);
        self.restitution.to_bits().hash(state);
    }
}

impl Body {
    const COLOR_NORMAL: Color = Color::from_hex(COLOR_NORMAL); 
    const COLOR_SELECTED: Color = Color::from_hex(COLOR_SELECTED);

    fn to_vec2(value: DVec2) -> Vec2 {
        vec2(value.x as f32, value.y as f32)
    }

    fn draw_velocity_arrow(&self, position: DVec2, radius: f64) {
        if self.velocity == DVec2::ZERO { return; }
        let velocity_normal = self.velocity.normalize();
        let normal_perpendicular = DVec2::new(-velocity_normal.y, velocity_normal.x);
        let arrow_base = position + velocity_normal * (radius * ARROW_DISTANCE_TO_RADIUS as f64);
        let arrow_top = arrow_base + velocity_normal * (ARROW_HEIGHT_TO_RADIUS as f64 * radius);
        let arrow_side1 = arrow_base + normal_perpendicular * (ARROW_WIDTH_TO_RADIUS as f64 * 0.5 * radius);
        let arrow_side2 = arrow_base - normal_perpendicular * (ARROW_WIDTH_TO_RADIUS as f64 * 0.5 * radius);
        draw_triangle(
            Self::to_vec2(arrow_top),
            Self::to_vec2(arrow_side1),
            Self::to_vec2(arrow_side2),
            Self::COLOR_SELECTED
        );
    }

    fn draw_name(&self, color: Color, position: DVec2, radius: f64) {
        let font_size = radius as f32 * 2.0 * FONT_SIZE_TO_DIAMETER;
        let text_size = measure_text(&self.name, None, font_size as u16, 1.);
        let position = Self::to_vec2(position);
        let text_pos = position - vec2(0.5 * text_size.width, 1. * radius as f32 * NAME_DISTANCE_TO_RADIUS);
        draw_text(&self.name, text_pos.x, text_pos.y, font_size, color);
    }

    pub fn draw(&self, selected: bool, draw_name: bool, position: DVec2, radius: f64) {
        let color = if selected { Self::COLOR_SELECTED } else { Self::COLOR_NORMAL };
        let translated_pos = Self::to_vec2(position);
        draw_circle(translated_pos.x, translated_pos.y, radius as f32, color);
        if selected {
            self.draw_velocity_arrow(position, radius);
        }
        if draw_name {
            self.draw_name(color, position, radius);
        }
    }
}

impl Default for Body {
    fn default() -> Self {
        Self { 
            position: DVec2::new(screen_width() as f64 / 2.0, screen_height() as f64 / 2.0),
            velocity: Default::default(),
            acceleration: DVec2::ZERO,
            mass: 1e2, 
            radius: 5e1,
            name: String::from("untitled body"),
            restitution: 1.
        }
    }
}