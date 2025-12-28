use std::{fmt::Display, usize};

use macroquad::{color::Color, shapes::draw_circle};

use crate::{body::{COLOR_NORMAL, COLOR_SELECTED}, simulation::Simulation};

static LINE_WIDTH_RATIO: f32 = 2e-1;
static SEGMENTS: usize = 50;

#[derive(Clone)]
pub enum TrajectoryVisibility {
    NONE,
    SELECTED,
    ALL
}

impl Display for TrajectoryVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrajectoryVisibility::ALL => {write!(f, "All")},
            TrajectoryVisibility::SELECTED => {write!(f, "Selected")},
            TrajectoryVisibility::NONE => {write!(f, "None")}
        }
    }
}

impl Simulation {

    pub fn draw_trajectories(&self) {
        match self._trajectory_visibility {
            TrajectoryVisibility::ALL => {
                let selected = self._selected.unwrap_or(usize::MAX);
                for i in 0..SEGMENTS {
                    let index = i * self._update_buffer.len() / SEGMENTS;
                    let bodies = &self._update_buffer[index];
                    let fade: f32 = i as f32 / SEGMENTS as f32;
                    for (i, body) in bodies.iter().enumerate() {
                        draw_circle(
                            body.position.x, 
                            body.position.y,
                            body.radius * LINE_WIDTH_RATIO * (1. - fade),
                            Color::from_hex(
                                if selected == i { COLOR_SELECTED } else { COLOR_NORMAL }
                            )
                        );
                    }
                }
            },
            TrajectoryVisibility::SELECTED => {
                if let Some (body_index) = self._selected {
                    for i in 0..SEGMENTS {
                        let index = i * self._update_buffer.len() / SEGMENTS;
                        let body = &self._update_buffer[index][body_index];
                        let fade: f32 = i as f32 / SEGMENTS as f32;
                        draw_circle(
                            body.position.x, 
                            body.position.y,
                            body.radius * LINE_WIDTH_RATIO * (1. - fade),
                            Color::from_hex(COLOR_SELECTED)
                        );
                    }
                }
            },
            TrajectoryVisibility::NONE => {}
        }
        
    }

}