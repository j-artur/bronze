use std::ptr::null_mut;
use winapi::um::{timeapi::*, winuser::*};

use super::{
    context::Context, game::Game, graphics::Graphics, timer::Timer, u16str, window::Window,
};

pub struct Engine {
    window: Window,
    graphics: Graphics,
    timer: Timer,
    paused: bool,
    frame_time: f64,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            window: Window::new(),
            graphics: Graphics::new(),
            timer: Timer::new(),
            paused: false,
            frame_time: 0.0,
        }
    }

    pub fn start(&mut self, level: Box<dyn Game>) {
        if !self.window.create() {
            unsafe {
                MessageBoxW(
                    null_mut(),
                    u16str!("Failed to create window"),
                    u16str!("Error"),
                    MB_OK | MB_ICONERROR,
                );
            }

            return;
        }

        if !self.graphics.initialize(&self.window) {
            unsafe {
                MessageBoxW(
                    self.window.handle(),
                    u16str!("Failed to create graphics"),
                    u16str!("Error"),
                    MB_OK | MB_ICONERROR,
                );
            }

            return;
        }

        unsafe { timeBeginPeriod(1) };

        self.game_loop(level);

        unsafe { timeEndPeriod(1) };
    }

    pub fn game_loop(&mut self, mut game: Box<dyn Game>) {
        self.timer.start();

        game.init(&mut self.window);

        unsafe {
            let mut msg: MSG = std::mem::zeroed();

            let mut pressing_pause = true;

            while msg.message != WM_QUIT {
                if PeekMessageW(&mut msg, null_mut(), 0, 0, PM_REMOVE) != 0 {
                    TranslateMessage(&mut msg);
                    DispatchMessageW(&mut msg);
                } else {
                    if pressing_pause {
                        if self.window.key_down(VK_PAUSE as u8) {
                            self.paused = !self.paused;
                            pressing_pause = false;

                            if self.paused {
                                self.timer.stop();
                                if cfg!(debug_assertions) {
                                    let title = format!("{} - PAUSED", self.window.title());
                                    SetWindowTextW(self.window.handle(), u16str!(&title));
                                }
                            } else {
                                self.timer.start();
                                if cfg!(debug_assertions) {
                                    let title = format!("{}", self.window.title());
                                    SetWindowTextW(self.window.handle(), u16str!(&title));
                                }
                            }
                        }
                    } else {
                        if self.window.key_up(VK_PAUSE as u8) {
                            pressing_pause = true;
                        }
                    }

                    if !self.paused {
                        self.frame_time = self.frame_time();

                        game.update(self.context());

                        self.graphics.clear();

                        game.render(self.context());

                        self.graphics.present();
                    } else {
                        game.on_pause();
                    }
                }
            }
        }

        game.finalize();
    }

    pub fn window(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn graphics(&mut self) -> &mut Graphics {
        &mut self.graphics
    }

    pub fn pause(&mut self) {
        self.paused = true;
        self.timer.stop();
    }

    pub fn resume(&mut self) {
        self.paused = false;
        self.timer.start();
    }

    fn frame_time(&mut self) -> f64 {
        let frame_time = self.timer.reset();

        if cfg!(debug_assertions) {
            unsafe {
                static mut TOTAL_TIME: f64 = 0.0;
                static mut FRAME_COUNT: u32 = 0;

                TOTAL_TIME += frame_time;
                FRAME_COUNT += 1;

                if TOTAL_TIME >= 1.0 {
                    let title = format!(
                        "{} - FPS: {} - Frame Time: {:.4}ms",
                        self.window.title(),
                        FRAME_COUNT,
                        frame_time * 1000.0
                    );
                    SetWindowTextW(self.window.handle(), u16str!(&title));

                    FRAME_COUNT = 0;
                    TOTAL_TIME -= 1.0;
                }
            }
        }

        frame_time
    }

    fn context(&mut self) -> Context {
        Context {
            window: &mut self.window,
            game_time: self.frame_time,
        }
    }
}
