use sfml::window::Event;

use super::{game::Game, input::InputManager, timer::Timer, window::Window};

pub struct Engine {
    window: Window,
    input: InputManager,
    timer: Timer,
}

impl Engine {
    pub fn new(window: Window) -> Engine {
        Engine {
            window,
            input: InputManager::new(),
            timer: Timer::new(),
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn input(&self) -> &InputManager {
        &self.input
    }

    pub fn run<G: Game>(&mut self, mut game: G) {
        self.timer.start();

        while self.window.is_open() && game.is_running() {
            while let Some(event) = self.window.poll_event() {
                match event {
                    Event::Closed => self.window.close(),
                    _ => {}
                }
                self.input.event(&event);
            }

            game.input(&self.input);

            let frame_time = self.timer.reset();

            game.pre_update(&self);
            game.update(self, frame_time);
            game.post_update(&self);

            self.input.update();

            self.window.clear();
            game.draw(&mut self.window);
            self.window.display();
        }
    }
}
