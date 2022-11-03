use sfml::{window::Cursor as SfmlCursor, SfBox};

use crate::resources::Image;

pub struct Cursor<'r> {
    cursor: SfBox<SfmlCursor>,
    phantom: std::marker::PhantomData<&'r ()>,
}

impl<'r> Cursor<'r> {
    pub fn from_image(image: &'r Image) -> Option<Self> {
        unsafe { SfmlCursor::from_pixels(image.pixels(), image.size(), image.size() / 2) }.map(
            |cursor| Cursor {
                cursor,
                phantom: std::marker::PhantomData,
            },
        )
    }

    pub fn cursor(&self) -> &SfmlCursor {
        &self.cursor
    }
}
