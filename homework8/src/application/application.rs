use glam::Vec2;
use utils::{
    graphic::{Action, Key},
    triangle::Rgb,
};

use crate::{drawer::Drawer, rope::Rope};

pub trait App {
    fn render(&mut self, drawer: &mut dyn Drawer);
    fn handle_event(&mut self, action: &Action);
}

#[derive(Clone, Copy)]
pub struct AppConfig {
    pub mass: f32,
    pub ks: f32,
    pub steps_per_frame: u32,
    pub gravity: Vec2,
}

impl AppConfig {
    pub fn default() -> Self {
        Self {
            mass: 1.0,
            ks: 100.0,
            steps_per_frame: 64,
            gravity: Vec2 { x: 0.0, y: 9.8 },
        }
    }
}

pub struct Application {
    config: AppConfig,

    rope_euler: Rope,
    rope_verlet: Rope,
}

impl Application {
    pub fn new(config: AppConfig) -> Self {
        // becouse the simple drawer use top left corner as the origin point,
        // we shifted the coordinate here so it's not same as it given by games101 official homework
        Self {
            config,
            rope_euler: Rope::new(
                Vec2 { x: 500.0, y: 300.0 },
                Vec2 {
                    x: 100.0,
                    y: 300.0,
                },
                10,
                config.mass,
                config.ks,
                vec![0],
            ),
            rope_verlet: Rope::new(
                Vec2 { x: 500.0, y: 300.0 },
                Vec2 {
                    x: 100.0,
                    y: 300.0,
                },
                3,
                config.mass,
                config.ks,
                vec![0],
            ),
        }
    }
}

impl App for Application {
    fn render(&mut self, drawer: &mut dyn Drawer) {
        for _ in 0..self.config.steps_per_frame {
            self.rope_euler
                .simulate_euler(1.0 / self.config.steps_per_frame as f32, self.config.gravity);
            self.rope_verlet
                .simulate_verlet(1.0 / self.config.steps_per_frame as f32, self.config.gravity);
        }

        struct RenderCase<'a> {
            // rope
            rope: &'a Rope,
            color: Rgb,
        }

        let render_case = [
            RenderCase {
                rope: &self.rope_euler,
                color: Rgb(0, 0, 255),
            },
            RenderCase {
                rope: &self.rope_verlet,
                color: Rgb(0, 255, 0),
            },
        ];

        for RenderCase { rope, color } in render_case {
            for posi in rope.masses_positions() {
                drawer.draw_point(posi, &color);
            }

            for (start, end) in rope.springs_position() {
                drawer.draw_line(start, end, &color);
            }
        }
    }

    fn handle_event(&mut self, action: &Action) {
        match action {
            Action::Key(Key::Minus) => {
                if self.config.steps_per_frame > 1 {
                    self.config.steps_per_frame /= 2;
                }
            }
            Action::Key(Key::Equals) => self.config.steps_per_frame += 2,
            _ => (),
        }
    }
}
