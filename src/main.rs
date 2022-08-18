pub mod engine;

use engine::{engine::*, game::*, graphics::*, resources::*, window::*};

pub struct D3DGame {}

impl D3DGame {
    fn new() -> Self {
        D3DGame {}
    }
}

impl Game for D3DGame {
    fn init(&mut self, _: &Window) {}

    fn update(&mut self, _: &Window) {}

    fn render(&mut self, _: &Window) {}

    fn finalize(&mut self) {}
}

use engine::window::WindowMode::*;

fn main() {
    let mut engine = Engine::new();

    engine.window().set_title("Window Game Demo");
    engine.window().set_icon(IDI_ICON);
    engine.window().set_cursor(IDC_CURSOR);
    engine.window().set_mode(Windowed);
    engine.window().set_size((960, 540));
    engine.window().set_bg(rgb!(40, 40, 40));

    engine.start(Box::new(D3DGame::new()));
}
