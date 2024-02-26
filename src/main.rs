use ggez;
use ggez::event::{self, EventHandler};
use ggez::graphics::Canvas;
use ggez::graphics::{Color, PxScale, TextFragment};
use ggez::input::keyboard::KeyCode;
use ggez::{graphics, Context, GameResult};
use mint::Point2;
use rand::Rng;

// resolution requires 4* expected pixel size, not sure why :)
const WINDOW_HEIGHT: f32 = 400.0;
const WINDOW_WIDTH: f32 = 800.0;
const PADDLE_HEIGHT: f32 = 200.0;
const PADDLE_WIDTH: f32 = 40.0;
const PADDLE_SPEED: f32 = 6.0;
const FONT: &str = "LiberationMono-Regular";
const TELEPORTER_WIDTH: f32 = 300.0;
const TELEPORTER_HEIGHT: f32 = 5.0;

#[derive(PartialEq)]
enum PlayerID {
    Player1,
    Player2,
    Comp,
}

#[derive(PartialEq)]
enum TeleporterLocation {
    Top,
    Bottom,
}

struct Player {
    /// The unique identifier for a player.
    _player_id: PlayerID,
    pos_x: f32,
    pos_y: f32,
    key_up: KeyCode,
    key_down: KeyCode,
}

impl Player {
    fn new(id: PlayerID) -> GameResult<Player> {
        let key_up;
        let key_down;
        let start_x: f32;
        let start_y: f32 = WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 4.0;
        if id == PlayerID::Player1 {
            key_up = KeyCode::W;
            key_down = KeyCode::S;
            start_x = PADDLE_WIDTH / 2.0;
        } else {
            key_up = KeyCode::Up;
            key_down = KeyCode::Down;
            start_x = WINDOW_WIDTH - PADDLE_WIDTH;
        }
        let p = Player {
            _player_id: id,
            pos_x: start_x,
            pos_y: start_y,
            key_up,
            key_down,
        };
        Ok(p)
    }
}

struct Ball {
    pos_x: f32,
    pos_y: f32,
    vel_x: f32,
    vel_y: f32,
    radius: f32,
}

impl Ball {
    fn new() -> GameResult<Ball> {
        let b = Ball {
            pos_x: WINDOW_WIDTH / 2.0,
            pos_y: WINDOW_HEIGHT / 2.0,
            vel_x: 12.0,
            vel_y: 1.0,
            radius: 10.0,
        };
        Ok(b)
    }
}

struct Teleporter {
    _location: TeleporterLocation,
    pos_x: f32,
    pos_y: f32,
    width: f32,
    height: f32,
}

impl Teleporter {
    fn new(locaction: TeleporterLocation) -> GameResult<Teleporter> {
        let _pos_y;
        if locaction == TeleporterLocation::Top {
            _pos_y = 0.0
        } else {
            _pos_y = WINDOW_HEIGHT - TELEPORTER_HEIGHT
        }
        let t = Teleporter {
            _location: locaction,
            pos_x: WINDOW_WIDTH / 2.0 - TELEPORTER_WIDTH / 4.0,
            pos_y: _pos_y,
            width: TELEPORTER_WIDTH,
            height: TELEPORTER_HEIGHT,
        };
        Ok(t)
    }
}

struct MainState {
    player1: Player,
    player2: Player,
    ball: Ball,
    player1_score: i32,
    player2_score: i32,
    teleporter_top: Teleporter,
    teleporter_bottom: Teleporter,
}

impl MainState {
    fn new(
        player1: Player,
        player2: Player,
        ball: Ball,
        teleporter_top: Teleporter,
        teleporter_bottom: Teleporter,
    ) -> GameResult<MainState> {
        let s = MainState {
            player1: player1,
            player2: player2,
            ball: ball,
            player1_score: 0,
            player2_score: 0,
            teleporter_top: teleporter_top,
            teleporter_bottom: teleporter_bottom,
        };
        Ok(s)
    }

    fn check_paddle_collisions(&self, player_id: PlayerID) -> bool {
        if player_id == PlayerID::Player1 {
            if self.ball.pos_x >= self.player1.pos_x
                && self.ball.pos_x <= self.player1.pos_x + PADDLE_WIDTH / 2.0
            {
                if self.ball.pos_y >= self.player1.pos_y
                    && self.ball.pos_y <= self.player1.pos_y + PADDLE_HEIGHT / 2.0
                {
                    return true;
                }
            }
        }
        if player_id != PlayerID::Player1 {
            if self.ball.pos_x >= self.player2.pos_x
                && self.ball.pos_x <= self.player2.pos_x + PADDLE_WIDTH
            {
                if self.ball.pos_y >= self.player2.pos_y
                    && self.ball.pos_y <= self.player2.pos_y + PADDLE_HEIGHT / 2.0
                {
                    return true;
                }
            }
        }
        return false;
    }

    fn check_teleport_collisions(&self, location: TeleporterLocation) -> bool {
        if location == TeleporterLocation::Top {
            if self.ball.pos_y <= TELEPORTER_HEIGHT
                && self.ball.pos_x >= self.teleporter_top.pos_x
                && self.ball.pos_x <= self.teleporter_top.pos_x + TELEPORTER_WIDTH
            {
                return true;
            }
        }
        if location == TeleporterLocation::Bottom {
            if self.ball.pos_y >= WINDOW_HEIGHT - TELEPORTER_HEIGHT
                && self.ball.pos_x >= self.teleporter_bottom.pos_x
                && self.ball.pos_x <= self.teleporter_bottom.pos_x + TELEPORTER_WIDTH
            {
                return true;
            }
        }
        return false;
    }

    fn reset_ball(&mut self) {
        self.ball.vel_x *= -1.0;
        self.ball.pos_y = rand::thread_rng().gen_range(0..=WINDOW_HEIGHT as i32) as f32;

        if self.ball.vel_x > 0.0 {
            self.ball.pos_x = PADDLE_WIDTH * 2.0;
        } else {
            self.ball.pos_x = WINDOW_WIDTH - PADDLE_WIDTH * 2.0;
        }
    }
}

// event::EventHandler is a trait which defines what functionality the type (MainState) must provide (here it is update and draw)
impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Movement

        if ctx.keyboard.is_key_pressed(self.player1.key_down)
            && self.player1.pos_y <= WINDOW_HEIGHT - PADDLE_HEIGHT / 2.0
        {
            self.player1.pos_y += PADDLE_SPEED
        } else if ctx.keyboard.is_key_pressed(self.player1.key_up) && self.player1.pos_y >= 0.0 {
            self.player1.pos_y -= PADDLE_SPEED
        }

        let noise: f32 = rand::thread_rng().gen_range(0.0..=2.0);
        // println!("{:2}", noise);
        if self.player2._player_id == PlayerID::Player2 {
            let noise: f32 = rand::thread_rng().gen_range(-1.0..=1.0);
            // Perfect tracking
            let y_offset = (self.ball.pos_y - (self.player2.pos_y + PADDLE_HEIGHT / 4.0));

            if y_offset > 0.0 && self.player2.pos_y <= WINDOW_HEIGHT - PADDLE_HEIGHT / 2.0 {
                self.player2.pos_y += y_offset.min(PADDLE_SPEED);
            } else if y_offset < 0.0 && self.player2.pos_y >= 0.0 {
                self.player2.pos_y += y_offset.max(-PADDLE_SPEED);
            }
        }

        if ctx.keyboard.is_key_pressed(self.player2.key_down)
            && self.player2.pos_y <= WINDOW_HEIGHT - PADDLE_HEIGHT / 2.0
        {
            self.player2.pos_y += PADDLE_SPEED
        } else if ctx.keyboard.is_key_pressed(self.player2.key_up) && self.player2.pos_y >= 0.0 {
            self.player2.pos_y -= PADDLE_SPEED
        }

        if self.check_teleport_collisions(TeleporterLocation::Top) {
            self.ball.pos_y = self.teleporter_bottom.pos_y - 1.0;
        }
        if self.check_teleport_collisions(TeleporterLocation::Bottom) {
            self.ball.pos_y = self.teleporter_top.pos_y + 1.0;
        }

        // Window Ball Collision Checks
        if self.ball.pos_y <= 0.0 {
            self.ball.vel_y *= -1.0;
        }
        if self.ball.pos_y >= WINDOW_HEIGHT {
            self.ball.vel_y *= -1.0;
        }

        if self.ball.pos_x >= WINDOW_WIDTH {
            self.player1_score += 1;
            self.reset_ball();
        }

        if self.ball.pos_x <= 0.0 {
            self.player2_score += 1;
            self.reset_ball();
        }

        // Paddle Collision Checks
        if self.check_paddle_collisions(PlayerID::Player1) {
            if self.ball.vel_x < 0.0 {
                self.ball.vel_x *= -1.0;
                let offset: f32 = self.ball.pos_y - (self.player1.pos_y + PADDLE_HEIGHT / 4.0);
                let absolute_speed: f32 = self.ball.vel_x.abs() + self.ball.vel_y.abs();
                self.ball.vel_y = absolute_speed * offset / 100.0;
                if self.ball.vel_y > 5.0 {
                    self.ball.vel_y = 5.0;
                }
                if self.ball.vel_y < -5.0 {
                    self.ball.vel_y = -5.0;
                }
            }
        }
        if self.check_paddle_collisions(PlayerID::Player2) {
            if self.ball.vel_x > 0.0 {
                self.ball.vel_x *= -1.0;
                let offset: f32 = self.ball.pos_y - (self.player2.pos_y + PADDLE_HEIGHT / 4.0);
                let absolute_speed: f32 = self.ball.vel_x.abs() + self.ball.vel_y.abs();
                self.ball.vel_y = absolute_speed * offset / 100.0;
                if self.ball.vel_y > 5.0 {
                    self.ball.vel_y = 5.0;
                }
                if self.ball.vel_y < -5.0 {
                    self.ball.vel_y = -5.0;
                }
            }
        }
        self.ball.pos_x += self.ball.vel_x;
        self.ball.pos_y += self.ball.vel_y;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        let paddle1 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                self.player1.pos_x,
                self.player1.pos_y,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),
            Color::WHITE,
        )?;

        let paddle2 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                self.player2.pos_x,
                self.player2.pos_y,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),
            Color::WHITE,
        )?;

        let teleport_top = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                self.teleporter_top.pos_x,
                self.teleporter_top.pos_y,
                self.teleporter_top.width,
                self.teleporter_top.height,
            ),
            Color::RED,
        )?;

        let teleport_bottom = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                self.teleporter_bottom.pos_x,
                self.teleporter_bottom.pos_y,
                self.teleporter_bottom.width,
                self.teleporter_bottom.height,
            ),
            Color::RED,
        )?;

        let ball = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Point2 {
                x: self.ball.pos_x,
                y: self.ball.pos_y,
            },
            self.ball.radius,
            2.0,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )?;

        let (mid_top, mid_bot) = (
            Point2 {
                x: WINDOW_WIDTH / 2.0,
                y: 0.0 - WINDOW_HEIGHT / 2.0,
            },
            Point2 {
                x: WINDOW_WIDTH / 2.0,
                y: WINDOW_HEIGHT * 2.0,
            },
        );
        let mid_line = graphics::Mesh::new_line(
            ctx,
            &[mid_top, mid_bot],
            3.0,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )?;
        let mut player1_score_text = graphics::Text::new(TextFragment {
            text: self.player1_score.to_string(),
            color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
            scale: Some(PxScale { x: 100.0, y: 100.0 }),
            font: Some(FONT.into()),
        });
        let mut player2_score_text = graphics::Text::new(TextFragment {
            text: self.player2_score.to_string(),
            color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
            scale: Some(PxScale { x: 100.0, y: 100.0 }),
            font: Some(FONT.into()),
        });
        if self.player1_score > 7 {
            self.ball.vel_x = 0.0;
            self.ball.vel_y = 0.0;
            player1_score_text = graphics::Text::new(TextFragment {
                text: "W".to_string(),
                color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
                font: Some(FONT.into()),
                scale: Some(PxScale { x: 100.0, y: 100.0 }),
            });
            player2_score_text = graphics::Text::new(TextFragment {
                text: "L".to_string(),
                color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
                font: Some(FONT.into()),
                scale: Some(PxScale { x: 100.0, y: 100.0 }),
            });
        }
        if self.player2_score > 7 {
            self.ball.vel_x = 0.0;
            self.ball.vel_y = 0.0;
            player1_score_text = graphics::Text::new(TextFragment {
                text: "L".to_string(),
                color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
                font: Some(FONT.into()),
                scale: Some(PxScale { x: 100.0, y: 100.0 }),
            });
            player2_score_text = graphics::Text::new(TextFragment {
                text: "W".to_string(),
                color: Some(Color::new(1.0, 1.0, 1.0, 1.0)),
                font: Some(FONT.into()),
                scale: Some(PxScale { x: 100.0, y: 100.0 }),
            });
        };
        canvas.draw(
            &paddle1,
            Point2 {
                x: self.player1.pos_x,
                y: self.player1.pos_y,
            },
        );
        canvas.draw(
            &paddle2,
            Point2 {
                x: self.player2.pos_x,
                y: self.player2.pos_y,
            },
        );
        canvas.draw(
            &ball,
            Point2 {
                x: self.ball.pos_x,
                y: self.ball.pos_y,
            },
        );
        canvas.draw(
            &mid_line,
            Point2 {
                x: WINDOW_WIDTH / 2.0,
                y: WINDOW_HEIGHT / 2.0,
            },
        );
        canvas.draw(
            &player1_score_text,
            Point2 {
                x: WINDOW_WIDTH - 150.0,
                y: 50.0,
            },
        );
        canvas.draw(
            &player2_score_text,
            Point2 {
                x: WINDOW_WIDTH + 100.0,
                y: 50.0,
            },
        );
        canvas.draw(
            &teleport_top,
            Point2 {
                x: self.teleporter_top.pos_x,
                y: self.teleporter_top.pos_y,
            },
        );
        canvas.draw(
            &teleport_bottom,
            Point2 {
                x: self.teleporter_bottom.pos_x,
                y: self.teleporter_bottom.pos_y,
            },
        );

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("rong", "young_guns")
        .window_setup(ggez::conf::WindowSetup::default().title("rong!"))
        .window_mode(
            ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH * 2.0, WINDOW_HEIGHT * 2.0),
        );
    let (ctx, event_loop) = cb.build()?;

    let p1 = Player::new(PlayerID::Player1)?;
    let p2 = Player::new(PlayerID::Player2)?;
    let ball = Ball::new()?;
    let tt = Teleporter::new(TeleporterLocation::Top)?;
    let tb = Teleporter::new(TeleporterLocation::Bottom)?;
    let state = MainState::new(p1, p2, ball, tt, tb)?;
    event::run(ctx, event_loop, state)
}
