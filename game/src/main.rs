use std::time::Duration;

use ball::Ball;
use block::Block;
use bronze::{
    cursor::Cursor,
    engine::Engine,
    game::Game,
    graphics::Sprite,
    icon::Icon,
    input::{InputManager, Key},
    resources::ResourcePool,
    scene::{Collision, Entity, Scene},
    sfml::{
        graphics::Color,
        system::{Vector2, Vector2f},
    },
    shape::{BBox, ShapeRef},
    window::{Canvas, FPSConfig, Window, WindowConfig},
};

use debugger::Debugger;
use player::Player;
use resources::*;

mod ball;
mod block;
mod debugger;
mod player;
mod resources;

pub const WINDOW_WIDTH: u32 = 960;
pub const WINDOW_HEIGHT: u32 = 540;

pub struct GameContext {
    pub player_top: Vector2f,
}

pub enum StaticEntity<'r> {
    Player(Player<'r>),
    Block(Block<'r>),
}

impl Entity for StaticEntity<'_> {
    type Ctx = GameContext;

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
    fn pre_update(&mut self, _ctx: &GameContext) {
        match self {
            StaticEntity::Player(player) => player.pre_update(_ctx),
            StaticEntity::Block(block) => block.pre_update(_ctx),
        }
    }

    #[inline]
    fn update(&mut self, _ctx: &mut GameContext, _frame_time: Duration) {
        match self {
            StaticEntity::Player(player) => player.update(_ctx, _frame_time),
            StaticEntity::Block(block) => block.update(_ctx, _frame_time),
        }
    }

    #[inline]
    fn post_update(&mut self, _ctx: &GameContext) {
        match self {
            StaticEntity::Player(player) => player.post_update(_ctx),
            StaticEntity::Block(block) => block.post_update(_ctx),
        }
    }

    #[inline]
    fn draw(&self, _ctx: &GameContext, _target: &mut Canvas) {
        match self {
            StaticEntity::Player(player) => player.draw(_ctx, _target),
            StaticEntity::Block(block) => block.draw(_ctx, _target),
        }
    }
}

impl<'r> Collision<Ball<'r>> for StaticEntity<'r> {
    #[inline]
    fn on_collision(&mut self, other: &Ball<'r>, ctx: &mut GameContext) {
        match self {
            StaticEntity::Player(_) => {}
            StaticEntity::Block(block) => block.on_collision(other, ctx),
        }
    }
}

pub struct MyGame<'r> {
    bg: Sprite<'r>,
    debugger: Debugger<'r>,
    scene: Scene<StaticEntity<'r>, Ball<'r>, GameContext>,
    player_top: Vector2f,
    running: bool,
    paused: bool,
}

impl<'r> MyGame<'r> {
    const LINE1: f32 = 50.0;
    const LINE2: f32 = 80.0;
    const LINE3: f32 = 110.0;
    const LINE4: f32 = 140.0;
    const LINE5: f32 = 170.0;

    fn new(resource_pool: &'r ResourcePool<Images, Audios, Fonts>, window: &Window) -> Self {
        let mut scene = Scene::new();

        let background = resource_pool.get_image(Images::Background);
        let bg = Sprite::new(background);

        let debug_font = resource_pool.get_font(Fonts::Debug);
        let debugger = Debugger::new(true, debug_font, 10);

        let player = resource_pool.get_image(Images::Player);
        let player = Player::new(player);

        let player_top = Vector2::new(
            player.bbox().left() + player.bbox().width() / 2.0,
            player.bbox().top(),
        );

        let ball = resource_pool.get_image(Images::Ball);
        let ball = Ball::new(ball, &player);

        scene.add_static(player.into());
        scene.add_dynamic(ball);

        let tile1 = resource_pool.get_image(Images::Tile1);
        let tile2 = resource_pool.get_image(Images::Tile2);
        let tile3 = resource_pool.get_image(Images::Tile3);
        let tile4 = resource_pool.get_image(Images::Tile4);
        let tile5 = resource_pool.get_image(Images::Tile5);

        scene.add_static(Block::new(tile1, window.center_x() - 350.0, Self::LINE1).into());
        scene.add_static(Block::new(tile1, window.center_x() - 270.0, Self::LINE1).into());
        scene.add_static(Block::new(tile1, window.center_x() - 190.0, Self::LINE1).into());
        scene.add_static(Block::new(tile1, window.center_x() - 110.0, Self::LINE1).into());
        scene.add_static(Block::new(tile1, window.center_x() - 30.0, Self::LINE1).into());
        scene.add_static(Block::new(tile1, window.center_x() + 50.0, Self::LINE1).into());
        scene.add_static(Block::new(tile1, window.center_x() + 130.0, Self::LINE1).into());
        scene.add_static(Block::new(tile1, window.center_x() + 210.0, Self::LINE1).into());
        scene.add_static(Block::new(tile1, window.center_x() + 290.0, Self::LINE1).into());

        scene.add_static(Block::new(tile2, window.center_x() - 350.0, Self::LINE2).into());
        scene.add_static(Block::new(tile2, window.center_x() - 270.0, Self::LINE2).into());
        scene.add_static(Block::new(tile2, window.center_x() - 190.0, Self::LINE2).into());
        scene.add_static(Block::new(tile2, window.center_x() - 110.0, Self::LINE2).into());
        scene.add_static(Block::new(tile2, window.center_x() - 30.0, Self::LINE2).into());
        scene.add_static(Block::new(tile2, window.center_x() + 50.0, Self::LINE2).into());
        scene.add_static(Block::new(tile2, window.center_x() + 130.0, Self::LINE2).into());
        scene.add_static(Block::new(tile2, window.center_x() + 210.0, Self::LINE2).into());
        scene.add_static(Block::new(tile2, window.center_x() + 290.0, Self::LINE2).into());

        scene.add_static(Block::new(tile3, window.center_x() - 350.0, Self::LINE3).into());
        scene.add_static(Block::new(tile3, window.center_x() - 270.0, Self::LINE3).into());
        scene.add_static(Block::new(tile3, window.center_x() - 190.0, Self::LINE3).into());
        scene.add_static(Block::new(tile3, window.center_x() - 110.0, Self::LINE3).into());
        scene.add_static(Block::new(tile3, window.center_x() - 30.0, Self::LINE3).into());
        scene.add_static(Block::new(tile3, window.center_x() + 50.0, Self::LINE3).into());
        scene.add_static(Block::new(tile3, window.center_x() + 130.0, Self::LINE3).into());
        scene.add_static(Block::new(tile3, window.center_x() + 210.0, Self::LINE3).into());
        scene.add_static(Block::new(tile3, window.center_x() + 290.0, Self::LINE3).into());

        scene.add_static(Block::new(tile4, window.center_x() - 350.0, Self::LINE4).into());
        scene.add_static(Block::new(tile4, window.center_x() - 270.0, Self::LINE4).into());
        scene.add_static(Block::new(tile4, window.center_x() - 190.0, Self::LINE4).into());
        scene.add_static(Block::new(tile4, window.center_x() - 110.0, Self::LINE4).into());
        scene.add_static(Block::new(tile4, window.center_x() - 30.0, Self::LINE4).into());
        scene.add_static(Block::new(tile4, window.center_x() + 50.0, Self::LINE4).into());
        scene.add_static(Block::new(tile4, window.center_x() + 130.0, Self::LINE4).into());
        scene.add_static(Block::new(tile4, window.center_x() + 210.0, Self::LINE4).into());
        scene.add_static(Block::new(tile4, window.center_x() + 290.0, Self::LINE4).into());

        scene.add_static(Block::new(tile5, window.center_x() - 350.0, Self::LINE5).into());
        scene.add_static(Block::new(tile5, window.center_x() - 270.0, Self::LINE5).into());
        scene.add_static(Block::new(tile5, window.center_x() - 190.0, Self::LINE5).into());
        scene.add_static(Block::new(tile5, window.center_x() - 110.0, Self::LINE5).into());
        scene.add_static(Block::new(tile5, window.center_x() - 30.0, Self::LINE5).into());
        scene.add_static(Block::new(tile5, window.center_x() + 50.0, Self::LINE5).into());
        scene.add_static(Block::new(tile5, window.center_x() + 130.0, Self::LINE5).into());
        scene.add_static(Block::new(tile5, window.center_x() + 210.0, Self::LINE5).into());
        scene.add_static(Block::new(tile5, window.center_x() + 290.0, Self::LINE5).into());

        MyGame {
            bg,
            debugger,
            scene,
            player_top,
            running: true,
            paused: false,
        }
    }
}

impl Game for MyGame<'_> {
    #[inline]
    fn is_running(&self) -> bool {
        self.running
    }

    #[inline]
    fn input(&mut self, input: &InputManager) {
        if input.key_press(Key::P) {
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
            let ctx = GameContext {
                player_top: self.player_top,
            };
            self.scene.pre_update(&ctx);
            self.debugger.pre_update(&ctx);
        }
    }

    #[inline]
    fn update(&mut self, _engine: &mut Engine, delta: Duration) {
        if !self.paused {
            let mut ctx = GameContext {
                player_top: self.player_top,
            };
            self.scene.update(&mut ctx, delta);
            self.debugger.update(&mut ctx, delta);
            self.player_top = ctx.player_top;
        }
    }

    #[inline]
    fn post_update(&mut self, _engine: &Engine) {
        if !self.paused {
            let mut ctx = GameContext {
                player_top: self.player_top,
            };
            self.scene.collisions(&mut ctx);
            self.player_top = ctx.player_top;
            self.scene.post_update(&ctx);
            self.debugger.post_update(&ctx);
        }
    }

    #[inline]
    fn draw(&self, target: &mut Canvas) {
        self.bg.draw(target);
        let ctx = GameContext {
            player_top: self.player_top,
        };
        self.scene.draw(&ctx, target);
    }
}

fn main() {
    let resource_pool = ResourcePool::new(load_image, load_audio, load_font);

    let win_config = WindowConfig {
        title: "My Game".to_string(),
        icon: Some(Icon::from_image(resource_pool.get_image(Images::Icon))),
        cursor: Cursor::from_image(resource_pool.get_image(Images::Cursor)),
        bg_color: Color::BLACK,
        show_cursor: true,
        fps_config: FPSConfig::Unlimited,
        mode: (960, 540).into(),
    };

    let mut engine = Engine::new(Window::new(win_config));

    let game = MyGame::new(&resource_pool, engine.window());
    engine.run(game);
}
