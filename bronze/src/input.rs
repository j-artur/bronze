use sfml::{system::Vector2i, window::Event};

pub use sfml::window::{mouse::Button, Key};

pub struct Keyboard {
    pub keys: [bool; 101],
    keys_ctrl: [bool; 101],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [false; 101],
            keys_ctrl: [false; 101],
        }
    }

    #[inline]
    pub fn propagate(&mut self, event: &Event) {
        match event {
            Event::KeyPressed { code, .. } if code != &Key::Unknown => {
                self.keys[*code as usize] = true
            }
            Event::KeyReleased { code, .. } if code != &Key::Unknown => {
                self.keys[*code as usize] = false
            }
            _ => {}
        }
    }

    #[inline]
    pub fn key_down(&self, key: Key) -> bool {
        self.keys[key as usize]
    }

    #[inline]
    pub fn key_up(&self, key: Key) -> bool {
        !self.keys[key as usize]
    }

    #[inline]
    pub fn key_pressed(&self, key: Key) -> bool {
        self.keys[key as usize] && !self.keys_ctrl[key as usize]
    }

    #[inline]
    pub fn key_released(&self, key: Key) -> bool {
        !self.keys[key as usize] && self.keys_ctrl[key as usize]
    }
}

pub struct Mouse {
    x: i32,
    y: i32,
    buttons: [bool; 5],
    buttons_ctrl: [bool; 5],
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            x: 0,
            y: 0,
            buttons: [false; 5],
            buttons_ctrl: [false; 5],
        }
    }

    #[inline]
    pub fn propagate(&mut self, event: &Event) {
        match event {
            Event::MouseButtonPressed { button, .. } => self.buttons[*button as usize] = true,
            Event::MouseButtonReleased { button, .. } => self.buttons[*button as usize] = false,
            Event::MouseMoved { x, y } => {
                self.x = *x;
                self.y = *y;
            }
            _ => {}
        }
    }

    #[inline]
    pub fn x(&self) -> i32 {
        self.x
    }

    #[inline]
    pub fn y(&self) -> i32 {
        self.y
    }

    #[inline]
    pub fn position(&self) -> Vector2i {
        Vector2i::new(self.x, self.y)
    }

    #[inline]
    pub fn button_down(&self, button: Button) -> bool {
        self.buttons[button as usize]
    }

    #[inline]
    pub fn button_up(&self, button: Button) -> bool {
        !self.buttons[button as usize]
    }

    #[inline]
    pub fn button_pressed(&self, button: Button) -> bool {
        self.buttons[button as usize] && !self.buttons_ctrl[button as usize]
    }

    #[inline]
    pub fn button_released(&self, button: Button) -> bool {
        !self.buttons[button as usize] && self.buttons_ctrl[button as usize]
    }
}

pub struct InputManager {
    pub keyboard: Keyboard,
    pub mouse: Mouse,
}

impl InputManager {
    #[inline]
    pub fn new() -> InputManager {
        InputManager {
            keyboard: Keyboard::new(),
            mouse: Mouse::new(),
        }
    }

    #[inline]
    pub fn propagate(&mut self, event: &Event) {
        self.keyboard.propagate(event);
        self.mouse.propagate(event);
    }

    #[inline]
    pub fn update(&mut self) {
        self.keyboard.keys_ctrl = self.keyboard.keys;
        self.mouse.buttons_ctrl = self.mouse.buttons;
    }

    #[inline]
    pub fn key_down(&self, key: Key) -> bool {
        self.keyboard.key_down(key)
    }

    #[inline]
    pub fn key_up(&self, key: Key) -> bool {
        self.keyboard.key_up(key)
    }

    #[inline]
    pub fn key_pressed(&self, key: Key) -> bool {
        self.keyboard.key_pressed(key)
    }

    #[inline]
    pub fn key_released(&self, key: Key) -> bool {
        self.keyboard.key_released(key)
    }

    #[inline]
    pub fn button_down(&self, button: Button) -> bool {
        self.mouse.button_down(button)
    }

    #[inline]
    pub fn button_up(&self, button: Button) -> bool {
        self.mouse.button_up(button)
    }

    #[inline]
    pub fn button_pressed(&self, button: Button) -> bool {
        self.mouse.button_pressed(button)
    }

    #[inline]
    pub fn button_released(&self, button: Button) -> bool {
        self.mouse.button_released(button)
    }
}
