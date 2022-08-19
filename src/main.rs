pub mod engine;

use engine::context::Context;
use engine::window::{Window, WindowMode::*};
use engine::{engine::*, game::*, resources::*, *};
use winapi::um::winuser::VK_ESCAPE;

pub struct FPSCounter {}

impl FPSCounter {
    fn new() -> Self {
        FPSCounter {}
    }
}

impl Game for FPSCounter {
    fn init(&mut self, _: &mut Window) {}

    fn update(&mut self, ctx: Context) {
        if ctx.window.key_down(VK_ESCAPE as u8) {
            ctx.window.close();
        }
    }

    fn render(&mut self, _: Context) {}

    fn finalize(&mut self) {}
}

fn main() {
    let mut engine = Engine::new();

    engine.window().set_title("FPSCounter");
    engine.window().set_icon(IDI_ICON);
    engine.window().set_cursor(IDC_CURSOR);
    engine.window().set_mode(Windowed);
    engine.window().set_size((960, 540));
    engine.window().set_bg(rgb!(40, 40, 40));

    engine.start(Box::new(FPSCounter::new()));
}
