use std::{rc::Rc, time::Duration};

use bronze::{
    graphics::Sprite,
    input::InputManager,
    input::Key,
    resources::Image,
    scene::{Collision, Entity},
    shape::{BBox, Movable, Point, Rect, ShapeRef},
    window::Canvas,
};

use crate::{
    game::{Dir, GameCtx},
    WINDOW_HEIGHT, WINDOW_WIDTH,
};

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
            let mut bbox = Rect::new(0.0, 0.0, 8.0, 8.0);
            bbox.set_center(x as f32, y as f32);

            Pivot {
                bbox,
                up,
                down,
                left,
                right,
            }
        }

        pub fn allows(&self, dir: Dir) -> bool {
            match dir {
                Dir::Up => self.up,
                Dir::Down => self.down,
                Dir::Left => self.left,
                Dir::Right => self.right,
            }
        }
    }

    impl Entity<GameCtx> for Pivot {
        fn bbox(&self) -> ShapeRef {
            self.bbox.as_ref()
        }

        fn should_be_removed(&self) -> bool {
            false
        }
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
        eaten: bool,
    }

    impl Food {
        pub fn new(image: &Rc<Image>, x: u32, y: u32) -> Self {
            let mut bbox = Rect::new(0.0, 0.0, 8.0, 8.0);
            bbox.set_center(x as f32, y as f32);

            Food {
                bbox,
                sprite: Sprite::new(image),
                eaten: false,
            }
        }
    }

    impl Entity<GameCtx> for Food {
        fn bbox(&self) -> ShapeRef {
            self.bbox.as_ref()
        }

        fn should_be_removed(&self) -> bool {
            self.eaten
        }

        fn draw(&self, _ctx: &GameCtx, target: &mut Canvas) {
            self.sprite.draw_centered(target, self.bbox.center());
        }
    }

    impl Collision<Pacman, GameCtx> for Food {
        fn on_collision(&mut self, _pacman: &Pacman, ctx: &mut GameCtx) {
            self.eaten = true;
            ctx.food_count -= 1;
        }
    }

    impl Into<StaticEntity> for Food {
        fn into(self) -> StaticEntity {
            StaticEntity::Food(self)
        }
    }
}
pub use food::*;

mod special {
    use super::*;

    pub struct Special {
        bbox: Rect,
        sprite: Sprite,
        eaten: bool,
    }

    impl Special {
        pub fn new(image: &Rc<Image>, x: u32, y: u32) -> Self {
            let mut bbox = Rect::new(0.0, 0.0, 16.0, 16.0);
            bbox.set_center(x as f32, y as f32);

            Special {
                bbox,
                sprite: Sprite::new(image),
                eaten: false,
            }
        }
    }

    impl Entity<GameCtx> for Special {
        fn bbox(&self) -> ShapeRef {
            self.bbox.as_ref()
        }

        fn should_be_removed(&self) -> bool {
            self.eaten
        }

        fn draw(&self, _ctx: &GameCtx, target: &mut Canvas) {
            self.sprite.draw_centered(target, self.bbox.center());
        }
    }

    impl Collision<Pacman, GameCtx> for Special {
        fn on_collision(&mut self, _pacman: &Pacman, ctx: &mut GameCtx) {
            self.eaten = true;
            ctx.food_count -= 1;
        }
    }

    impl Into<StaticEntity> for Special {
        fn into(self) -> StaticEntity {
            StaticEntity::Special(self)
        }
    }
}
pub use special::*;

mod static_entities {
    use super::*;

    pub enum StaticEntity {
        Pivot(Pivot),
        Food(Food),
        Special(Special),
    }

    impl Entity<GameCtx> for StaticEntity {
        fn bbox(&self) -> ShapeRef {
            match self {
                Self::Pivot(pivot) => pivot.bbox(),
                Self::Food(food) => food.bbox(),
                Self::Special(special) => special.bbox(),
            }
        }

        fn should_be_removed(&self) -> bool {
            match self {
                Self::Pivot(pivot) => pivot.should_be_removed(),
                Self::Food(food) => food.should_be_removed(),
                Self::Special(special) => special.should_be_removed(),
            }
        }

        fn update(&mut self, ctx: &mut GameCtx, frame_time: Duration) {
            match self {
                Self::Pivot(pivot) => pivot.update(ctx, frame_time),
                Self::Food(food) => food.update(ctx, frame_time),
                Self::Special(special) => special.update(ctx, frame_time),
            }
        }

        fn draw(&self, ctx: &GameCtx, target: &mut Canvas) {
            match self {
                Self::Pivot(pivot) => pivot.draw(ctx, target),
                Self::Food(food) => food.draw(ctx, target),
                Self::Special(special) => special.draw(ctx, target),
            }
        }
    }

    impl Collision<DynamicEntity, GameCtx> for StaticEntity {
        fn on_collision(&mut self, other: &DynamicEntity, ctx: &mut GameCtx) {
            use DynamicEntity::*;
            use StaticEntity::*;
            match (self, other) {
                (Food(food), Pacman(pacman)) => food.on_collision(pacman, ctx),
                (Special(special), Pacman(pacman)) => special.on_collision(pacman, ctx),
                (Pivot(_), Pacman(_)) => {}
                (Pivot(_), Ghost(_)) => {}
                (Food(_), Ghost(_)) => {}
                (Special(_), Ghost(_)) => {}
            }
        }
    }
}
pub use static_entities::*;

mod pacman {
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
    }

    impl Pacman {
        pub const SPEED: f32 = 200.0;

        pub fn new(x: u32, y: u32) -> Self {
            let mut bbox = Rect::new(0.0, 0.0, 40.0, 40.0);
            bbox.set_center(x as f32, y as f32);

            Pacman {
                bbox,
                curr_state: State::Idle,
                next_state: State::Idle,
            }
        }
    }

    impl Entity<GameCtx> for Pacman {
        fn bbox(&self) -> ShapeRef {
            self.bbox.as_ref()
        }

        fn should_be_removed(&self) -> bool {
            false
        }

        fn input(&mut self, input: &InputManager) {
            if input.key_pressed(Key::Up) {
                self.next_state = State::Moving(Dir::Up);
                if self.curr_state == State::Moving(Dir::Down) {
                    self.curr_state = self.next_state;
                }
            } else if input.key_pressed(Key::Down) {
                self.next_state = State::Moving(Dir::Down);
                if self.curr_state == State::Moving(Dir::Up) {
                    self.curr_state = self.next_state;
                }
            } else if input.key_pressed(Key::Left) {
                self.next_state = State::Moving(Dir::Left);
                if self.curr_state == State::Moving(Dir::Right) {
                    self.curr_state = self.next_state;
                }
            } else if input.key_pressed(Key::Right) {
                self.next_state = State::Moving(Dir::Right);
                if self.curr_state == State::Moving(Dir::Left) {
                    self.curr_state = self.next_state;
                }
            }
        }

        fn update(&mut self, ctx: &mut GameCtx, frame_time: Duration) {
            let delta_speed = Self::SPEED * frame_time.as_secs_f32();

            match self.curr_state {
                State::Idle => {}
                State::Moving(Dir::Up) => self.bbox.move_by(0.0, -delta_speed),
                State::Moving(Dir::Down) => self.bbox.move_by(0.0, delta_speed),
                State::Moving(Dir::Left) => self.bbox.move_by(-delta_speed, 0.0),
                State::Moving(Dir::Right) => self.bbox.move_by(delta_speed, 0.0),
            };

            if self.bbox.center_x() < 0.0 {
                self.bbox
                    .set_center(WINDOW_WIDTH as f32, self.bbox.center_y());
            } else if self.bbox.center_x() > WINDOW_WIDTH as f32 {
                self.bbox.set_center(0.0, self.bbox.center_y());
            }

            if self.bbox.center_y() < 0.0 {
                self.bbox
                    .set_center(self.bbox.center_x(), WINDOW_HEIGHT as f32);
            } else if self.bbox.center_y() > WINDOW_HEIGHT as f32 {
                self.bbox.set_center(self.bbox.center_x(), 0.0);
            }

            ctx.pacman_center = Point::new(self.bbox.center_x(), self.bbox.center_y());
        }

        fn draw(&self, ctx: &GameCtx, target: &mut Canvas) {
            let sprite = match self.curr_state {
                State::Moving(dir) => ctx.pacman_sprites.sprite(&dir),
                State::Idle => match self.next_state {
                    State::Moving(dir) => ctx.pacman_sprites.sprite(&dir),
                    State::Idle => ctx.pacman_sprites.sprite(&Dir::Right),
                },
            };

            sprite.draw_centered(target, self.bbox.center());

            let mut rect = self.bbox.clone();

            rect.move_by(-(WINDOW_WIDTH as f32), 0.0);
            sprite.draw_centered(target, rect.center());

            rect.move_by(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
            sprite.draw_centered(target, rect.center());

            rect.move_by(WINDOW_WIDTH as f32, -(WINDOW_HEIGHT as f32));
            sprite.draw_centered(target, rect.center());

            rect.move_by(-(WINDOW_WIDTH as f32), -(WINDOW_HEIGHT as f32));
            sprite.draw_centered(target, rect.center());
        }
    }

    impl Collision<Pivot, GameCtx> for Pacman {
        fn on_collision(&mut self, pivot: &Pivot, _ctx: &mut GameCtx) {
            if !self.bbox.center().intersects(&pivot.bbox()) {
                return;
            }

            if let State::Moving(dir) = self.curr_state {
                if !pivot.allows(dir) {
                    let Point { x, y } = pivot.bbox().center();
                    self.bbox.set_center(x, y);
                    self.curr_state = State::Idle;
                }
            }

            if let State::Moving(next_dir) = self.next_state {
                if self.curr_state != self.next_state && pivot.allows(next_dir) {
                    let Point { x, y } = pivot.bbox().center();
                    self.bbox.set_center(x, y);
                    self.curr_state = self.next_state;
                }
            }
        }
    }

    impl Collision<Food, GameCtx> for Pacman {
        fn on_collision(&mut self, _food: &Food, ctx: &mut GameCtx) {
            ctx.score += 1;
        }
    }

    impl Collision<Special, GameCtx> for Pacman {
        fn on_collision(&mut self, _special: &Special, ctx: &mut GameCtx) {
            ctx.score += 10;
        }
    }

    impl Collision<Ghost, GameCtx> for Pacman {
        fn on_collision(&mut self, _ghost: &Ghost, ctx: &mut GameCtx) {
            ctx.pacman_alive = false;
        }
    }

    impl Into<DynamicEntity> for Pacman {
        fn into(self) -> DynamicEntity {
            DynamicEntity::Pacman(self)
        }
    }
}
pub use pacman::*;

mod ghost {
    use crate::resources::GhostColor;

    use super::*;

    pub struct Ghost {
        bbox: Rect,
        color: GhostColor,
        dir: Dir,
        next_dir: Option<Dir>,
        alive: bool,
    }

    impl Ghost {
        pub const SPEED: f32 = 180.0;

        pub fn new(x: u32, y: u32, color: GhostColor) -> Self {
            let mut bbox = Rect::new(0.0, 0.0, 40.0, 40.0);
            bbox.set_center(x as f32, y as f32);

            Ghost {
                bbox,
                color,
                dir: Dir::Up,
                next_dir: None,
                alive: true,
            }
        }
    }

    impl Entity<GameCtx> for Ghost {
        fn should_be_removed(&self) -> bool {
            !self.alive
        }

        fn bbox(&self) -> ShapeRef {
            self.bbox.as_ref()
        }

        fn update(&mut self, _ctx: &mut GameCtx, frame_time: Duration) {
            let delta_speed = Self::SPEED * frame_time.as_secs_f32();

            match self.dir {
                Dir::Up => self.bbox.move_by(0.0, -delta_speed),
                Dir::Down => self.bbox.move_by(0.0, delta_speed),
                Dir::Left => self.bbox.move_by(-delta_speed, 0.0),
                Dir::Right => self.bbox.move_by(delta_speed, 0.0),
            };

            if self.bbox.center_x() < 0.0 {
                let (x, y) = (WINDOW_WIDTH as f32, self.bbox.center_y());
                self.bbox.set_center(x, y);
            } else if self.bbox.center_x() > WINDOW_WIDTH as f32 {
                self.bbox.set_center(0.0, self.bbox.center_y());
            }

            if self.bbox.center_y() < 0.0 {
                let (x, y) = (self.bbox.center_x(), WINDOW_HEIGHT as f32);
                self.bbox.set_center(x, y);
            } else if self.bbox.center_y() > WINDOW_HEIGHT as f32 {
                self.bbox.set_center(self.bbox.center_x(), 0.0);
            }

            // self.next_dir = match self.bbox.center().angle(&ctx.pacman_center) {
            //     angle if angle < 45.0 || angle > 315.0 => Some(Dir::Right),
            //     angle if angle < 135.0 => Some(Dir::Down),
            //     angle if angle < 225.0 => Some(Dir::Left),
            //     angle if angle < 315.0 => Some(Dir::Up),
            //     _ => None,
            // };
        }

        fn draw(&self, ctx: &GameCtx, target: &mut Canvas) {
            let sprite = if let Some(dir) = self.next_dir {
                ctx.ghost_sprites.sprite(&self.color, &dir)
            } else {
                ctx.ghost_sprites.sprite(&self.color, &self.dir)
            };

            sprite.draw_centered(target, self.bbox.center());
        }
    }

    impl Collision<Pivot, GameCtx> for Ghost {
        fn on_collision(&mut self, pivot: &Pivot, _ctx: &mut GameCtx) {
            if !self.bbox.center().intersects(&pivot.bbox()) {
                return;
            }

            // if let Some(next_dir) = self.next_dir {
            //     if pivot.allows(next_dir) {
            //         let Point { x, y } = pivot.bbox().center();
            //         self.bbox.set_center(x, y);
            //         return self.dir = next_dir;
            //     }
            // }

            if !pivot.allows(self.dir) {
                let Point { x, y } = pivot.bbox().center();
                self.bbox.set_center(x, y);
                // if let Some(next_dir) = self.next_dir {
                //     if pivot.allows(next_dir) {
                //         self.dir = next_dir;
                //     } else {
                //         self.dir = Dir::random_dirs()
                //             .iter()
                //             .find(|dir| pivot.allows(**dir))
                //             .unwrap_or(&self.dir.opposite())
                //             .clone();
                //     }
                // } else {
                self.dir = Dir::random_dirs()
                    .iter()
                    .find(|dir| pivot.allows(**dir))
                    .unwrap_or(&self.dir.opposite())
                    .clone();
                // }
            }
        }
    }

    impl Collision<Pacman, GameCtx> for Ghost {
        fn on_collision(&mut self, _pacman: &Pacman, _ctx: &mut GameCtx) {
            self.alive = false;
        }
    }

    impl Into<DynamicEntity> for Ghost {
        fn into(self) -> DynamicEntity {
            DynamicEntity::Ghost(self)
        }
    }
}
pub use ghost::*;

mod dynamic_entities {
    use super::*;

    pub enum DynamicEntity {
        Pacman(Pacman),
        Ghost(Ghost),
    }

    impl Entity<GameCtx> for DynamicEntity {
        fn bbox(&self) -> ShapeRef {
            match self {
                DynamicEntity::Pacman(pacman) => pacman.bbox(),
                DynamicEntity::Ghost(ghost) => ghost.bbox(),
            }
        }

        fn should_be_removed(&self) -> bool {
            match self {
                DynamicEntity::Pacman(pacman) => pacman.should_be_removed(),
                DynamicEntity::Ghost(ghost) => ghost.should_be_removed(),
            }
        }

        fn input(&mut self, input: &InputManager) {
            match self {
                DynamicEntity::Pacman(pacman) => pacman.input(input),
                DynamicEntity::Ghost(ghost) => ghost.input(input),
            }
        }

        fn pre_update(&mut self, ctx: &GameCtx) {
            match self {
                DynamicEntity::Pacman(pacman) => pacman.pre_update(ctx),
                DynamicEntity::Ghost(ghost) => ghost.pre_update(ctx),
            }
        }

        fn update(&mut self, ctx: &mut GameCtx, frame_time: Duration) {
            match self {
                DynamicEntity::Pacman(pacman) => pacman.update(ctx, frame_time),
                DynamicEntity::Ghost(ghost) => ghost.update(ctx, frame_time),
            }
        }

        fn draw(&self, ctx: &GameCtx, target: &mut Canvas) {
            match self {
                DynamicEntity::Pacman(pacman) => pacman.draw(ctx, target),
                DynamicEntity::Ghost(ghost) => ghost.draw(ctx, target),
            }
        }

        fn post_update(&mut self, ctx: &GameCtx) {
            match self {
                DynamicEntity::Pacman(pacman) => pacman.post_update(ctx),
                DynamicEntity::Ghost(ghost) => ghost.post_update(ctx),
            }
        }
    }

    impl Collision<StaticEntity, GameCtx> for DynamicEntity {
        fn on_collision(&mut self, other: &StaticEntity, ctx: &mut GameCtx) {
            use DynamicEntity::*;
            use StaticEntity::*;
            match (self, other) {
                (Pacman(pacman), Pivot(pivot)) => pacman.on_collision(pivot, ctx),
                (Pacman(pacman), Food(food)) => pacman.on_collision(food, ctx),
                (Pacman(pacman), Special(special)) => pacman.on_collision(special, ctx),
                (Ghost(ghost), Pivot(pivot)) => ghost.on_collision(pivot, ctx),
                _ => {}
            }
        }
    }

    impl Collision<DynamicEntity, GameCtx> for DynamicEntity {
        fn on_collision(&mut self, other: &DynamicEntity, ctx: &mut GameCtx) {
            use DynamicEntity::*;
            match (self, other) {
                (Pacman(pacman), Ghost(ghost)) => pacman.on_collision(ghost, ctx),
                (Ghost(ghost), Pacman(pacman)) => ghost.on_collision(pacman, ctx),
                _ => {}
            }
        }
    }
}
pub use dynamic_entities::*;
