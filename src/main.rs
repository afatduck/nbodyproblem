use macroquad::prelude::*;

use crate::body::{BodyVec};

pub mod body;

pub const DT: f32 = 2e-3;

#[macroquad::main(window_conf)]
async fn main() {

    let body1 = body::Body {
        position: Vec2 { x: 110., y: 110. },
        velocity: Vec2 { x: 0., y: 10. },
        // acceleration: Vec2::ZERO,
        mass: 15.0,
        radius: 20.0
    };

    let body2 = body::Body {
        position: Vec2 { x: 500., y: 300. },
        velocity: Vec2 { x: 0.0, y: 0.0 },
        // acceleration: Vec2::ZERO,
        mass: 100.0,
        radius: 50.
    };

    let body3 = body::Body {
        position: Vec2 { x: 800., y: 300. },
        velocity: Vec2 { x: 0., y: -6. },
        // acceleration: Vec2::ZERO,
        mass: 20.0,
        radius: 12.
    };

    let mut bodies = Vec::from([body1, body2, body3]);
    let mut acc = 0.0;

    bodies.leapfrog();

    loop {
        acc += get_frame_time();
        while acc > DT {
            acc -= DT;
            bodies.update();
        }

        bodies.draw();

        next_frame().await; // Wait for next frame
    }
}

fn window_conf() -> Conf {
    Conf {
        fullscreen: false,
        high_dpi: true,
        sample_count: 1,
        window_title: "N Body Problem".into(),
        window_width: 1920,
        window_height: 1080,
        icon: None,
        window_resizable: false,
        platform: Default::default()
    }
}
