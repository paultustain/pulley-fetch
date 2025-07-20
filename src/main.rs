use std::f64::consts::PI;
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{self, Color, DrawParams, Rectangle, Texture};
use tetra::input::{self, MouseButton, is_mouse_button_pressed};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const WINDOW_WIDTH: f32 = 1260.0;
const WINDOW_HEIGHT: f32 = 720.0;
const FRICTION: f32 = 0.002;
const MAX_ROTATION: f32 = 1.;
const DEPTH: f32 = 20.; // Number of rotations for a score to register

fn main() -> tetra::Result {
    ContextBuilder::new("Ratio Pulling", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .show_mouse(true)
        .build()?
        .run(GameState::new)
}

struct GameState {
    score: Text,
    main_gear: Gear,
    secondary_gear: Gear,
    hover_main: HoverBox,
    hover_second: HoverBox,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let texture = Texture::new(ctx, "./resources/saw.png")?;

        let main_position = Vec2::new(
            (WINDOW_WIDTH - texture.width() as f32) / 2. - 200.,
            (WINDOW_HEIGHT - texture.height() as f32) / 2.,
        );

        let main_gear = Gear::new(texture.clone(), main_position, 0., 9.);
        let secondary_gear = Gear::new(
            texture.clone(),
            Vec2::new(main_position.x + 400., main_position.y),
            0.,
            52.,
        );

        let hover_main = HoverBox::new(Text::new(
            "This makes the ting go broom broom",
            Font::vector(ctx, "./resources/DejaVuSansMono.ttf", 16.)?,
        ));

        let hover_second = HoverBox::new(Text::new(
            "This goes broom broom from the ting",
            Font::vector(ctx, "./resources/DejaVuSansMono.ttf", 16.)?,
        ));

        Ok(GameState {
            score: Text::new(
                format!("Rotation: {}", 0.),
                Font::vector(ctx, "./resources/DejaVuSansMono.ttf", 16.)?,
            ),
            main_gear: main_gear,
            secondary_gear: secondary_gear,
            hover_main: hover_main,
            hover_second: hover_second,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        let position = input::get_mouse_position(ctx).round();

        if is_mouse_button_pressed(ctx, MouseButton::Left) {
            if self.main_gear.bounds().contains_point(position) {
                self.main_gear.rotation_speed += 0.05;
                if self.main_gear.rotation_speed > MAX_ROTATION {
                    self.main_gear.rotation_speed = MAX_ROTATION;
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);
        let mouse_pos = input::get_mouse_position(ctx).round();

        if self.main_gear.bounds().contains_point(mouse_pos) {
            self.hover_main
                .text
                .draw(ctx, Vec2::new(self.main_gear.position.x - 50., 16.));
        }

        if self.secondary_gear.bounds().contains_point(mouse_pos) {
            self.hover_second
                .text
                .draw(ctx, Vec2::new(self.secondary_gear.position.x - 50., 16.));
        }

        let ratio = self.main_gear.teeth / self.secondary_gear.teeth;

        self.main_gear.rotation += PI as f32 * self.main_gear.rotation_speed;
        self.secondary_gear.rotation += PI as f32 * (self.main_gear.rotation_speed * ratio);

        self.main_gear.rotation_speed -= self.main_gear.friction;
        if self.main_gear.rotation_speed < 0. {
            self.main_gear.rotation_speed = 0.;
        }

        let new_score = format!(
            "Score: {:.0}",
            (self.secondary_gear.rotation / (2. * std::f32::consts::PI)) / DEPTH
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

    fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x - self.texture.width() as f32 / 2.,
            self.position.y - self.texture.height() as f32 / 2.,
            self.texture.width() as f32,
            self.texture.height() as f32,
        )
    }
}

struct HoverBox {
    text: Text,
}

impl HoverBox {
    fn new(text: Text) -> HoverBox {
        HoverBox { text }
    }
}
