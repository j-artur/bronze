pub mod engine;

use engine::{engine::*, game::*, resources::*, window::*};
use winapi::um::wingdi::RGB;
use winapi::um::winuser::VK_ESCAPE;

struct P {
    pos: Point,
    speed: Size,
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
                pos: (0, 0),
                speed: (15, 11),
            },
            p2: P {
                pos: (0, 0),
                speed: (9, 13),
            },
        }
    }
}

impl<'a> Game for WinGame<'a> {
    fn init(&mut self) {
        let (w, h) = self.window.size();
        self.size_text = format!("Size: {} x {}", w, h);
        self.mode_text = format!("Mode: {:?}", self.window.mode());
    }

    fn update(&mut self) {
        if self.window.key_down(VK_ESCAPE as u8) {
            self.window.close();
        }

        let (x, y) = self.window.mouse();
        self.mouse_text = format!("Mouse: {} x {}", x, y);

        if self.p1.pos.0 < 0 || self.p1.pos.0 > self.window.width() {
            self.p1.speed.0 = -self.p1.speed.0;
        }
        if self.p1.pos.1 < 0 || self.p1.pos.1 > self.window.height() {
            self.p1.speed.1 = -self.p1.speed.1;
        }
        if self.p2.pos.0 < 0 || self.p2.pos.0 > self.window.width() {
            self.p2.speed.0 = -self.p2.speed.0;
        }
        if self.p2.pos.1 < 0 || self.p2.pos.1 > self.window.height() {
            self.p2.speed.1 = -self.p2.speed.1;
        }

        self.p1.pos.0 += self.p1.speed.0;
        self.p1.pos.1 += self.p1.speed.1;
        self.p2.pos.0 += self.p2.speed.0;
        self.p2.pos.1 += self.p2.speed.1;
    }

    fn render(&mut self) {
        self.window
            .print("Window Game Demo", (10, 10), RGB(0, 0, 0));
        self.window.print(&self.size_text, (10, 50), RGB(0, 0, 0));
        self.window.print(&self.mode_text, (10, 70), RGB(0, 0, 0));
        self.window.print(&self.mouse_text, (10, 90), RGB(0, 0, 0));
        self.window.line(self.p1.pos, self.p2.pos);
    }

    fn finalize(self) {}
}

use engine::window::WindowMode::*;

fn main() {
    let mut window = Window::new("Window Game Demo");
    window.set_icon(IDI_ICON);
    window.set_cursor(IDC_CURSOR);
    window.set_mode(Windowed);
    window.set_size((960, 540));
    window.set_bg(RGB(240, 240, 160));
    window.create();

    let mut engine = Engine::new(&window);

    engine.start(WinGame::new(&window));
}
