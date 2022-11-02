use sfml::window::{mouse::Button, Event, Key};

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
    pub fn event(&mut self, event: &Event) {
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

    pub fn key_down(&self, key: Key) -> bool {
        self.keys[key as usize]
    }

    pub fn key_up(&self, key: Key) -> bool {
        !self.keys[key as usize]
    }

    pub fn key_press(&self, key: Key) -> bool {
        self.keys[key as usize] && !self.keys_ctrl[key as usize]
    }

    pub fn key_release(&self, key: Key) -> bool {
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
    pub fn event(&mut self, event: &Event) {
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

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn button_down(&self, button: Button) -> bool {
        self.buttons[button as usize]
    }

    pub fn button_up(&self, button: Button) -> bool {
        !self.buttons[button as usize]
    }

    pub fn button_press(&self, button: Button) -> bool {
        self.buttons[button as usize] && !self.buttons_ctrl[button as usize]
    }

    pub fn button_release(&self, button: Button) -> bool {
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
    pub fn event(&mut self, event: &Event) {
        self.keyboard.event(event);
        self.mouse.event(event);
    }

    #[inline]
    pub fn update(&mut self) {
        self.keyboard.keys_ctrl = self.keyboard.keys;
        self.mouse.buttons_ctrl = self.mouse.buttons;
    }

    pub fn key_down(&self, key: Key) -> bool {
        self.keyboard.key_down(key)
    }

    pub fn key_up(&self, key: Key) -> bool {
        self.keyboard.key_up(key)
    }

    pub fn key_press(&self, key: Key) -> bool {
        self.keyboard.key_press(key)
    }

    pub fn key_release(&self, key: Key) -> bool {
        self.keyboard.key_release(key)
    }

    pub fn button_down(&self, button: Button) -> bool {
        self.mouse.button_down(button)
    }

    pub fn button_up(&self, button: Button) -> bool {
        self.mouse.button_up(button)
    }

    pub fn button_press(&self, button: Button) -> bool {
        self.mouse.button_press(button)
    }

    pub fn button_release(&self, button: Button) -> bool {
        self.mouse.button_release(button)
    }
}
