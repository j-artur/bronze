use std::{fs::read_to_string, time::Duration};

use bronze::{
    engine::Engine, game::Game as BronzeGame, graphics::Sprite, input::InputManager, input::Key,
    scene::Scene, shape::Point, window::Canvas,
};
use rand::seq::SliceRandom;

use crate::{
    entities::{DynamicEntity, Food, Ghost, Pacman, Pivot, Special, StaticEntity},
    resources::*,
    Pool,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn opposite(&self) -> Self {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }

    pub fn random_dirs() -> [Self; 4] {
        let mut dirs = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];
        dirs.shuffle(&mut rand::thread_rng());
        dirs
    }
}

#[derive(Clone)]
pub struct GameCtx {
    pub score: u32,
    pub pacman_alive: bool,
    pub food_count: u32,
    pub pacman_center: Point,
    pub pacman_sprites: PacmanSprites,
    pub ghost_sprites: GhostSprites,
}

impl GameCtx {
    pub fn new(resource_pool: &Pool) -> GameCtx {
        GameCtx {
            score: 0,
            pacman_alive: true,
            pacman_center: Point::new(0.0, 0.0),
            pacman_sprites: PacmanSprites::new(resource_pool),
            ghost_sprites: GhostSprites::new(resource_pool),
            food_count: 0,
        }
    }
}

mod level {
    use super::*;

    pub enum Levels {
        TitleScreen,
        Level1,
        Level2,
    }

    pub trait Level {
        fn is_running(&self) -> bool;

        fn input(&mut self, input: &InputManager) {
            let _ = input;
        }

        fn pre_update(&mut self, ctx: &GameCtx) {
            let _ = ctx;
        }

        fn update(&mut self, ctx: &mut GameCtx, frame_time: Duration) {
            let _ = (ctx, frame_time);
        }

        fn post_update(&mut self, ctx: &GameCtx) {
            let _ = ctx;
        }

        fn draw(&self, ctx: &GameCtx, target: &mut Canvas) {
            let _ = (ctx, target);
        }

        fn next_level(&self) -> Option<Levels>;
    }

    pub fn char_to_bool(char: char) -> Option<bool> {
        match char {
            'T' | 't' => Some(true),
            'F' | 'f' => Some(false),
            _ => None,
        }
    }

    pub fn read_pivots(path: &str) -> Option<Vec<(bool, bool, bool, bool, u32, u32)>> {
        let content = read_to_string(path).ok()?;
        content
            .lines()
            .filter(|line| !line.starts_with('#') && !line.is_empty())
            .map(|line| {
                let mut parts = line.split_whitespace();

                let up = char_to_bool(parts.next()?.parse::<char>().ok()?)?;
                let down = char_to_bool(parts.next()?.parse::<char>().ok()?)?;
                let left = char_to_bool(parts.next()?.parse::<char>().ok()?)?;
                let right = char_to_bool(parts.next()?.parse::<char>().ok()?)?;
                let x = parts.next()?.parse::<u32>().ok()?;
                let y = parts.next()?.parse::<u32>().ok()?;

                Some((up, down, left, right, x, y))
            })
            .collect()
    }

    pub fn read_positions(path: &str) -> Option<Vec<(u32, u32)>> {
        let str = read_to_string(path).ok()?;
        let lines = str
            .lines()
            .filter(|line| !line.starts_with('#') && !line.is_empty());

        let mut positions = Vec::new();

        for line in lines {
            let mut parts = line.split_whitespace();

            let x = parts.next()?.parse::<u32>().ok()?;
            let y = parts.next()?.parse::<u32>().ok()?;

            positions.push((x, y));
        }

        Some(positions)
    }
}
pub use level::*;

mod title_screen {
    use super::*;

    pub struct TitleScreen {
        bg: Sprite,
        running: bool,
        next: bool,
    }

    impl TitleScreen {
        pub fn new(resource_pool: &Pool) -> Box<Self> {
            let bg = Sprite::new(&resource_pool.get_image(Images::TitleScreenBg));

            Box::new(TitleScreen {
                bg,
                running: true,
                next: false,
            })
        }
    }

    impl Level for TitleScreen {
        fn is_running(&self) -> bool {
            self.running
        }

        fn input(&mut self, input: &InputManager) {
            if input.key_down(Key::Escape) {
                self.running = false;
            }
            if input.key_pressed(Key::Enter) {
                self.next = true;
            }
        }

        fn update(&mut self, _ctx: &mut GameCtx, _frame_time: Duration) {}

        fn draw(&self, _ctx: &GameCtx, target: &mut Canvas) {
            self.bg.draw(target, (0.0, 0.0));
        }

        fn next_level(&self) -> Option<Levels> {
            self.next.then_some(Levels::Level1)
        }
    }
}
pub use title_screen::*;

mod level1 {
    use super::*;

    pub struct Level1 {
        bg: Sprite,
        running: bool,
        next: bool,
        scene: Scene<StaticEntity, DynamicEntity, GameCtx>,
    }

    impl Level1 {
        pub fn new(resource_pool: &Pool, ctx: &mut GameCtx) -> Box<Self> {
            let bg = Sprite::new(&resource_pool.get_image(Images::Level1Bg));

            let mut scene = Scene::new();

            scene.add_dynamic(Pacman::new(480, 450));

            scene.add_dynamic(Ghost::new(405, 360, GhostColor::Blue));
            scene.add_dynamic(Ghost::new(455, 340, GhostColor::Orange));
            scene.add_dynamic(Ghost::new(505, 340, GhostColor::Pink));
            scene.add_dynamic(Ghost::new(555, 360, GhostColor::Red));

            if let Some(pivots) = read_pivots("examples/pacman/assets/levels/level1_pivots.txt") {
                for (up, down, left, right, x, y) in pivots {
                    scene.add_static(Pivot::new(up, down, left, right, x, y));
                }
            }

            let sprite = resource_pool.get_image(Images::Food);

            if let Some(ps) = read_positions("examples/pacman/assets/levels/level1_foods.txt") {
                ctx.food_count = ps.len() as u32;
                for (x, y) in ps {
                    scene.add_static(Food::new(&sprite, x, y));
                }
            }

            let sprite = resource_pool.get_image(Images::Special);

            if let Some(ps) = read_positions("examples/pacman/assets/levels/level1_specials.txt") {
                ctx.food_count += ps.len() as u32;
                for (x, y) in ps {
                    scene.add_static(Special::new(&sprite, x, y));
                }
            }

            Box::new(Level1 {
                bg,
                running: true,
                next: false,
                scene,
            })
        }
    }

    impl Level for Level1 {
        fn is_running(&self) -> bool {
            self.running
        }

        fn input(&mut self, input: &InputManager) {
            if input.key_down(Key::Escape) {
                self.running = false;
            }

            self.scene.input(input);
        }

        fn pre_update(&mut self, ctx: &GameCtx) {
            self.scene.pre_update(ctx);
        }

        fn update(&mut self, ctx: &mut GameCtx, frame_time: Duration) {
            self.scene.update(ctx, frame_time);

            self.scene.collisions(ctx);
        }

        fn post_update(&mut self, ctx: &GameCtx) {
            self.scene.post_update(ctx);
            if ctx.food_count == 0 {
                self.next = true;
            }
        }

        fn draw(&self, ctx: &GameCtx, target: &mut Canvas) {
            self.bg.draw(target, (0.0, 0.0));
            self.scene.draw(ctx, target);
        }

        fn next_level(&self) -> Option<Levels> {
            self.next.then_some(Levels::Level2)
        }
    }
}
pub use level1::*;

mod level2 {
    use super::*;

    pub struct Level2 {
        bg: Sprite,
        running: bool,
        next: bool,
        scene: Scene<StaticEntity, DynamicEntity, GameCtx>,
    }

    impl Level2 {
        pub fn new(resource_pool: &Pool, ctx: &mut GameCtx) -> Box<Self> {
            let bg = Sprite::new(&resource_pool.get_image(Images::Level2Bg));

            let mut scene = Scene::new();

            scene.add_dynamic(Pacman::new(480, 450));

            if let Some(pivots) = read_pivots("examples/pacman/assets/levels/level2_pivots.txt") {
                for (up, down, left, right, x, y) in pivots {
                    scene.add_static(Pivot::new(up, down, left, right, x, y));
                }
            }

            let sprite = resource_pool.get_image(Images::Food);

            if let Some(ps) = read_positions("examples/pacman/assets/levels/level2_foods.txt") {
                ctx.food_count = ps.len() as u32;
                for (x, y) in ps {
                    scene.add_static(Food::new(&sprite, x, y));
                }
            }

            let sprite = resource_pool.get_image(Images::Special);

            if let Some(ps) = read_positions("examples/pacman/assets/levels/level2_specials.txt") {
                ctx.food_count += ps.len() as u32;
                for (x, y) in ps {
                    scene.add_static(Special::new(&sprite, x, y));
                }
            }

            Box::new(Level2 {
                bg,
                running: true,
                next: false,
                scene,
            })
        }
    }

    impl Level for Level2 {
        fn is_running(&self) -> bool {
            self.running
        }

        fn input(&mut self, input: &InputManager) {
            if input.key_down(Key::Escape) {
                self.running = false;
            }

            self.scene.input(input);
        }

        fn update(&mut self, ctx: &mut GameCtx, frame_time: Duration) {
            self.scene.update(ctx, frame_time);

            self.scene.collisions(ctx);
        }

        fn post_update(&mut self, ctx: &GameCtx) {
            self.scene.post_update(ctx);
            if ctx.food_count == 0 {
                self.next = true;
            }
        }

        fn draw(&self, ctx: &GameCtx, target: &mut Canvas) {
            self.bg.draw(target, (0.0, 0.0));
            self.scene.draw(ctx, target);
        }

        fn next_level(&self) -> Option<Levels> {
            self.next.then_some(Levels::TitleScreen)
        }
    }
}
pub use level2::*;

mod game {
    use super::*;

    pub struct Game {
        resource_pool: Pool,
        level: Box<dyn Level>,
        ctx: GameCtx,
    }

    impl Game {
        pub fn new(resource_pool: Pool) -> Game {
            let title_screen = TitleScreen::new(&resource_pool);
            let ctx = GameCtx::new(&resource_pool);

            Game {
                resource_pool,
                level: title_screen,
                ctx,
            }
        }
    }

    impl BronzeGame for Game {
        fn is_running(&self) -> bool {
            self.level.is_running()
        }

        fn input(&mut self, input: &InputManager) {
            self.level.input(input);
        }

        fn pre_update(&mut self, _engine: &Engine) {
            self.level.pre_update(&self.ctx);
        }

        fn update(&mut self, _engine: &mut Engine, frame_time: Duration) {
            self.level.update(&mut self.ctx, frame_time);
        }

        fn post_update(&mut self, _engine: &Engine) {
            self.level.post_update(&self.ctx);

            if let Some(next_level) = self.level.next_level() {
                self.level = match next_level {
                    Levels::TitleScreen => TitleScreen::new(&self.resource_pool),
                    Levels::Level1 => Level1::new(&self.resource_pool, &mut self.ctx),
                    Levels::Level2 => Level2::new(&self.resource_pool, &mut self.ctx),
                };
                self.resource_pool.try_clear();
            }
        }

        fn draw(&self, target: &mut Canvas) {
            self.level.draw(&self.ctx, target);
        }
    }
}
pub use game::*;
