use std::{rc::Rc, time::Duration};

use bronze::{
    graphics::{Color, Sprite},
    resources::Image,
    scene::{Collision, Entity},
    shape::{BBox, Movable, Rect, ShapeRef},
    system::Vector2,
    window::Canvas,
};

use crate::{ball::Ball, GameContext, StaticEntity, WINDOW_HEIGHT};

pub struct Block {
    sprite: Sprite,
    bbox: Rect,
    falling: bool,
}

impl Block {
    const SPEED: f32 = 640.0;

    pub fn new(image: &Rc<Image>, x: f32, y: f32) -> Self {
        let Vector2 {
            x: width,
            y: height,
        } = image.size();
        let width = width as f32;
        let height = height as f32;

        let bbox = Rect {
            x,
            y,
            width,
            height,
        };

        let sprite = Sprite::new(&image);

        Block {
            sprite,
            bbox,
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

impl Entity<GameContext> for Block {
    fn bbox(&self) -> ShapeRef {
        self.bbox.as_ref()
    }

    fn update(&mut self, _ctx: &mut GameContext, frame_time: Duration) {
        if self.falling {
            let velocity = Self::SPEED * frame_time.as_secs_f32();
            self.bbox.move_by(0.0, velocity);
        }

        if self.bbox.y > WINDOW_HEIGHT as f32 {
            self.falling = false;
        }
    }

    fn draw(&self, _ctx: &GameContext, target: &mut Canvas) {
        if !self.falling {
            self.sprite.draw(target, self.bbox.position());
        } else {
            self.sprite
                .draw(target, (self.bbox.position(), Color::from(0xffffff7f)));
        }
    }
}

impl Into<StaticEntity> for Block {
    fn into(self) -> StaticEntity {
        StaticEntity::Block(self)
    }
}

impl Collision<Ball, GameContext> for Block {
    #[inline]
    fn on_collision(&mut self, _other: &Ball, _ctx: &mut GameContext) {
        self.fall();
    }
}
