use std::f64::consts::PI;
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{self, Color, DrawParams, Texture};
use tetra::input::{self, MouseButton, is_mouse_button_pressed};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const FRICTION: f32 = 0.0051;

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .show_mouse(true)
        .build()?
        .run(GameState::new)
}

struct GameState {
    score: Text,
    main_gear: Gear,
    secondary_gear: Gear,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let texture = Texture::new(ctx, "./resources/player1.png")?;
        let main_position = Vec2::new(
            (WINDOW_WIDTH - texture.width() as f32) / 2. - 200.,
            (WINDOW_HEIGHT - texture.height() as f32) / 2.,
        );

        let main_gear = Gear::new(texture.clone(), main_position, 0., 28.);
        let secondary_gear = Gear::new(
            texture.clone(),
            Vec2::new(main_position.x + 400., main_position.y),
            0.,
            9.,
        );

        Ok(GameState {
            score: Text::new(
                format!("Rotation: {}", 0.),
                Font::vector(ctx, "./resources/DejaVuSansMono.ttf", 16.)?,
            ),
            main_gear: main_gear,
            secondary_gear: secondary_gear,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if is_mouse_button_pressed(ctx, MouseButton::Left) {
            self.main_gear.rotation_speed = 0.1;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.769, 0.812, 0.631));

        let ratio = self.main_gear.teeth / self.secondary_gear.teeth;

        self.main_gear.rotation += PI as f32 * self.main_gear.rotation_speed;
        self.secondary_gear.rotation += PI as f32 * (self.main_gear.rotation_speed * ratio);

        self.main_gear.rotation_speed -= self.main_gear.friction;
        if self.main_gear.rotation_speed < 0. {
            self.main_gear.rotation_speed = 0.;
        }

        let new_score = format!(
            "Score: {}",
            self.secondary_gear.rotation as f64 / (2. * std::f64::consts::PI)
        );

        self.score.set_content(&new_score);
        self.score.draw(ctx, Vec2::new(16., 16.));
        self.main_gear.texture.draw(
            ctx,
            DrawParams::new()
                .position(self.main_gear.position)
                .origin(self.main_gear.center())
                .rotation(self.main_gear.rotation),
        );

        self.secondary_gear.texture.draw(
            ctx,
            DrawParams::new()
                .position(self.secondary_gear.position)
                .origin(self.secondary_gear.center())
                .scale(Vec2::new(1. / ratio, 1. / ratio))
                .rotation(self.secondary_gear.rotation),
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
    teeth: f32,
}

impl Gear {
    fn new(texture: Texture, position: Vec2<f32>, rotation: f32, teeth: f32) -> Gear {
        Gear {
            texture,
            position,
            rotation,
            friction: FRICTION,
            rotation_speed: 0.,
            teeth: teeth,
        }
    }

    fn center(&self) -> Vec2<f32> {
        Vec2::new(
            self.texture.width() as f32 / 2.,
            self.texture.height() as f32 / 2.,
        )
    }
}
