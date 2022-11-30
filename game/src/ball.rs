use std::time::Duration;

use bronze::{
    input::{InputManager, Key},
    resources::Image,
    scene::{Collision, Entity},
    sfml::system::Vector2,
    shape::{BBox, Circle, Movable, ShapeRef},
    sprite::Sprite,
    window::Canvas,
};

use crate::{GameContext, Player, StaticEntity, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct Ball<'r> {
    sprite: Sprite<'r>,
    velocity: Vector2<f32>,
    moving: bool,
    hitbox: Circle,
}

impl<'r> Ball<'r> {
    const SPEED: f32 = 480.0;

    pub fn new(image: &'r Image, player: &Player) -> Self {
        let Vector2 { x, y } = image.size();

        let width = x as f32;
        let height = y as f32;

        let x = player.bbox().center_x() - width / 2.0;
        let y = player.bbox().top() - height;

        let hitbox = Circle {
            x,
            y,
            radius: width / 2.0,
        };

        Ball {
            sprite: Sprite::new(image),
            velocity: Vector2::new(0.0, 0.0),
            hitbox,
            moving: false,
        }
    }
}

impl<'r> Collision<Ball<'r>> for Ball<'r> {
    fn on_collision(&mut self, _other: &Ball<'r>, _ctx: &mut GameContext) {}
}

impl Entity for Ball<'_> {
    type Ctx = GameContext;

    #[inline]
    fn bbox(&self) -> ShapeRef {
        self.hitbox.as_ref()
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
        self.hitbox.move_by(velocity.x, velocity.y);

        if self.hitbox.x < 0.0 {
            self.hitbox.move_by(-self.hitbox.x, 0.0);
            self.velocity.x = -self.velocity.x;
        } else if self.hitbox.x + self.hitbox.width() > WINDOW_WIDTH as f32 {
            self.hitbox.move_by(
                WINDOW_WIDTH as f32 - self.hitbox.x - self.hitbox.width(),
                0.0,
            );
            self.velocity.x = -self.velocity.x;
        }

        if self.hitbox.y < 0.0 {
            self.hitbox.move_by(0.0, -self.hitbox.y);
            self.velocity.y = -self.velocity.y;
        } else if self.hitbox.y + self.hitbox.height() > WINDOW_HEIGHT as f32 {
            self.hitbox.move_by(
                0.0,
                WINDOW_HEIGHT as f32 - self.hitbox.y - self.hitbox.height(),
            );
            self.velocity.y = -self.velocity.y;
        }
    }

    #[inline]
    fn post_update(&mut self, ctx: &GameContext) {
        if !self.moving {
            self.hitbox.set_position(
                ctx.player_top.x - self.hitbox.width() / 2.0,
                ctx.player_top.y - self.hitbox.height(),
            );
        }
    }

    #[inline]
    fn draw(&self, _ctx: &GameContext, target: &mut Canvas) {
        self.sprite.draw(target, self.hitbox.position(), 0.0, 1.0);
    }
}

impl Collision<StaticEntity<'_>> for Ball<'_> {
    #[inline]
    fn on_collision(&mut self, other: &StaticEntity<'_>, _ctx: &mut GameContext) {
        match other {
            StaticEntity::Player(player) => {
                let left = self.hitbox.right() > player.bbox().left()
                    && self.hitbox.left() < player.bbox().left();
                let right = self.hitbox.left() < player.bbox().right()
                    && self.hitbox.right() > player.bbox().right();
                let top = self.hitbox.bottom() > player.bbox().top()
                    && self.hitbox.top() < player.bbox().top();
                let bottom = self.hitbox.top() < player.bbox().bottom()
                    && self.hitbox.bottom() > player.bbox().bottom();

                if left {
                    self.hitbox
                        .move_by(player.bbox().left() - self.hitbox.right(), 0.0);
                    self.velocity.x = -self.velocity.x;
                }

                if right {
                    self.hitbox
                        .move_by(player.bbox().right() - self.hitbox.left(), 0.0);
                    self.velocity.x = -self.velocity.x;
                }

                if top {
                    self.hitbox
                        .move_by(0.0, player.bbox().top() - self.hitbox.bottom());
                    self.velocity.y = -self.velocity.y;
                }

                if bottom {
                    self.hitbox
                        .move_by(0.0, player.bbox().bottom() - self.hitbox.top());
                    self.velocity.y = -self.velocity.y;
                }
            }
            StaticEntity::Block(block) => {
                if !block.is_falling() {
                    let block = block.bbox();
                    let ball = &self.hitbox;

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
