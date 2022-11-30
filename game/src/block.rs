use std::time::Duration;

use bronze::{
    resources::Image,
    scene::{Collision, Entity},
    sfml::system::Vector2,
    shape::{BBox, Movable, Rect, ShapeRef},
    sprite::Sprite,
    window::Canvas,
};

use crate::{ball::Ball, GameContext, StaticEntity, WINDOW_HEIGHT};

pub struct Block<'r> {
    sprite: Sprite<'r>,
    hitbox: Rect,
    falling: bool,
}

impl<'r> Block<'r> {
    const SPEED: f32 = 640.0;

    pub fn new(image: &'r Image, x: f32, y: f32) -> Self {
        let Vector2 {
            x: width,
            y: height,
        } = image.size();
        let width = width as f32;
        let height = height as f32;

        let hitbox = Rect {
            x,
            y,
            width,
            height,
        };

        let sprite = Sprite::new(image);

        Block {
            sprite,
            hitbox,
            falling: false,
        }
    }

    pub fn fall(&mut self) {
        self.falling = true;
    }

    pub fn is_falling(&self) -> bool {
        self.falling
    }
}

impl Entity for Block<'_> {
    type Ctx = GameContext;

    fn bbox(&self) -> ShapeRef {
        self.hitbox.as_ref()
    }

    fn update(&mut self, _ctx: &mut GameContext, frame_time: Duration) {
        if self.falling {
            let velocity = Self::SPEED * frame_time.as_secs_f32();
            self.hitbox.move_by(0.0, velocity);
        }

        if self.hitbox.y > WINDOW_HEIGHT as f32 {
            self.falling = false;
        }
    }

    fn draw(&self, _ctx: &GameContext, target: &mut Canvas) {
        self.sprite.draw(target, self.hitbox.position(), 0.0, 1.0);
    }
}

impl<'r> Into<StaticEntity<'r>> for Block<'r> {
    fn into(self) -> StaticEntity<'r> {
        StaticEntity::Block(self)
    }
}

impl<'r> Collision<Ball<'r>> for Block<'r> {
    #[inline]
    fn on_collision(&mut self, _other: &Ball<'r>, _ctx: &mut GameContext) {
        self.fall();
    }
}
