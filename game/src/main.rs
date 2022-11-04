use std::time::Duration;

use bronze::{
    cursor::Cursor,
    engine::Engine,
    entity::{Entity, EntityPosition},
    game::Game,
    graphics::{Canvas, Sprite},
    icon::Icon,
    input::{InputManager, Key},
    resources::{Image, ResourcePool},
    scene::Scene,
    sfml::{graphics::Color, system::Vector2},
    window::{FPSConfig, Window, WindowConfig},
};

use bronze_macros::{position, size, Entity};

use debugger::Debugger;
use resources::*;

mod debugger;
mod resources;

const WINDOW_WIDTH: u32 = 960;
const WINDOW_HEIGHT: u32 = 540;

#[position]
#[size]
#[derive(Entity)]
struct Player<'r> {
    sprite: Sprite<'r>,
    velocity: f32,
}

impl<'r> Player<'r> {
    const SPEED: f32 = 640.0;

    pub fn new(image: &'r Image) -> Self {
        let Vector2 { x, y } = image.size();

        Player {
            sprite: Sprite::new(image),
            velocity: 0.0,
            position: (
                (WINDOW_WIDTH - x) as f32 / 2.0,
                (WINDOW_HEIGHT - y - 32) as f32,
            )
                .into(),
            size: (x as f32, y as f32).into(),
        }
    }
}

impl<'r> Entity<()> for Player<'r> {
    fn input(&mut self, input: &InputManager) {
        self.velocity = 0.0;
        if input.key_down(Key::Left) {
            self.velocity -= Self::SPEED;
        }
        if input.key_down(Key::Right) {
            self.velocity += Self::SPEED;
        }
    }

    fn update(&mut self, _: &mut (), frame_time: Duration) {
        let Vector2 { x: width, y: _ } = self.size;
        let velocity = self.velocity * frame_time.as_secs_f32();

        self.move_by(velocity, 0.0);

        let Vector2 { x, y: _ } = self.position;
        if x < 0.0 {
            self.move_by(-x, 0.0);
        } else if x + width as f32 > WINDOW_WIDTH as f32 {
            self.move_by(WINDOW_WIDTH as f32 - x - width as f32, 0.0);
        }

        self.sprite.set_position(self.position);
    }

    fn draw(&self, _ctx: &(), target: &mut dyn Canvas) {
        self.sprite.draw(target);
    }
}

#[position]
#[size]
#[derive(Entity)]

struct Ball<'r> {
    sprite: Sprite<'r>,
    velocity: Vector2<f32>,
    moving: bool,
}

impl<'r> Ball<'r> {
    const SPEED: f32 = 480.0;

    pub fn new(image: &'r Image, player: &Player) -> Self {
        let Vector2 { x, y } = image.size();

        let position =
            player.position + Vector2::new((player.size.x - x as f32) / 2.0, -(y as f32));

        Ball {
            sprite: Sprite::new(image),
            velocity: Vector2::new(0.0, 0.0),
            position,
            size: (x as f32, y as f32).into(),
            moving: false,
        }
    }
}

impl<'r> Entity<()> for Ball<'r> {
    fn input(&mut self, input: &InputManager) {
        if !self.moving {
            self.velocity = (0.0, 0.0).into();
            if input.key_down(Key::Left) {
                self.velocity -= (Player::SPEED, 0.0).into();
            }
            if input.key_down(Key::Right) {
                self.velocity += (Player::SPEED, 0.0).into();
            }
            if input.key_down(Key::Space) {
                self.velocity = Vector2::new(Self::SPEED, -Self::SPEED);
                self.moving = true;
            }
        }
    }

    fn update(&mut self, _ctx: &mut (), frame_time: Duration) {
        let velocity = self.velocity * frame_time.as_secs_f32();
        self.move_by(velocity.x, velocity.y);

        // Check for collisions with the walls

        let Vector2 { x, y } = self.position;
        let Vector2 {
            x: width,
            y: height,
        } = self.size;

        if x < 0.0 {
            self.move_by(-x, 0.0);
            self.velocity.x = -self.velocity.x;
        } else if x + width > WINDOW_WIDTH as f32 {
            self.move_by(WINDOW_WIDTH as f32 - x - width, 0.0);
            self.velocity.x = -self.velocity.x;
        }

        if y < 0.0 {
            self.move_by(0.0, -y);
            self.velocity.y = -self.velocity.y;
        } else if y + height > WINDOW_HEIGHT as f32 {
            self.move_by(0.0, WINDOW_HEIGHT as f32 - y - height);
            self.velocity.y = -self.velocity.y;
        }

        self.sprite.set_position(self.position);
    }

    fn draw(&self, _ctx: &(), target: &mut dyn Canvas) {
        self.sprite.draw(target);
    }
}

pub struct MyGame<'r> {
    bg: Sprite<'r>,
    scene: Scene<'r, ()>,
    running: bool,
}

impl<'r> MyGame<'r> {
    fn new(resource_pool: &'r ResourcePool<Images, Audios, Fonts>) -> Self {
        let bg = Sprite::new(resource_pool.get_image(Images::Background));

        let mut scene = Scene::new();

        let debugger = Debugger::new(true, resource_pool.get_font(Fonts::Debug), 10);
        scene.add_entity(Box::new(debugger));

        let player = Player::new(resource_pool.get_image(Images::Player));
        let ball = Ball::new(resource_pool.get_image(Images::Ball), &player);

        scene.add_entity(Box::new(player));
        scene.add_entity(Box::new(ball));

        MyGame {
            bg,
            scene,
            running: true,
        }
    }
}

impl<'r> Game for MyGame<'r> {
    fn is_running(&self) -> bool {
        self.running
    }

    fn input(&mut self, input: &InputManager) {
        if input.key_down(Key::Escape) {
            self.running = false;
        }

        self.scene.input(input);
    }

    fn pre_update(&mut self, engine: &Engine) {
        let _ = engine;
        self.scene.pre_update(&());
    }

    fn update(&mut self, _engine: &mut Engine, delta: Duration) {
        self.scene.update(&mut (), delta);
    }

    fn post_update(&mut self, engine: &Engine) {
        let _ = engine;
        self.scene.post_update(&());
    }

    fn draw<C: Canvas>(&self, target: &mut C) {
        self.bg.draw(target);
        self.scene.draw(&(), target);
    }
}

fn main() {
    let resource_pool = ResourcePool::new(load_image, load_audio, load_font);

    let win_config = WindowConfig {
        title: "My Game".to_string(),
        icon: Some(Icon::from_image(resource_pool.get_image(Images::Icon))),
        cursor: Cursor::from_image(resource_pool.get_image(Images::Cursor)),
        bg_color: Color::BLACK,
        show_cursor: true,
        fps_config: FPSConfig::Unlimited,
        mode: (960, 540).into(),
    };

    let mut engine = Engine::new(Window::new(win_config));

    let game = MyGame::new(&resource_pool);
    engine.run(game);
}
