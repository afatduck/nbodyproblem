use macroquad::{color::Color, math::{Vec2, vec2}, shapes::{draw_circle, draw_triangle}, window::{screen_height, screen_width}};

static COLOR_NORMAL: u32 = 0x225588;
static COLOR_SELECTED: u32 = 0xaa5560;
static ARROW_DISTANCE_TO_RADIUS: f32 = 1.3;
static RADIUS_MULTIPLIER: f32 = 5e-2;
static ARROW_WIDTH_TO_RADIUS: f32 = 12.0 * RADIUS_MULTIPLIER;
static ARROW_HEIGHT_TO_RADIUS: f32 = 8.0 * RADIUS_MULTIPLIER;

#[derive(Clone)]
pub struct Body {
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f32,
    pub radius: f32,
}

impl Body {
    const COLOR_NORMAL: Color = Color::from_hex(COLOR_NORMAL); 
    const COLOR_SELECTED: Color = Color::from_hex(COLOR_SELECTED);

    fn draw_velocity_arrow(&self, translate: &Vec2) {
        if self.velocity == Vec2::ZERO { return; }
        let velocity_normal = self.velocity.normalize();
        let normal_perpendicular = velocity_normal.perp();
        let arrow_base = self.position + velocity_normal * self.radius * ARROW_DISTANCE_TO_RADIUS + *translate;
        let arrow_top = arrow_base + velocity_normal * ARROW_HEIGHT_TO_RADIUS * self.radius;  
        let arrow_side1 = arrow_base + normal_perpendicular * ARROW_WIDTH_TO_RADIUS * 0.5 * self.radius;
        let arrow_side2 = arrow_base - normal_perpendicular * ARROW_WIDTH_TO_RADIUS * 0.5 * self.radius;
        draw_triangle(arrow_top, arrow_side1, arrow_side2, Self::COLOR_SELECTED);
    }

    pub fn draw(&self, translate: &Vec2, selected: bool) {
        let color = if selected { Self::COLOR_SELECTED } else { Self::COLOR_NORMAL };
        let translated_pos = self.position + *translate;
        draw_circle(translated_pos.x, translated_pos.y, self.radius, color);
        if selected {
            self.draw_velocity_arrow(translate);
        }
    }
}

impl Default for Body {
    fn default() -> Self {
        Self { 
            position: vec2(screen_width() / 2.0, screen_height() / 2.0), 
            velocity: Default::default(), 
            mass: 1e2, 
            radius: 5e1
        }
    }
}