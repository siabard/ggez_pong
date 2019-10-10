use ggez::{graphics, Context, GameResult};
use rand::prelude::*;
use crate::paddle::Paddle;

pub struct Ball {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    dx: f32,
    dy: f32,
}

impl Ball {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Ball {
        let mut rng = rand::thread_rng();
        let dx: f32 = match rng.gen_range(0, 2) {
            1 => 100.,
            _ => -100.,
        };
        let dy: f32 = rng.gen_range(-50., 50.) * 1.5;

        Ball {
            x,
            y,
            width,
            height,
            dx,
            dy,
        }
    }

    pub fn reset(&mut self, screen_width: f32, screen_height: f32) {
        let mut rng = rand::thread_rng();
        self.x = (screen_width - self.width) / 2.0;
        self.y = (screen_height - self.height) / 2.0;
        let dx: f32 = match rng.gen_range(0, 2) {
            1 => 100.,
            _ => -100.,
        };
        let dy: f32 = rng.gen_range(-50., 50.) * 1.5;

        self.dx = dx;
        self.dy = dy;
    }

    pub fn update(&mut self, dt: f64) {
        self.x = self.x + (self.dx as f64 * dt) as f32;
        self.y = self.y + (self.dy as f64 * dt) as f32;
    }

    pub fn render(&self, ctx: &mut Context) -> GameResult {
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(self.x, self.y, self.width, self.height),
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &rect, graphics::DrawParam::default())?;

        Ok(())
    }

    pub fn collides(&mut self, paddle: &Paddle, max_y: f32) {
        if self.x <= paddle.x + paddle.width &&
            self.x + self.width >= paddle.x &&
            self.y <= paddle.y + paddle.height &&
            self.y + self.height >= paddle.y {
                // collision
                self.dx = -self.dx * 1.03;
                self.x = paddle.x + self.dx;

                let mut rng = rand::thread_rng();


                if self.dy < 0. {
                    self.dy = -1. * rng.gen_range(10., 150.);
                } else {
                    self.dy = rng.gen_range(10., 150.);
                }
            }

        if self.y <= 0. || self.y >= max_y {
            self.dy = self.dy * -1.;
        }
    }
}
