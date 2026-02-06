use macroquad::prelude::*;

use crate::simulation::{Simulation};
use crate::styles::set_app_style;

pub mod body;
pub mod simulation;
pub mod styles;

#[macroquad::main(window_conf)]
async fn main() {

    let body1 = body::Body {
        position: Vec2 { x: 250., y: 350. },
        velocity: Vec2 { x: 0., y: 25. },
        acceleration: Vec2::ZERO,
        mass: 20.0,
        radius: 20.0,
        name: String::from("Earth"),
        restitution: 1.
    };

    let body2 = body::Body {
        position: Vec2 { x: 600., y: 350. },
        velocity: Vec2 { x: 0.0, y: 0.0 },
        acceleration: Vec2::ZERO,
        mass: 2000.0,
        radius: 50.,
        name: String::from("Sun"),
        restitution: 1.
    };

    let body3 = body::Body {
        position: Vec2 { x: 600., y: 450. },
        velocity: Vec2 { x: 50., y: 0. },
        acceleration: Vec2::ZERO,
        mass: 5.0,
        radius: 8.0,
        name: String::from("Mercury"),
        restitution: 1.
    };

    set_app_style().await;

    let mut simulation = Simulation::new();
    simulation.add_body(body1);
    simulation.add_body(body2);
    simulation.add_body(body3);

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
