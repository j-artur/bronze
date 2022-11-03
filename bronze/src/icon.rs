use sfml::system::Vector2u;

use crate::resources::Image;

pub struct Icon<'r> {
    image: &'r Image,
}

impl<'r> Icon<'r> {
    pub fn from_image(image: &'r Image) -> Self {
        Icon { image }
    }

    pub fn size(&self) -> Vector2u {
        self.image.size()
    }

    pub fn pixels(&self) -> &[u8] {
        self.image.pixels()
    }
}
