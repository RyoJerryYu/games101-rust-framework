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
            gravity: Vec2 { x: 0.0, y: -1.0 },
        }
    }
}

pub struct Application<'a> {
    config: AppConfig,

    rope_euler: Rope<'a>,
    rope_verlet: Rope<'a>,
}

impl<'a> Application<'a> {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            rope_euler: Rope::new(
                Vec2 { x: 0.0, y: 200.0 },
                Vec2 {
                    x: -400.0,
                    y: 200.0,
                },
                3,
                config.mass,
                config.ks,
                vec![0],
            ),
            rope_verlet: Rope::new(
                Vec2 { x: 0.0, y: 200.0 },
                Vec2 {
                    x: -400.0,
                    y: 200.0,
                },
                3,
                config.mass,
                config.ks,
                vec![0],
            ),
        }
    }
}

impl<'a> App for Application<'a> {
    fn render(&mut self, drawer: &mut dyn Drawer) {
        for _ in 0..self.config.steps_per_frame {
            self.rope_euler
                .simulate_euler(1 / self.config.steps_per_frame, self.config.gravity);
            self.rope_verlet
                .simulate_verlet(1 / self.config.steps_per_frame, self.config.gravity);
        }

        struct RenderCase<'a> {
            // rope
            rope: &'a Rope<'a>,
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
            for m in &rope.masses {
                drawer.draw_point(m.position, &color);
            }

            for s in &rope.springs {
                drawer.draw_line(s.m1.position, s.m2.position, &color);
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
