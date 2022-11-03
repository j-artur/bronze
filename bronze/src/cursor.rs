use sfml::{window::Cursor as SfmlCursor, SfBox};

use crate::graphics::Image;

pub struct Cursor {
    cursor: SfBox<SfmlCursor>,
}

impl Cursor {
    pub fn from_image(path: &str) -> Option<Self> {
        Image::from_file(path)
            .and_then(|image| unsafe {
                SfmlCursor::from_pixels(image.pixel_data(), image.size(), image.size() / 2)
            })
            .map(|cursor| Cursor { cursor })
            .or_else(|| {
                eprintln!("Failed to load cursor from \"{}\"", path);
                None
            })
    }

    pub fn cursor(&self) -> &SfmlCursor {
        &self.cursor
    }
}
