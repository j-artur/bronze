use std::{rc::Rc, time::Duration};

use bronze::{
    graphics::Sprite,
    input::{InputManager, Key},
    resources::Image,
    scene::Entity,
    shape::{BBox, Movable, Rect, ShapeRef},
    system::Vector2,
    window::Canvas,
};

use crate::{GameContext, StaticEntity, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct Player {
    sprite: Sprite,
    velocity: f32,
    bbox: Rect,
}

impl Player {
    const SPEED: f32 = 640.0;

    pub fn new(image: &Rc<Image>) -> Self {
        let Vector2 {
            x: width,
            y: height,
        } = image.size();

        let x = (WINDOW_WIDTH - width) as f32 / 2.0;
        let y = (WINDOW_HEIGHT - height) as f32 - 32.0;

        let bbox = Rect::new(x, y, width as f32, height as f32);

        Player {
            sprite: Sprite::new(image),
            velocity: 0.0,
            bbox,
        }
    }
}

impl Entity for Player {
    type Ctx = GameContext;

    #[inline]
    fn bbox(&self) -> ShapeRef {
        self.bbox.as_ref()
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
        let width = self.bbox.width;
        let velocity = self.velocity * frame_time.as_secs_f32();

        self.bbox.move_by(velocity, 0.0);

        if self.bbox.x < 0.0 {
            self.bbox.x = 0.0;
        } else if self.bbox.x + width > WINDOW_WIDTH as f32 {
            self.bbox.x = WINDOW_WIDTH as f32 - width;
        }

        ctx.player_top = Vector2::new(self.bbox.x + self.bbox.width / 2.0, self.bbox.y);
    }

    #[inline]
    fn draw(&self, _ctx: &GameContext, target: &mut Canvas) {
        self.sprite.draw(target, self.bbox.position());
    }
}

impl Into<StaticEntity> for Player {
    fn into(self) -> StaticEntity {
        StaticEntity::Player(self)
    }
}
