use glam::Vec2;
use utils::{
    graphic::{Action, Key},
    triangle::Rgb,
};

use crate::drawer::Drawer;

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

pub struct Application {
    config: AppConfig,

    ropeEuler: u8,
    ropeVerlet: u8,
}

impl Application {
    pub fn new(config: AppConfig) -> Self {
        // TODO init rope
        Self {
            config,
            ropeEuler: 0,
            ropeVerlet: 0,
        }
    }
}

impl App for Application {
    fn render(&mut self, drawer: &mut dyn Drawer) {
        for i in 0..self.config.steps_per_frame {
            // self rope simulate
        }

        struct RenderCase<'a> {
            // rope
            rope: &'a u8,
            color: Rgb,
        }

        let render_case = [
            RenderCase {
                rope: &self.ropeEuler,
                color: Rgb(0, 0, 255),
            },
            RenderCase {
                rope: &self.ropeVerlet,
                color: Rgb(0, 255, 0),
            },
            RenderCase {
                rope: &self.ropeVerlet,
                color: Rgb(0, 255, 0),
            },
        ];

        for RenderCase { rope, color } in render_case {
            // for m in rope.masses
            // draw point
            let p = Vec2::new(*rope as f32, *rope as f32);
            drawer.draw_point(p, &color);

            // for s in rope springs
            drawer.draw_line(p, -p, &color);
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
