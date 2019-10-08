use ggez;

use ggez::event::{self, EventHandler};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::timer;
use ggez::{graphics, Context, ContextBuilder, GameResult};

use std::env;
use std::path;

use ggez::nalgebra as na;

/// constants
/// screen width & height

const SCREEN_WIDTH: f32 = 412.0;
const SCREEN_HEIGHT: f32 = 240.0;
const PADDLE_SPEED: f32 = 200.0;

/// Paddle

/// Game state
struct MainState {
    message: graphics::Text,
    player_score_1: u32,
    player_score_2: u32,
    player_y_1: i32,
    player_y_2: i32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/NanumGothic.ttf")?;
        let message = graphics::Text::new(("Pong!", font, 12.0));

        let player_score_1: u32 = 0;
        let player_score_2: u32 = 0;
        let player_y_1: i32 = 0;
        let player_y_2: i32 = SCREEN_HEIGHT as i32 - 50;

        let s = MainState {
            message,
            player_score_1,
            player_score_2,
            player_y_1,
            player_y_2,
        };
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // 플레이어 Y좌표의 계산
        let dt = timer::duration_to_f64(timer::delta(ctx));

        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.player_y_2 = self.player_y_2 - (PADDLE_SPEED as f64 * dt) as i32;
        }

        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.player_y_2 = self.player_y_2 + (PADDLE_SPEED as f64 * dt) as i32;
        }

        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.player_y_1 = self.player_y_1 - (PADDLE_SPEED as f64 * dt) as i32;
        }

        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.player_y_1 = self.player_y_1 + (PADDLE_SPEED as f64 * dt) as i32;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::set_default_filter(ctx, graphics::FilterMode::Linear);
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // 화면에 출력하는 부분
        let span = *&self.message.width(ctx) as f32;
        let dest_point = na::Point2::new((SCREEN_WIDTH - span) / 2.0, 20.0);
        graphics::draw(ctx, &self.message, (dest_point, 0.0, graphics::WHITE))?;

        // MeshBuilder 를이용해서 여러 개를 한번에 출력
        let mb = &mut graphics::MeshBuilder::new();

        mb.rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(10., self.player_y_1 as f32, 5., 20.),
            graphics::WHITE,
        );
        mb.rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(SCREEN_WIDTH - 10., self.player_y_2 as f32, 5., 20.),
            graphics::WHITE,
        );
        mb.rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(SCREEN_WIDTH / 2. - 2., SCREEN_HEIGHT / 2. - 2., 4., 4.),
            graphics::WHITE,
        );

        let mesh = mb.build(ctx)?;

        graphics::draw(ctx, &mesh, graphics::DrawParam::new())?;

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
