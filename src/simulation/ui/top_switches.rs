use macroquad::{color::WHITE, math::vec2, text::draw_text, ui::root_ui};

use crate::simulation::{Simulation, trajectory::TrajectoryVisibility};

static MARGIN: f32 = 2e1;

fn format_timer(timer: f32) -> String {
    let itimer = timer as i32;
    let hours = itimer / (60 * 60);
    let minutes = itimer / 60 % 60;
    let seconds = timer % 60.;
    if hours != 0 {
        return format!("{hours}h {minutes}m {seconds:.2}s");
    }
    else if minutes != 0 {
        return format!("{minutes}m {seconds:.2}s");
    }
    return format!("{seconds:.2}s");
}
 
impl Simulation {
    fn draw_trajectory_visibility_switch(&mut self) {
        let text_tim = draw_text("Show trajectories: ", MARGIN, MARGIN, 20., WHITE);
        let pos = vec2(
            text_tim.width + MARGIN,
            5.
        );
        if root_ui().button(pos, self._trajectory_visibility.to_string()) {
            self.stop_mouse_propagation();
            match self._trajectory_visibility {
                TrajectoryVisibility::ALL => {
                    self._trajectory_visibility = TrajectoryVisibility::SELECTED
                },
                TrajectoryVisibility::SELECTED => {
                    self._trajectory_visibility = TrajectoryVisibility::NONE
                },
                TrajectoryVisibility::NONE => {
                    self._trajectory_visibility = TrajectoryVisibility::ALL
                },
            }
        }       
    }

    fn draw_show_names_switch(&mut self) {
        let offset = 260.;
        let text_tim = draw_text("Body names: ", offset, MARGIN, 20., WHITE);
        let pos = vec2(
            offset + text_tim.width + MARGIN,
            5.
        );
        let button_label = if self._show_names { String::from("Hide") } else { String::from("Show") };
        if root_ui().button(pos, button_label) {
            self.stop_mouse_propagation();
            self._show_names = !self._show_names;
        }       
    }

    fn draw_timer(&mut self) {
        let offset = 440.;
        let text_tim = draw_text(
            &format!("Running for: {}", format_timer(self._timer)),
            offset, 
            MARGIN, 
            20., 
            WHITE
        );
        let pos = vec2(
            offset + text_tim.width + MARGIN,
            5.
        );
        if root_ui().button(pos, "Reset") {
            self.stop_mouse_propagation();
            self._timer = 0.;
        }       
    }

    pub fn draw_top_switches(&mut self) {
        let skin = root_ui().default_skin();
        root_ui().pop_skin();
        self.draw_trajectory_visibility_switch();
        self.draw_show_names_switch();
        self.draw_timer();
        root_ui().push_skin(&skin);
    }
}