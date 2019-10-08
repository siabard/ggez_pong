use ggez;

use ggez::event::{self, EventHandler};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::timer;
use ggez::{graphics, Context, ContextBuilder, GameResult};

use std::cmp;
use std::env;
use std::path;

use ggez::nalgebra as na;

use rand::prelude::*;

/// constants
/// screen width & height

const SCREEN_WIDTH: f32 = 412.0;
const SCREEN_HEIGHT: f32 = 240.0;
/// Paddle
const PADDLE_SPEED: f32 = 200.0;

/// Game state
struct MainState {
    message: graphics::Text,
    player_score_1: u32,
    player_score_2: u32,
    player_y_1: i32,
    player_y_2: i32,
    ball_x: f32,
    ball_y: f32,
    ball_dx: f32,
    ball_dy: f32,
    game_state: String,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/NanumGothic.ttf")?;
        let message = graphics::Text::new(("Hello Pong!", font, 12.0));

        let player_score_1: u32 = 0;
        let player_score_2: u32 = 0;
        let player_y_1: i32 = 30;
        let player_y_2: i32 = SCREEN_HEIGHT as i32 - 50;

        let mut rng = rand::thread_rng();

        let ball_x = SCREEN_WIDTH / 2.0 - 2.0;
        let ball_y = SCREEN_HEIGHT / 2.0 - 2.0;
        let ball_dx = match rng.gen_range(0, 2) {
            1 => 100.,
            _ => -100.,
        };
        let ball_dy = rng.gen_range(-50., 50.) * 1.5;
        let game_state = "start".to_owned();
        let s = MainState {
            message,
            player_score_1,
            player_score_2,
            player_y_1,
            player_y_2,
            ball_x,
            ball_y,
            ball_dx,
            ball_dy,
            game_state,
        };
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // 플레이어 Y좌표의 계산
        let dt = timer::duration_to_f64(timer::delta(ctx));

        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.player_y_2 = cmp::max(0, self.player_y_2 - (PADDLE_SPEED as f64 * dt) as i32);
        }

        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.player_y_2 = cmp::min(
                (SCREEN_HEIGHT - 20.) as i32,
                self.player_y_2 + (PADDLE_SPEED as f64 * dt) as i32,
            );
        }

        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.player_y_1 = cmp::max(0, self.player_y_1 - (PADDLE_SPEED as f64 * dt) as i32);
        }

        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.player_y_1 = cmp::min(
                (SCREEN_HEIGHT - 20.) as i32,
                self.player_y_1 + (PADDLE_SPEED as f64 * dt) as i32,
            );
        }

        if self.game_state == "play" {
            self.ball_x = self.ball_x + (self.ball_dx as f64 * dt) as f32;
            self.ball_y = self.ball_y + (self.ball_dy as f64 * dt) as f32;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::set_default_filter(ctx, graphics::FilterMode::Linear);
        graphics::clear(ctx, [40. / 255., 45. / 255., 52. / 255., 1.0].into());

        // 화면에 출력하는 부분
        let font = graphics::Font::new(ctx, "/NanumGothic.ttf")?;
        self.message = graphics::Text::new((
            "Hello ".to_owned() + &self.game_state + " state",
            font,
            12.0,
        ));
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
            graphics::Rect::new(self.ball_x, self.ball_y, 4., 4.),
            graphics::WHITE,
        );

        let mesh = mb.build(ctx)?;

        graphics::draw(ctx, &mesh, graphics::DrawParam::new())?;

        // 점수를 출력하는 부분

        if self.game_state == "start" {
            let score_1 = graphics::Text::new((self.player_score_1.to_string(), font, 40.0));
            let score_2 = graphics::Text::new((self.player_score_2.to_string(), font, 40.0));

            graphics::draw(
                ctx,
                &score_1,
                (
                    na::Point2::new(SCREEN_WIDTH / 2.0 - 50.0, SCREEN_HEIGHT / 3.0),
                    0.0,
                    graphics::WHITE,
                ),
            )?;
            graphics::draw(
                ctx,
                &score_2,
                (
                    na::Point2::new(SCREEN_WIDTH / 2.0 + 30.0, SCREEN_HEIGHT / 3.0),
                    0.0,
                    graphics::WHITE,
                ),
            )?;
        }

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

        if keycode == KeyCode::Return {
            if self.game_state == "start" {
                self.game_state = "play".to_owned();
            } else {
                self.game_state = "start".to_owned();
            }
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
