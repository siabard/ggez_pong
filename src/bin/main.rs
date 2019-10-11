use ggez;

use ggez::event::{self, EventHandler};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::timer;
use ggez::{graphics, Context, ContextBuilder, GameResult};

use std::env;
use std::path;

use ggez::nalgebra as na;

use ggez_pong::*;

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

    player_1: paddle::Paddle,
    player_2: paddle::Paddle,
    ball: ball::Ball,

    game_state: String,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/NanumGothic.ttf")?;
        let message = graphics::Text::new(("Hello Pong!", font, 12.0));

        let player_1 = paddle::Paddle::new(
            10.,
            30.,
            5.,
            20.,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            paddle::PaddleDirection::LEFT,
        );
        let player_2 = paddle::Paddle::new(
            SCREEN_WIDTH - 10.,
            SCREEN_HEIGHT - 30.,
            5.,
            20.,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            paddle::PaddleDirection::RIGHT,
        );
        let player_score_1: u32 = 0;
        let player_score_2: u32 = 0;
        let ball = ball::Ball::new(SCREEN_WIDTH / 2.0 - 2.0, SCREEN_HEIGHT / 2.0 - 2.0, 4., 4.);

        let game_state = "start".to_owned();
        let s = MainState {
            message,

            player_score_1,
            player_score_2,

            player_1,
            player_2,
            ball,

            game_state,
        };
        Ok(s)
    }

    fn display_fps(&self, ctx: &mut Context) {
        let font = graphics::Font::new(ctx, "/NanumGothic.ttf").unwrap();
        let fps = ggez::timer::fps(ctx);
        let fps_message = graphics::Text::new((fps.to_string(), font, 12.0));

        let dest_point = na::Point2::new(10., 10.);
        graphics::draw(
            ctx,
            &fps_message,
            (dest_point, 0.0, graphics::Color::from_rgba(0, 255, 0, 255)),
        )
        .unwrap();
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // 플레이어 Y좌표의 계산
        let dt = timer::duration_to_f64(timer::delta(ctx));

        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.player_2.set_dy(PADDLE_SPEED * -1.);
        } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.player_2.set_dy(PADDLE_SPEED);
        } else {
            self.player_2.set_dy(0.);
        }

        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.player_1.set_dy(PADDLE_SPEED * -1.);
        } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.player_1.set_dy(PADDLE_SPEED);
        } else {
            self.player_1.set_dy(0.);
        }

        if self.game_state == "play" {
            self.ball.update(dt);
            self.ball.collides(&self.player_1);
            self.ball.collides(&self.player_2);
            self.ball.bounce(SCREEN_HEIGHT);
        }

        self.player_1.update(dt);
        self.player_2.update(dt);

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

        self.player_1.render(ctx).unwrap();
        self.player_2.render(ctx).unwrap();

        self.ball.render(ctx).unwrap();

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

        self.display_fps(ctx);
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
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
                .fullscreen_type(ggez::conf::FullscreenType::True),
        )
        .window_setup(ggez::conf::WindowSetup::default().title("Pong: ggez"))
        .build()?;

    let state = &mut MainState::new(ctx)?;

    event::run(ctx, event_loop, state)
}
