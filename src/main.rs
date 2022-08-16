pub mod engine;

use engine::{
    engine::*,
    game::Game,
    resources::*,
    window::Window,
    window::{coords, Coords},
};
use winapi::um::wingdi::RGB;
use winapi::um::winuser::*;

struct P {
    pos: Coords,
    speed: Coords,
}

pub struct WinGame<'a> {
    window: &'a Window,
    size_text: String,
    mode_text: String,
    mouse_text: String,
    p1: P,
    p2: P,
}

impl<'a> WinGame<'a> {
    fn new(window: &'a Window) -> WinGame<'a> {
        WinGame {
            window,
            size_text: String::new(),
            mode_text: String::new(),
            mouse_text: String::new(),
            p1: P {
                pos: coords!(0, 0),
                speed: coords!(15, 11),
            },
            p2: P {
                pos: coords!(0, 0),
                speed: coords!(9, 13),
            },
        }
    }
}

impl<'a> Game for WinGame<'a> {
    fn init(&mut self) {
        self.size_text = format!("Size: {} x {}", self.window.size().x, self.window.size().y);
        self.mode_text = format!("Mode: {:?}", self.window.mode());
    }

    fn update(&mut self) {
        if self.window.key_down(VK_ESCAPE as u8) {
            self.window.close();
        }

        let Coords { x, y } = self.window.mouse();
        self.mouse_text = format!("Mouse: {} x {}", x, y);

        if self.p1.pos.x < 0 || self.p1.pos.x > self.window.width() {
            self.p1.speed.x = -self.p1.speed.x;
        }
        if self.p1.pos.y < 0 || self.p1.pos.y > self.window.height() {
            self.p1.speed.y = -self.p1.speed.y;
        }
        if self.p2.pos.x < 0 || self.p2.pos.x > self.window.width() {
            self.p2.speed.x = -self.p2.speed.x;
        }
        if self.p2.pos.y < 0 || self.p2.pos.y > self.window.height() {
            self.p2.speed.y = -self.p2.speed.y;
        }

        self.p1.pos.x += self.p1.speed.x;
        self.p1.pos.y += self.p1.speed.y;
        self.p2.pos.x += self.p2.speed.x;
        self.p2.pos.y += self.p2.speed.y;
    }

    fn render(&mut self) {
        self.window
            .print("Window Game Demo", coords!(10, 10), RGB(0, 0, 0));
        self.window
            .print(&self.size_text, coords!(10, 50), RGB(0, 0, 0));
        self.window
            .print(&self.mode_text, coords!(10, 70), RGB(0, 0, 0));
        self.window
            .print(&self.mouse_text, coords!(10, 90), RGB(0, 0, 0));

        self.window.line(self.p1.pos, self.p2.pos);
    }

    fn finalize(self) {}
}

#[allow(unused)]
fn main() {
    let mut window = Window::new("Window Game Demo");
    window.set_icon(IDI_ICON);
    window.set_cursor(IDC_CURSOR);
    // window.set_mode(Windowed);
    // window.set_size(coords!(960, 540));
    window.set_bg(RGB(240, 240, 160));
    window.create();

    let mut engine = Engine::new(&window);

    engine.start(WinGame::new(&window));
}
