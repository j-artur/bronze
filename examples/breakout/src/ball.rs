use std::{rc::Rc, time::Duration};

use bronze::{
    graphics::Sprite,
    input::{InputManager, Key},
    resources::Image,
    scene::{Collision, Entity},
    shape::{BBox, Circle, Movable, ShapeRef},
    system::Vector2,
    window::Canvas,
};

use crate::{GameContext, Player, StaticEntity, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct Ball {
    sprite: Sprite,
    velocity: Vector2<f32>,
    moving: bool,
    bbox: Circle,
}

impl Ball {
    const SPEED: f32 = 480.0;

    pub fn new(image: &Rc<Image>, player: &Player) -> Self {
        let Vector2 { x, y } = image.size();

        let width = x as f32;
        let height = y as f32;

        let x = player.bbox().center_x() - width / 2.0;
        let y = player.bbox().top() - height;

        let bbox = Circle {
            x,
            y,
            radius: width / 2.0,
        };

        Ball {
            sprite: Sprite::new(image),
            velocity: Vector2::new(0.0, 0.0),
            bbox,
            moving: false,
        }
    }
}

impl Collision<Ball, GameContext> for Ball {
    fn on_collision(&mut self, _other: &Ball, _ctx: &mut GameContext) {}
}

impl Entity<GameContext> for Ball {
    #[inline]
    fn bbox(&self) -> ShapeRef {
        self.bbox.as_ref()
    }

    #[inline]
    fn input(&mut self, input: &InputManager) {
        if !self.moving && input.key_down(Key::Space) {
            self.velocity = Vector2::new(Self::SPEED, -Self::SPEED);
            self.moving = true;
        }
    }

    #[inline]
    fn update(&mut self, _ctx: &mut GameContext, frame_time: Duration) {
        let velocity = self.velocity * frame_time.as_secs_f32();
        self.bbox.move_by(velocity.x, velocity.y);

        if self.bbox.x < 0.0 {
            self.bbox.move_by(-self.bbox.x, 0.0);
            self.velocity.x = -self.velocity.x;
        } else if self.bbox.x + self.bbox.width() > WINDOW_WIDTH as f32 {
            self.bbox
                .move_by(WINDOW_WIDTH as f32 - self.bbox.x - self.bbox.width(), 0.0);
            self.velocity.x = -self.velocity.x;
        }

        if self.bbox.y < 0.0 {
            self.bbox.move_by(0.0, -self.bbox.y);
            self.velocity.y = -self.velocity.y;
        } else if self.bbox.y + self.bbox.height() > WINDOW_HEIGHT as f32 {
            self.bbox
                .move_by(0.0, WINDOW_HEIGHT as f32 - self.bbox.y - self.bbox.height());
            self.velocity.y = -self.velocity.y;
        }
    }

    #[inline]
    fn post_update(&mut self, ctx: &GameContext) {
        if !self.moving {
            self.bbox.set_position(
                ctx.player_top.x - self.bbox.width() / 2.0,
                ctx.player_top.y - self.bbox.height(),
            );
        }
    }

    #[inline]
    fn draw(&self, _ctx: &GameContext, target: &mut Canvas) {
        self.sprite.draw(target, self.bbox.position());
    }
}

impl Collision<StaticEntity, GameContext> for Ball {
    #[inline]
    fn on_collision(&mut self, other: &StaticEntity, _ctx: &mut GameContext) {
        match other {
            StaticEntity::Player(player) => {
                let left = self.bbox.right() > player.bbox().left()
                    && self.bbox.left() < player.bbox().left();
                let right = self.bbox.left() < player.bbox().right()
                    && self.bbox.right() > player.bbox().right();
                let top = self.bbox.bottom() > player.bbox().top()
                    && self.bbox.top() < player.bbox().top();
                let bottom = self.bbox.top() < player.bbox().bottom()
                    && self.bbox.bottom() > player.bbox().bottom();

                if left {
                    self.bbox
                        .move_by(player.bbox().left() - self.bbox.right(), 0.0);
                    self.velocity.x = -self.velocity.x;
                }

                if right {
                    self.bbox
                        .move_by(player.bbox().right() - self.bbox.left(), 0.0);
                    self.velocity.x = -self.velocity.x;
                }

                if top {
                    self.bbox
                        .move_by(0.0, player.bbox().top() - self.bbox.bottom());
                    self.velocity.y = -self.velocity.y;
                }

                if bottom {
                    self.bbox
                        .move_by(0.0, player.bbox().bottom() - self.bbox.top());
                    self.velocity.y = -self.velocity.y;
                }
            }
            StaticEntity::Block(block) => {
                if !block.is_falling() {
                    let block = block.bbox();
                    let ball = &self.bbox;

                    if ball.right() > block.left() && ball.left() < block.left()
                        || ball.left() < block.right() && ball.right() > block.right()
                    {
                        self.velocity.x = -self.velocity.x;
                    }
                    if ball.bottom() > block.top() && ball.top() < block.top()
                        || ball.top() < block.bottom() && ball.bottom() > block.bottom()
                    {
                        self.velocity.y = -self.velocity.y;
                    }
                }
            }
        }
    }
}
