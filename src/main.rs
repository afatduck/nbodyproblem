use macroquad::prelude::*;

use crate::simulation::{Simulation};
use crate::styles::set_app_style;

pub mod body;
pub mod simulation;
pub mod styles;

#[macroquad::main(window_conf)]
async fn main() {

    let sun = body::Body {
        position: DVec2 { x: 0.0, y: 0.0 },
        velocity: DVec2 { x: 0.0, y: 0.0 },
        acceleration: DVec2::ZERO,
        mass: 1.0,
        radius: 0.00465,
        name: String::from("Sun"),
        restitution: 1.0
    };

    let mercury = body::Body {
        position: DVec2 { x: 0.387, y: 0.0 },
        velocity: DVec2 { x: 0.0, y: 10.086 },
        acceleration: DVec2::ZERO,
        mass: 1.66e-7,
        radius: 1.631e-5,
        name: String::from("Mercury"),
        restitution: 1.0
    };

    let venus = body::Body {
        position: DVec2 { x: 0.723, y: 0.0 },
        velocity: DVec2 { x: 0.0, y: 7.382 },
        acceleration: DVec2::ZERO,
        mass: 2.447e-6,
        radius: 4.045e-5,
        name: String::from("Venus"),
        restitution: 1.0
    };

    let earth = body::Body {
        position: DVec2 { x: 1.0, y: 0.0 },
        velocity: DVec2 { x: 0.0, y: 6.2832 },
        acceleration: DVec2::ZERO,
        mass: 3.0035e-6,
        radius: 4.2635e-5,
        name: String::from("Earth"),
        restitution: 1.0
    };

    let moon = body::Body {
        position: DVec2 { x: 1.00257, y: 0.0 },
        velocity: DVec2 { x: 0.0, y: 6.4988 },
        acceleration: DVec2::ZERO,
        mass: 3.694e-8,
        radius: 1.161e-5,
        name: String::from("Moon"),
        restitution: 1.0
    };

    let mars = body::Body {
        position: DVec2 { x: 1.524, y: 0.0 },
        velocity: DVec2 { x: 0.0, y: 5.087 },
        acceleration: DVec2::ZERO,
        mass: 3.227e-7,
        radius: 2.266e-5,
        name: String::from("Mars"),
        restitution: 1.0
    };

    let jupiter = body::Body {
        position: DVec2 { x: 5.203, y: 0.0 },
        velocity: DVec2 { x: 0.0, y: 2.755 },
        acceleration: DVec2::ZERO,
        mass: 9.543e-4,
        radius: 0.000477,
        name: String::from("Jupiter"),
        restitution: 1.0
    };

    let saturn = body::Body {
        position: DVec2 { x: 9.537, y: 0.0 },
        velocity: DVec2 { x: 0.0, y: 2.037 },
        acceleration: DVec2::ZERO,
        mass: 2.857e-4,
        radius: 0.000403,
        name: String::from("Saturn"),
        restitution: 1.0
    };

    let uranus = body::Body {
        position: DVec2 { x: 19.191, y: 0.0 },
        velocity: DVec2 { x: 0.0, y: 1.435 },
        acceleration: DVec2::ZERO,
        mass: 4.365e-5,
        radius: 0.000170,
        name: String::from("Uranus"),
        restitution: 1.0
    };

    let neptune = body::Body {
        position: DVec2 { x: 30.069, y: 0.0 },
        velocity: DVec2 { x: 0.0, y: 1.148 },
        acceleration: DVec2::ZERO,
        mass: 5.149e-5,
        radius: 0.000165,
        name: String::from("Neptune"),
        restitution: 1.0
    };

    let bodies = vec![sun, mercury, venus, earth, moon, mars, jupiter, saturn, uranus, neptune];

    set_app_style().await;

    let mut simulation = Simulation::new();
    for body in bodies {
        simulation.add_body(body);
    }

    loop {
        simulation.frame_update(get_frame_time());
        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        fullscreen: false,
        high_dpi: true,
        sample_count: 16,
        window_title: "N Body Problem".into(),
        window_width: 1600,
        window_height: 900,
        icon: None,
        window_resizable: false,
        platform: Default::default()
    }
}
