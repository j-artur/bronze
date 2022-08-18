use super::graphics::Graphics;
use super::{game::Game, window::Window};
use std::ffi::OsStr;
use std::os::windows::prelude::OsStrExt;
use std::{ptr::null_mut, thread::sleep, time::Duration};
use winapi::shared::windef::*;
use winapi::um::winuser::*;

pub struct Engine {
    window: Window,
    graphics: Graphics,
}

impl Engine {
    pub fn new() -> Self {
        let window = Window::new();
        let graphics = Graphics::new();
        Engine { window, graphics }
    }

    pub fn window(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn start(&mut self, level: Box<dyn Game>) {
        if !self.window.create() {
            unsafe {
                MessageBoxW(
                    null_mut(),
                    OsStr::new("Failed to create window")
                        .encode_wide()
                        .chain(Some(0).into_iter())
                        .collect::<Vec<u16>>()
                        .as_ptr(),
                    OsStr::new("Error")
                        .encode_wide()
                        .chain(Some(0).into_iter())
                        .collect::<Vec<u16>>()
                        .as_ptr(),
                    MB_OK | MB_ICONERROR,
                );
            }

            return;
        }

        if !self.graphics.initialize(&self.window) {
            unsafe {
                MessageBoxW(
                    self.window.hwnd(),
                    OsStr::new("Failed to create graphics")
                        .encode_wide()
                        .chain(Some(0).into_iter())
                        .collect::<Vec<u16>>()
                        .as_ptr(),
                    OsStr::new("Error")
                        .encode_wide()
                        .chain(Some(0).into_iter())
                        .collect::<Vec<u16>>()
                        .as_ptr(),
                    MB_OK | MB_ICONERROR,
                );
            }

            return;
        }

        self.game_loop(level)
    }

    pub fn game_loop(&mut self, mut game: Box<dyn Game>) {
        game.init(&self.window);

        let mut msg = MSG {
            hwnd: null_mut(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT { x: 0, y: 0 },
        };

        while msg.message != WM_QUIT {
            if unsafe { PeekMessageW(&mut msg, null_mut(), 0, 0, PM_REMOVE) } != 0 {
                unsafe {
                    TranslateMessage(&mut msg);
                    DispatchMessageW(&mut msg);
                }
            } else {
                game.update(&self.window);

                self.graphics.clear();

                game.render(&self.window);

                self.graphics.present();

                sleep(Duration::from_millis(1000 / 60));
            }
        }

        game.finalize();
    }
}
