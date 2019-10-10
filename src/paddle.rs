use ggez::{graphics, Context, GameResult};
use rand::prelude::*;
use std::cmp;

pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub dy: f32,
    max_x: f32,
    max_y: f32,
}

impl Paddle {
    pub fn new(x: f32, y: f32, width: f32, height: f32, max_x: f32, max_y: f32) -> Paddle {
        Paddle {
            x,
            y,
            width,
            height,
            dy: 0.,
            max_x,
            max_y,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.y = self.y + (self.dy as f64 * dt) as f32;
        if self.y < 0. {
            self.y = 0.;
        } else if self.y > (self.max_y - self.height) {
            self.y = self.max_y - self.height;
        }
    }

    pub fn set_dy(&mut self, dy: f32) {
        self.dy = dy;
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
}
