use std::time::Duration;

use bronze::{
    graphics::Sprite,
    input::{InputManager, Key},
    resources::Image,
    scene::Entity,
    sfml::system::Vector2,
    shape::{Movable, Rect, ShapeRef},
    window::Canvas,
};

use crate::{GameContext, StaticEntity, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct Player<'r> {
    sprite: Sprite<'r>,
    velocity: f32,
    hitbox: Rect,
}

impl<'r> Player<'r> {
    const SPEED: f32 = 640.0;

    pub fn new(image: &'r Image) -> Self {
        let Vector2 {
            x: width,
            y: height,
        } = image.size();

        let x = (WINDOW_WIDTH - width) as f32 / 2.0;
        let y = (WINDOW_HEIGHT - height) as f32 - 32.0;

        let hitbox = Rect::new(x, y, width as f32, height as f32);

        Player {
            sprite: Sprite::new(image),
            velocity: 0.0,
            hitbox,
        }
    }
}

impl Entity for Player<'_> {
    type Ctx = GameContext;

    #[inline]
    fn bbox(&self) -> ShapeRef {
        self.hitbox.as_ref()
    }

    #[inline]
    fn input(&mut self, input: &InputManager) {
        self.velocity = 0.0;
        if input.key_down(Key::A) {
            self.velocity -= Self::SPEED;
        }
        if input.key_down(Key::D) {
            self.velocity += Self::SPEED;
        }
    }

    #[inline]
    fn update(&mut self, ctx: &mut GameContext, frame_time: Duration) {
        let width = self.hitbox.width;
        let velocity = self.velocity * frame_time.as_secs_f32();

        self.hitbox.move_by(velocity, 0.0);

        if self.hitbox.x < 0.0 {
            self.hitbox.x = 0.0;
        } else if self.hitbox.x + width > WINDOW_WIDTH as f32 {
            self.hitbox.x = WINDOW_WIDTH as f32 - width;
        }

        ctx.player_top = Vector2::new(self.hitbox.x + self.hitbox.width / 2.0, self.hitbox.y);

        self.sprite.set_position(self.hitbox.x, self.hitbox.y);
    }

    #[inline]
    fn draw(&self, _ctx: &GameContext, target: &mut Canvas) {
        self.sprite.draw(target);
    }
}

impl<'r> Into<StaticEntity<'r>> for Player<'r> {
    fn into(self) -> StaticEntity<'r> {
        StaticEntity::Player(self)
    }
}
