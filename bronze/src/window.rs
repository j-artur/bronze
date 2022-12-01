use sfml::{
    graphics::{Color, Drawable, RenderTarget, RenderWindow},
    system::Vector2u,
    window::{Event, Style, VideoMode},
};

use crate::resources::{Cursor, Icon};

pub enum FPSConfig {
    VSync,
    Unlimited,
    Limited(u32),
}

pub enum WindowMode {
    Fullscreen,
    Windowed { width: u32, height: u32 },
}

impl From<Vector2u> for WindowMode {
    fn from(Vector2u { x, y }: Vector2u) -> Self {
        WindowMode::Windowed {
            width: x,
            height: y,
        }
    }
}

impl From<(u32, u32)> for WindowMode {
    fn from((width, height): (u32, u32)) -> Self {
        WindowMode::Windowed { width, height }
    }
}

pub struct WindowConfig {
    pub title: String,
    pub icon: Option<Icon>,
    pub cursor: Option<Cursor>,
    pub bg_color: Color,
    pub show_cursor: bool,
    pub fps_config: FPSConfig,
    pub mode: WindowMode,
}

impl Default for WindowConfig {
    fn default() -> Self {
        WindowConfig {
            title: String::new(),
            bg_color: Color::BLACK,
            show_cursor: true,
            fps_config: FPSConfig::VSync,
            mode: WindowMode::Fullscreen,
            icon: None,
            cursor: None,
        }
    }
}

pub struct Canvas<'w> {
    target: &'w mut RenderWindow,
}

impl Canvas<'_> {
    pub fn draw<D: Drawable>(&mut self, drawable: &D) {
        self.target.draw(drawable)
    }
}

pub struct Window {
    sfml_window: RenderWindow,
    config: WindowConfig,
}

impl Window {
    pub fn new(config: WindowConfig) -> Self {
        let (sfml_mode, sfml_style) = match config.mode {
            WindowMode::Fullscreen => (VideoMode::desktop_mode(), Style::NONE),
            WindowMode::Windowed { width, height } => (
                VideoMode::new(width, height, VideoMode::desktop_mode().bits_per_pixel),
                Style::CLOSE,
            ),
        };

        let mut sfml_window =
            RenderWindow::new(sfml_mode, &config.title, sfml_style, &Default::default());

        if let Some(icon) = &config.icon {
            unsafe {
                sfml_window.set_icon(icon.size().x, icon.size().y, icon.pixels());
            }
        }

        if let Some(cursor) = &config.cursor {
            unsafe {
                sfml_window.set_mouse_cursor(cursor.cursor());
            }
        }

        sfml_window.set_mouse_cursor_visible(config.show_cursor);

        sfml_window.set_vertical_sync_enabled(matches!(config.fps_config, FPSConfig::VSync));
        sfml_window.set_framerate_limit(match config.fps_config {
            FPSConfig::Limited(limit) => limit,
            _ => 0,
        });

        Window {
            sfml_window,
            config,
        }
    }

    pub fn show_cursor(&mut self, show: bool) {
        self.config.show_cursor = show;
        self.sfml_window.set_mouse_cursor_visible(show);
    }

    pub fn is_open(&self) -> bool {
        self.sfml_window.is_open()
    }

    pub fn set_title<S: Into<String>>(&mut self, title: S) {
        self.sfml_window.set_title(&title.into())
    }

    pub fn poll_event(&mut self) -> Option<Event> {
        self.sfml_window.poll_event()
    }

    pub fn clear(&mut self) {
        self.sfml_window.clear(self.config.bg_color)
    }

    pub fn display(&mut self) {
        self.sfml_window.display()
    }

    pub fn close(&mut self) {
        self.sfml_window.close()
    }

    pub fn width(&self) -> u32 {
        self.sfml_window.size().x
    }

    pub fn height(&self) -> u32 {
        self.sfml_window.size().y
    }

    pub fn center_x(&self) -> f32 {
        self.width() as f32 / 2.0
    }

    pub fn center_y(&self) -> f32 {
        self.height() as f32 / 2.0
    }

    pub fn canvas(&mut self) -> Canvas {
        Canvas {
            target: &mut self.sfml_window,
        }
    }
}
