use crate::graphics::Image;

pub struct Icon {
    image: Image,
}

impl Icon {
    pub fn from_image(path: &str) -> Option<Self> {
        Image::from_file(path)
            .map(|image| Icon { image })
            .or_else(|| {
                eprintln!("Failed to load icon from \"{}\"", path);
                None
            })
    }

    pub fn width(&self) -> u32 {
        self.image.size().x
    }

    pub fn height(&self) -> u32 {
        self.image.size().y
    }

    pub fn pixels(&self) -> &[u8] {
        self.image.pixel_data()
    }
}
