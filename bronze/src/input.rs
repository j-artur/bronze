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

    pub fn is_key_down(&self, key: Key) -> bool {
        self.keys[key as usize]
    }

    pub fn is_key_up(&self, key: Key) -> bool {
        !self.keys[key as usize]
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.keys[key as usize] && !self.keys_ctrl[key as usize]
    }

    pub fn is_key_released(&self, key: Key) -> bool {
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

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn position(&self) -> Vector2i {
        Vector2i::new(self.x, self.y)
    }

    pub fn is_button_down(&self, button: Button) -> bool {
        self.buttons[button as usize]
    }

    pub fn is_button_up(&self, button: Button) -> bool {
        !self.buttons[button as usize]
    }

    pub fn is_button_pressed(&self, button: Button) -> bool {
        self.buttons[button as usize] && !self.buttons_ctrl[button as usize]
    }

    pub fn is_button_released(&self, button: Button) -> bool {
        !self.buttons[button as usize] && self.buttons_ctrl[button as usize]
    }
}

pub struct InputManager {
    pub keyboard: Keyboard,
    pub mouse: Mouse,
}

impl InputManager {
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

    pub fn key_down(&self, key: Key) -> bool {
        self.keyboard.is_key_down(key)
    }

    pub fn key_up(&self, key: Key) -> bool {
        self.keyboard.is_key_up(key)
    }

    pub fn key_press(&self, key: Key) -> bool {
        self.keyboard.is_key_pressed(key)
    }

    pub fn key_release(&self, key: Key) -> bool {
        self.keyboard.is_key_released(key)
    }

    pub fn button_down(&self, button: Button) -> bool {
        self.mouse.is_button_down(button)
    }

    pub fn button_up(&self, button: Button) -> bool {
        self.mouse.is_button_up(button)
    }

    pub fn button_press(&self, button: Button) -> bool {
        self.mouse.is_button_pressed(button)
    }

    pub fn button_release(&self, button: Button) -> bool {
        self.mouse.is_button_released(button)
    }
}
