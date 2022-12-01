use std::{fs::read_to_string, time::Duration};

use bronze::{
    engine::Engine, game::Game as BronzeGame, graphics::Sprite, input::InputManager, input::Key,
    scene::Scene, window::Canvas,
};

use crate::{
    entities::{DynamicEntity, GameCtx, Pacman, Pivot, StaticEntity},
    resources::*,
    Pool,
};

pub enum Levels {
    TitleScreen,
    Level1,
    Level2,
    EndScreen,
    GameOver,
}

trait Level: BronzeGame {
    fn next_level(&self) -> Option<Levels>;
}

pub fn read_pivots(path: &str) -> Option<Vec<(bool, bool, bool, bool, u32, u32)>> {
    let content = read_to_string(path).ok()?;
    content
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
        .map(|line| {
            let mut parts = line.split_whitespace();

            let up = parts.next()?.parse::<u8>().ok()? != 0;
            let down = parts.next()?.parse::<u8>().ok()? != 0;
            let left = parts.next()?.parse::<u8>().ok()? != 0;
            let right = parts.next()?.parse::<u8>().ok()? != 0;
            let x = parts.next()?.parse::<u32>().ok()?;
            let y = parts.next()?.parse::<u32>().ok()?;

            Some((up, down, left, right, x, y))
        })
        .collect()
}

pub fn read_foods(path: &str) -> Option<Vec<(u32, u32)>> {
    let str = read_to_string(path).ok()?;
    let lines = str
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty());

    let mut foods = Vec::new();

    for line in lines {
        let mut parts = line.split_whitespace();

        let x = parts.next()?.parse::<u32>().ok()?;
        let y = parts.next()?.parse::<u32>().ok()?;

        foods.push((x, y));
    }

    Some(foods)
}

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

    impl BronzeGame for TitleScreen {
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

        fn update(&mut self, _engine: &mut Engine, _frame_time: Duration) {}

        fn draw(&self, target: &mut Canvas) {
            self.bg.draw(target, (0.0, 0.0));
        }
    }

    impl Level for TitleScreen {
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
        pub fn new(resource_pool: &Pool) -> Box<Self> {
            let bg = Sprite::new(&resource_pool.get_image(Images::Level1Bg));

            let mut scene = Scene::new();

            scene.add_dynamic(Pacman::new(480, 450, resource_pool));

            if let Some(pivots) = read_pivots("examples/pacman/assets/levels/level1_pivots.txt") {
                for (up, down, left, right, x, y) in pivots {
                    scene.add_static(Pivot::new(up, down, left, right, x, y));
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

    impl BronzeGame for Level1 {
        fn is_running(&self) -> bool {
            self.running
        }

        fn input(&mut self, input: &InputManager) {
            if input.key_down(Key::Escape) {
                self.running = false;
            }

            self.scene.input(input);
        }

        fn pre_update(&mut self, _engine: &Engine) {
            self.scene.pre_update(&());
        }

        fn update(&mut self, _engine: &mut Engine, frame_time: Duration) {
            self.scene.update(&mut (), frame_time);

            self.scene.collisions(&mut ());
        }

        fn post_update(&mut self, _engine: &Engine) {
            self.scene.post_update(&());
        }

        fn draw(&self, target: &mut Canvas) {
            self.bg.draw(target, (0.0, 0.0));
            self.scene.draw(&(), target);
        }
    }

    impl Level for Level1 {
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
        pub fn new(resource_pool: &Pool) -> Box<Self> {
            let bg = Sprite::new(&resource_pool.get_image(Images::Level2Bg));

            let scene = Scene::new();

            Box::new(Level2 {
                bg,
                running: true,
                next: false,
                scene,
            })
        }
    }

    impl BronzeGame for Level2 {
        fn is_running(&self) -> bool {
            self.running
        }

        fn input(&mut self, input: &InputManager) {
            if input.key_down(Key::Escape) {
                self.running = false;
            }

            self.scene.input(input);
        }

        fn update(&mut self, _engine: &mut Engine, frame_time: Duration) {
            self.scene.update(&mut (), frame_time);

            self.scene.collisions(&mut ());
        }

        fn post_update(&mut self, _engine: &Engine) {
            self.scene.post_update(&());
        }

        fn draw(&self, target: &mut Canvas) {
            self.bg.draw(target, (0.0, 0.0));
            self.scene.draw(&(), target);
        }
    }

    impl Level for Level2 {
        fn next_level(&self) -> Option<Levels> {
            self.next.then_some(Levels::EndScreen)
        }
    }
}
pub use level2::*;

mod game {
    use super::*;

    pub struct Game {
        resource_pool: Pool,
        level: Box<dyn Level>,
    }

    impl Game {
        pub fn new(resource_pool: Pool) -> Game {
            let title_screen = TitleScreen::new(&resource_pool);

            Game {
                resource_pool,
                level: title_screen,
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

        fn pre_update(&mut self, engine: &Engine) {
            self.level.pre_update(engine);
        }

        fn update(&mut self, engine: &mut Engine, frame_time: Duration) {
            self.level.update(engine, frame_time);
        }

        fn post_update(&mut self, engine: &Engine) {
            self.level.post_update(engine);

            if let Some(next_level) = self.level.next_level() {
                self.level = match next_level {
                    Levels::TitleScreen => TitleScreen::new(&self.resource_pool),
                    Levels::Level1 => Level1::new(&self.resource_pool),
                    Levels::Level2 => Level2::new(&self.resource_pool),
                    Levels::GameOver => todo!("game over"),
                    Levels::EndScreen => todo!("end screen"),
                };
            }
        }

        fn draw(&self, target: &mut Canvas) {
            self.level.draw(target);
        }
    }
}
pub use game::*;
