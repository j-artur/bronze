use super::{game::Game, window::Window};
use std::{ptr::null_mut, thread::sleep, time::Duration};
use winapi::shared::windef::*;
use winapi::um::{wingdi::CreateSolidBrush, winuser::*};

pub struct Engine<'a> {
    pub window: &'a Window,
}

impl<'a> Engine<'a> {
    pub fn new(window: &'a Window) -> Engine<'a> {
        Engine { window }
    }

    pub fn start<G: Game>(&mut self, level: G) {
        self.game_loop(level)
    }

    pub fn game_loop<G: Game>(&mut self, mut game: G) {
        let mut msg = MSG {
            hwnd: null_mut(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT { x: 0, y: 0 },
        };

        let mut rect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };

        unsafe {
            let hdc = GetDC(self.window.hwnd());
            GetClientRect(self.window.hwnd(), &mut rect);

            game.init();

            while msg.message != WM_QUIT {
                if PeekMessageW(&mut msg, null_mut(), 0, 0, PM_REMOVE) != 0 {
                    TranslateMessage(&mut msg);
                    DispatchMessageW(&mut msg);
                } else {
                    game.update();

                    FillRect(hdc, &mut rect, CreateSolidBrush(self.window.bg()));

                    game.render();

                    sleep(Duration::from_millis(1000 / 60));
                }
            }

            game.finalize();

            ReleaseDC(self.window.hwnd(), hdc)
        };
    }
}
