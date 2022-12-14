use std::time::Duration;

use bronze::{
    engine::Engine,
    game::Game,
    graphics::{Color, Sprite},
    input::{InputManager, Key},
    resources::{Cursor, Icon, ResourcePool},
    scene::{Collision, Entity, Scene},
    shape::{BBox, ShapeRef},
    system::{Vector2, Vector2f},
    window::{Canvas, FPSConfig, Window, WindowConfig},
};

mod ball;
mod block;
mod debugger;
mod player;
mod resources;

use crate::{ball::Ball, block::Block, debugger::Debugger, player::Player, resources::*};

pub const WINDOW_WIDTH: u32 = 960;
pub const WINDOW_HEIGHT: u32 = 540;

pub struct GameContext {
    pub player_top: Vector2f,
}

pub enum StaticEntity {
    Player(Player),
    Block(Block),
}

impl Entity<GameContext> for StaticEntity {
    #[inline]
    fn bbox(&self) -> ShapeRef {
        match self {
            StaticEntity::Player(player) => player.bbox(),
            StaticEntity::Block(block) => block.bbox(),
        }
    }

    #[inline]
    fn input(&mut self, input: &InputManager) {
        match self {
            StaticEntity::Player(player) => player.input(input),
            StaticEntity::Block(block) => block.input(input),
        }
    }

    #[inline]
    fn pre_update(&mut self, ctx: &GameContext) {
        match self {
            StaticEntity::Player(player) => player.pre_update(ctx),
            StaticEntity::Block(block) => block.pre_update(ctx),
        }
    }

    #[inline]
    fn update(&mut self, ctx: &mut GameContext, frame_time: Duration) {
        match self {
            StaticEntity::Player(player) => player.update(ctx, frame_time),
            StaticEntity::Block(block) => block.update(ctx, frame_time),
        }
    }

    #[inline]
    fn post_update(&mut self, ctx: &GameContext) {
        match self {
            StaticEntity::Player(player) => player.post_update(ctx),
            StaticEntity::Block(block) => block.post_update(ctx),
        }
    }

    #[inline]
    fn draw(&self, ctx: &GameContext, target: &mut Canvas) {
        match self {
            StaticEntity::Player(player) => player.draw(ctx, target),
            StaticEntity::Block(block) => block.draw(ctx, target),
        }
    }
}

impl Collision<Ball, GameContext> for StaticEntity {
    #[inline]
    fn on_collision(&mut self, other: &Ball, ctx: &mut GameContext) {
        match self {
            StaticEntity::Player(_) => {}
            StaticEntity::Block(block) => block.on_collision(other, ctx),
        }
    }
}

pub struct Breakout {
    bg: Sprite,
    debugger: Debugger,
    scene: Scene<StaticEntity, Ball, GameContext>,
    ctx: GameContext,
    running: bool,
    paused: bool,
}

impl Breakout {
    const LINE1: f32 = 50.0;
    const LINE2: f32 = 80.0;
    const LINE3: f32 = 110.0;
    const LINE4: f32 = 140.0;
    const LINE5: f32 = 170.0;

    fn new(resource_pool: &ResourcePool<Images, Audios, Fonts>, window: &Window) -> Self {
        let mut scene = Scene::new();

        let background = resource_pool.get_image(Images::Background);
        let bg = Sprite::new(&background);

        let debug_font = resource_pool.get_font(Fonts::Debug);
        let debugger = Debugger::new(false, &debug_font, 10);

        let player = resource_pool.get_image(Images::Player);
        let player = Player::new(&player);

        let player_top = Vector2::new(
            player.bbox().left() + player.bbox().width() / 2.0,
            player.bbox().top(),
        );

        let ball = resource_pool.get_image(Images::Ball);
        let ball = Ball::new(&ball, &player);

        scene.add_static(player);
        scene.add_dynamic(ball);

        let tile1 = resource_pool.get_image(Images::Tile1);
        let tile2 = resource_pool.get_image(Images::Tile2);
        let tile3 = resource_pool.get_image(Images::Tile3);
        let tile4 = resource_pool.get_image(Images::Tile4);
        let tile5 = resource_pool.get_image(Images::Tile5);

        scene.add_static(Block::new(&tile1, window.center_x() - 350.0, Self::LINE1));
        scene.add_static(Block::new(&tile1, window.center_x() - 270.0, Self::LINE1));
        scene.add_static(Block::new(&tile1, window.center_x() - 190.0, Self::LINE1));
        scene.add_static(Block::new(&tile1, window.center_x() - 110.0, Self::LINE1));
        scene.add_static(Block::new(&tile1, window.center_x() - 30.0, Self::LINE1));
        scene.add_static(Block::new(&tile1, window.center_x() + 50.0, Self::LINE1));
        scene.add_static(Block::new(&tile1, window.center_x() + 130.0, Self::LINE1));
        scene.add_static(Block::new(&tile1, window.center_x() + 210.0, Self::LINE1));
        scene.add_static(Block::new(&tile1, window.center_x() + 290.0, Self::LINE1));

        scene.add_static(Block::new(&tile2, window.center_x() - 350.0, Self::LINE2));
        scene.add_static(Block::new(&tile2, window.center_x() - 270.0, Self::LINE2));
        scene.add_static(Block::new(&tile2, window.center_x() - 190.0, Self::LINE2));
        scene.add_static(Block::new(&tile2, window.center_x() - 110.0, Self::LINE2));
        scene.add_static(Block::new(&tile2, window.center_x() - 30.0, Self::LINE2));
        scene.add_static(Block::new(&tile2, window.center_x() + 50.0, Self::LINE2));
        scene.add_static(Block::new(&tile2, window.center_x() + 130.0, Self::LINE2));
        scene.add_static(Block::new(&tile2, window.center_x() + 210.0, Self::LINE2));
        scene.add_static(Block::new(&tile2, window.center_x() + 290.0, Self::LINE2));

        scene.add_static(Block::new(&tile3, window.center_x() - 350.0, Self::LINE3));
        scene.add_static(Block::new(&tile3, window.center_x() - 270.0, Self::LINE3));
        scene.add_static(Block::new(&tile3, window.center_x() - 190.0, Self::LINE3));
        scene.add_static(Block::new(&tile3, window.center_x() - 110.0, Self::LINE3));
        scene.add_static(Block::new(&tile3, window.center_x() - 30.0, Self::LINE3));
        scene.add_static(Block::new(&tile3, window.center_x() + 50.0, Self::LINE3));
        scene.add_static(Block::new(&tile3, window.center_x() + 130.0, Self::LINE3));
        scene.add_static(Block::new(&tile3, window.center_x() + 210.0, Self::LINE3));
        scene.add_static(Block::new(&tile3, window.center_x() + 290.0, Self::LINE3));

        scene.add_static(Block::new(&tile4, window.center_x() - 350.0, Self::LINE4));
        scene.add_static(Block::new(&tile4, window.center_x() - 270.0, Self::LINE4));
        scene.add_static(Block::new(&tile4, window.center_x() - 190.0, Self::LINE4));
        scene.add_static(Block::new(&tile4, window.center_x() - 110.0, Self::LINE4));
        scene.add_static(Block::new(&tile4, window.center_x() - 30.0, Self::LINE4));
        scene.add_static(Block::new(&tile4, window.center_x() + 50.0, Self::LINE4));
        scene.add_static(Block::new(&tile4, window.center_x() + 130.0, Self::LINE4));
        scene.add_static(Block::new(&tile4, window.center_x() + 210.0, Self::LINE4));
        scene.add_static(Block::new(&tile4, window.center_x() + 290.0, Self::LINE4));

        scene.add_static(Block::new(&tile5, window.center_x() - 350.0, Self::LINE5));
        scene.add_static(Block::new(&tile5, window.center_x() - 270.0, Self::LINE5));
        scene.add_static(Block::new(&tile5, window.center_x() - 190.0, Self::LINE5));
        scene.add_static(Block::new(&tile5, window.center_x() - 110.0, Self::LINE5));
        scene.add_static(Block::new(&tile5, window.center_x() - 30.0, Self::LINE5));
        scene.add_static(Block::new(&tile5, window.center_x() + 50.0, Self::LINE5));
        scene.add_static(Block::new(&tile5, window.center_x() + 130.0, Self::LINE5));
        scene.add_static(Block::new(&tile5, window.center_x() + 210.0, Self::LINE5));
        scene.add_static(Block::new(&tile5, window.center_x() + 290.0, Self::LINE5));

        Breakout {
            bg,
            debugger,
            scene,
            ctx: GameContext { player_top },
            running: true,
            paused: false,
        }
    }
}

impl Game for Breakout {
    #[inline]
    fn is_running(&self) -> bool {
        self.running
    }

    #[inline]
    fn input(&mut self, input: &InputManager) {
        if input.key_pressed(Key::P) {
            self.paused = !self.paused;
        }

        if !self.paused {
            if input.key_down(Key::Escape) {
                self.running = false;
            }

            self.scene.input(input);
            self.debugger.input(input);
        }
    }

    #[inline]
    fn pre_update(&mut self, _engine: &Engine) {
        if !self.paused {
            self.scene.pre_update(&self.ctx);
            self.debugger.pre_update(&self.ctx);
        }
    }

    #[inline]
    fn update(&mut self, _engine: &mut Engine, frame_time: Duration) {
        if !self.paused {
            self.scene.update(&mut self.ctx, frame_time);
            self.debugger.update(&mut self.ctx, frame_time);
            self.scene.collisions(&mut self.ctx);
        }
    }

    #[inline]
    fn post_update(&mut self, _engine: &Engine) {
        if !self.paused {
            self.scene.post_update(&self.ctx);
            self.debugger.post_update(&self.ctx);
        }
    }

    #[inline]
    fn draw(&self, target: &mut Canvas) {
        self.bg.draw(target, (0.0, 0.0));
        self.scene.draw(&self.ctx, target);
        self.debugger.draw(&self.ctx, target);
    }
}

fn main() {
    let resource_pool = ResourcePool::new(load_image, load_audio, load_font);

    let win_config = WindowConfig {
        title: "My Game".to_string(),
        icon: Some(Icon::from_image(&resource_pool.get_image(Images::Icon))),
        cursor: Cursor::from_image(&resource_pool.get_image(Images::Cursor)),
        bg_color: Color::BLACK,
        show_cursor: true,
        fps_config: FPSConfig::Unlimited,
        mode: (960, 540).into(),
    };

    let mut engine = Engine::new(Window::new(win_config));

    engine.run(Breakout::new(&resource_pool, engine.window()));
}
