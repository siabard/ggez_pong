use ggez;

use ggez::event::{self, EventHandler};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use std::env;
use std::path;

use ggez::nalgebra as na;

/// constants
/// screen width & height

const SCREEN_WIDTH: f32 = 1240.0;
const SCREEN_HEIGHT: f32 = 600.0;

/// Paddle

/// Game state
struct MainState {
    message: graphics::Text,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/NanumGothic.ttf")?;
        let message = graphics::Text::new(("하하하하하!", font, 48.0));

        let s = MainState { message };
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::set_default_filter(ctx, graphics::FilterMode::Linear);
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // 화면에 출력하는 부분
        let span = *&self.message.width(ctx) as f32;
        let dest_point = na::Point2::new((SCREEN_WIDTH - span) / 2.0, SCREEN_HEIGHT / 2.0);
        graphics::draw(ctx, &self.message, (dest_point, 0.0, graphics::WHITE))?;

        let rect1 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0., 0., 10., 32.),
            graphics::WHITE,
        )?;

        graphics::draw(ctx, &rect1, graphics::DrawParam::default())?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if keycode == KeyCode::Escape || keycode == KeyCode::Q {
            ggez::event::quit(ctx);
        }
    }
}

pub fn main() -> GameResult {
    // Resource Directory
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    // Filter
    let cb = ContextBuilder::new("Pong", "ggez").add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut cb
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()?;

    let state = &mut MainState::new(ctx)?;

    event::run(ctx, event_loop, state)
}
