use std::f64::consts::PI;
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{self, Color, DrawParams, Texture};
use tetra::input::{self, MouseButton, is_mouse_button_pressed};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const FRICTION: f32 = 0.005;

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .show_mouse(true)
        .build()?
        .run(GameState::new)
}

struct GameState {
    position: Vec2<f32>,
    score: Text,
    gear: Gear,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let texture = Texture::new(ctx, "./resources/player1.png")?;
        let position = Vec2::new(
            (WINDOW_WIDTH - texture.width() as f32) / 2.,
            (WINDOW_HEIGHT - texture.height() as f32) / 2.,
        );

        let gear = Gear::new(texture, position, 0.);

        let clicks = 0;
        Ok(GameState {
            position: position,
            score: Text::new(
                format!("Rotation: {}", 0.),
                Font::vector(ctx, "./resources/DejaVuSansMono.ttf", 16.)?,
            ),
            gear: gear,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.position = input::get_mouse_position(ctx).round();

        if is_mouse_button_pressed(ctx, MouseButton::Left) {
            self.gear.rotation_speed = 0.1;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.769, 0.812, 0.631));

        self.gear.rotation += PI as f32 * self.gear.rotation_speed;
        self.gear.rotation_speed -= self.gear.friction;
        if self.gear.rotation_speed < 0. {
            self.gear.rotation_speed = 0.;
        }

        let new_score = format!(
            "Score: {}",
            self.gear.rotation as f64 / (2. * std::f64::consts::PI)
        );

        self.score.set_content(&new_score);
        self.score.draw(ctx, Vec2::new(16., 16.));
        self.gear.texture.draw(
            ctx,
            DrawParams::new()
                .position(self.gear.position)
                .origin(self.gear.center())
                .rotation(self.gear.rotation),
        );

        Ok(())
    }
}

struct Gear {
    texture: Texture,
    position: Vec2<f32>,
    rotation: f32,
    friction: f32,
    rotation_speed: f32,
}

impl Gear {
    fn new(texture: Texture, position: Vec2<f32>, rotation: f32) -> Gear {
        Gear {
            texture,
            position,
            rotation,
            friction: FRICTION,
            rotation_speed: 0.,
        }
    }

    fn center(&self) -> Vec2<f32> {
        Vec2::new(
            self.texture.width() as f32 / 2.,
            self.texture.height() as f32 / 2.,
        )
    }
}
