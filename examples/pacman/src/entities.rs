use std::{rc::Rc, time::Duration};

use bronze::{
    graphics::Sprite,
    input::InputManager,
    input::Key,
    resources::Image,
    scene::{Collision, Entity},
    shape::{BBox, Movable, Rect, ShapeRef},
    window::Canvas,
};

use crate::{resources::*, Pool, WINDOW_HEIGHT, WINDOW_WIDTH};

pub type GameCtx = ();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

mod pivot {
    use super::*;

    pub struct Pivot {
        bbox: Rect,
        up: bool,
        down: bool,
        left: bool,
        right: bool,
    }

    impl Pivot {
        pub fn new(up: bool, down: bool, left: bool, right: bool, x: u32, y: u32) -> Self {
            let mut bbox = Rect::new(0.0, 0.0, 4.0, 4.0);
            bbox.set_center(x as f32, y as f32);

            Self {
                bbox,
                up,
                down,
                left,
                right,
            }
        }

        pub fn dir(&self, dir: Dir) -> bool {
            match dir {
                Dir::Up => self.up,
                Dir::Down => self.down,
                Dir::Left => self.left,
                Dir::Right => self.right,
            }
        }
    }

    impl Entity for Pivot {
        type Ctx = GameCtx;

        fn bbox(&self) -> ShapeRef {
            ShapeRef::Rect(&self.bbox)
        }

        fn update(&mut self, _ctx: &mut GameCtx, _frame_time: Duration) {}

        fn draw(&self, _ctx: &GameCtx, _target: &mut Canvas) {}
    }

    impl Into<StaticEntity> for Pivot {
        fn into(self) -> StaticEntity {
            StaticEntity::Pivot(self)
        }
    }
}
pub use pivot::*;

mod food {
    use super::*;

    pub struct Food {
        bbox: Rect,
        sprite: Sprite,
    }

    impl Food {
        pub fn new(image: &Rc<Image>, x: u32, y: u32) -> Self {
            let mut bbox = Rect::new(0.0, 0.0, 8.0, 8.0);
            bbox.set_center(x as f32, y as f32);

            Food {
                bbox,
                sprite: Sprite::new(image),
            }
        }
    }

    impl Entity for Food {
        type Ctx = GameCtx;

        fn bbox(&self) -> ShapeRef {
            ShapeRef::Rect(&self.bbox)
        }

        fn update(&mut self, _ctx: &mut GameCtx, _frame_time: Duration) {}

        fn draw(&self, _ctx: &GameCtx, target: &mut Canvas) {
            self.sprite
                .draw(target, (self.bbox.x - 4.0, self.bbox.y - 4.0));
        }
    }

    impl Into<StaticEntity> for Food {
        fn into(self) -> StaticEntity {
            StaticEntity::Food(self)
        }
    }
}
pub use food::*;

mod static_entities {
    use super::*;

    pub enum StaticEntity {
        Pivot(Pivot),
        Food(Food),
        Special,
    }

    impl Entity for StaticEntity {
        type Ctx = GameCtx;

        fn bbox(&self) -> ShapeRef {
            match self {
                Self::Pivot(pivot) => pivot.bbox(),
                Self::Food(food) => food.bbox(),
                Self::Special => todo!("Special bbox"),
            }
        }

        fn update(&mut self, ctx: &mut GameCtx, frame_time: Duration) {
            match self {
                Self::Pivot(pivot) => pivot.update(ctx, frame_time),
                Self::Food(food) => food.update(ctx, frame_time),
                Self::Special => todo!("Special update"),
            }
        }

        fn draw(&self, ctx: &GameCtx, target: &mut Canvas) {
            match self {
                Self::Pivot(pivot) => pivot.draw(ctx, target),
                Self::Food(food) => food.draw(ctx, target),
                Self::Special => todo!("Special draw"),
            }
        }
    }

    impl Collision<DynamicEntity> for StaticEntity {
        fn on_collision(&mut self, _other: &DynamicEntity, _ctx: &mut GameCtx) {}
    }
}
pub use static_entities::*;

mod pacman {
    use bronze::shape::Point;

    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum State {
        Idle,
        Moving(Dir),
    }

    pub struct Pacman {
        bbox: Rect,
        curr_state: State,
        next_state: State,
        sprite_up: Sprite,
        sprite_down: Sprite,
        sprite_left: Sprite,
        sprite_right: Sprite,
    }

    impl Pacman {
        pub const SPEED: f32 = 200.0;

        pub fn new(x: u32, y: u32, resource_pool: &Pool) -> Self {
            let sprite_up = Sprite::new(&resource_pool.get_image(Images::PacmanUp));
            let sprite_down = Sprite::new(&resource_pool.get_image(Images::PacmanDown));
            let sprite_left = Sprite::new(&resource_pool.get_image(Images::PacmanLeft));
            let sprite_right = Sprite::new(&resource_pool.get_image(Images::PacmanRight));

            let mut bbox = Rect::new(0.0, 0.0, 40.0, 40.0);
            bbox.set_center(x as f32, y as f32);

            Self {
                bbox,
                curr_state: State::Idle,
                next_state: State::Idle,
                sprite_up,
                sprite_down,
                sprite_left,
                sprite_right,
            }
        }
    }

    impl Entity for Pacman {
        type Ctx = GameCtx;

        fn bbox(&self) -> ShapeRef {
            ShapeRef::Rect(&self.bbox)
        }

        fn input(&mut self, input: &InputManager) {
            if input.key_pressed(Key::Up) {
                self.next_state = State::Moving(Dir::Up);
                if self.curr_state == State::Idle || self.curr_state == State::Moving(Dir::Down) {
                    self.curr_state = self.next_state;
                }
            } else if input.key_pressed(Key::Down) {
                self.next_state = State::Moving(Dir::Down);
                if self.curr_state == State::Idle || self.curr_state == State::Moving(Dir::Up) {
                    self.curr_state = self.next_state;
                }
            } else if input.key_pressed(Key::Left) {
                self.next_state = State::Moving(Dir::Left);
                if self.curr_state == State::Idle || self.curr_state == State::Moving(Dir::Right) {
                    self.curr_state = self.next_state;
                }
            } else if input.key_pressed(Key::Right) {
                self.next_state = State::Moving(Dir::Right);
                if self.curr_state == State::Idle || self.curr_state == State::Moving(Dir::Left) {
                    self.curr_state = self.next_state;
                }
            }
        }

        fn update(&mut self, _ctx: &mut GameCtx, frame_time: Duration) {
            let delta = Self::SPEED * frame_time.as_secs_f32();

            match self.curr_state {
                State::Idle => {}
                State::Moving(Dir::Up) => {
                    self.bbox.move_by(0.0, -delta);
                }
                State::Moving(Dir::Down) => {
                    self.bbox.move_by(0.0, delta);
                }
                State::Moving(Dir::Left) => {
                    self.bbox.move_by(-delta, 0.0);
                }
                State::Moving(Dir::Right) => {
                    self.bbox.move_by(delta, 0.0);
                }
            }

            if self.bbox.left() < 0.0 {
                self.bbox.set_position(0.0, self.bbox.top());
            } else if self.bbox.right() > WINDOW_WIDTH as f32 {
                self.bbox
                    .set_position(WINDOW_WIDTH as f32 - self.bbox.width(), self.bbox.top());
            }

            if self.bbox.top() < 0.0 {
                self.bbox.set_position(self.bbox.left(), 0.0);
            } else if self.bbox.bottom() > WINDOW_HEIGHT as f32 {
                self.bbox
                    .set_position(self.bbox.left(), WINDOW_HEIGHT as f32 - self.bbox.height());
            }
        }

        fn draw(&self, _ctx: &GameCtx, target: &mut Canvas) {
            let sprite = match self.curr_state {
                State::Moving(Dir::Up) => &self.sprite_up,
                State::Moving(Dir::Down) => &self.sprite_down,
                State::Moving(Dir::Left) => &self.sprite_left,
                State::Moving(Dir::Right) => &self.sprite_right,
                State::Idle => match self.next_state {
                    State::Moving(Dir::Up) => &self.sprite_up,
                    State::Moving(Dir::Down) => &self.sprite_down,
                    State::Moving(Dir::Left) => &self.sprite_left,
                    State::Moving(Dir::Right) => &self.sprite_right,
                    State::Idle => &self.sprite_right,
                },
            };

            sprite.draw(target, (self.bbox.x - 4.0, self.bbox.y - 4.0));
        }
    }

    impl Collision<Pivot> for Pacman {
        fn on_collision(&mut self, pivot: &Pivot, _ctx: &mut GameCtx) {
            if Point::new(self.bbox.center_x(), self.bbox.center_y()).intersects(&pivot.bbox()) {
                if let State::Moving(dir) = self.curr_state {
                    if !pivot.dir(dir) {
                        self.bbox
                            .set_center(pivot.bbox().center_x(), pivot.bbox().center_y());
                        self.curr_state = State::Idle;
                    }
                }
                if let State::Moving(next_dir) = self.next_state {
                    if pivot.dir(next_dir) {
                        self.bbox
                            .set_center(pivot.bbox().center_x(), pivot.bbox().center_y());
                        self.curr_state = self.next_state;
                    }
                }
            }
        }
    }

    impl Into<DynamicEntity> for Pacman {
        fn into(self) -> DynamicEntity {
            DynamicEntity::Pacman(self)
        }
    }
}
pub use pacman::*;

mod dynamic_entities {
    use super::*;

    pub enum DynamicEntity {
        Pacman(Pacman),
        Ghost,
    }

    impl Entity for DynamicEntity {
        type Ctx = GameCtx;

        fn bbox(&self) -> ShapeRef {
            match self {
                DynamicEntity::Pacman(pacman) => pacman.bbox(),
                DynamicEntity::Ghost => todo!("Ghost::bbox"),
            }
        }

        fn input(&mut self, input: &InputManager) {
            match self {
                DynamicEntity::Pacman(pacman) => pacman.input(input),
                DynamicEntity::Ghost => todo!("Ghost::input"),
            }
        }

        fn pre_update(&mut self, ctx: &GameCtx) {
            match self {
                DynamicEntity::Pacman(pacman) => pacman.pre_update(ctx),
                DynamicEntity::Ghost => todo!("Ghost::pre_update"),
            }
        }

        fn update(&mut self, ctx: &mut GameCtx, frame_time: Duration) {
            match self {
                DynamicEntity::Pacman(pacman) => pacman.update(ctx, frame_time),
                DynamicEntity::Ghost => todo!("Ghost::update"),
            }
        }

        fn draw(&self, ctx: &GameCtx, target: &mut Canvas) {
            match self {
                DynamicEntity::Pacman(pacman) => pacman.draw(ctx, target),
                DynamicEntity::Ghost => todo!("Ghost::draw"),
            }
        }

        fn post_update(&mut self, ctx: &GameCtx) {
            match self {
                DynamicEntity::Pacman(pacman) => pacman.post_update(ctx),
                DynamicEntity::Ghost => todo!("Ghost::post_update"),
            }
        }
    }

    impl Collision<StaticEntity> for DynamicEntity {
        fn on_collision(&mut self, other: &StaticEntity, ctx: &mut GameCtx) {
            use DynamicEntity::*;
            use StaticEntity::*;
            match (self, other) {
                (Pacman(pacman), Pivot(pivot)) => pacman.on_collision(pivot, ctx),
                _ => todo!("Implement collision between static and dynamic entities"),
            }
        }
    }

    impl Collision<DynamicEntity> for DynamicEntity {
        fn on_collision(&mut self, other: &DynamicEntity, ctx: &mut GameCtx) {
            let (_, _) = (other, ctx);
            todo!("Implement collision between dynamic entities")
        }
    }
}
pub use dynamic_entities::*;
