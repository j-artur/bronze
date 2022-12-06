use std::rc::Rc;

use sfml::SfBox;

mod image {
    use sfml::{
        graphics::{Image as SfmlImage, IntRect, Texture},
        system::Vector2,
    };

    use super::*;

    pub struct Image {
        image: SfmlImage,
        texture: SfBox<Texture>,
    }

    impl Image {
        pub fn new(path: &str) -> Option<Self> {
            SfmlImage::from_file(path).and_then(|image| {
                let Vector2 { x, y } = image.size();
                let mut texture = Texture::new()?;
                texture
                    .load_from_image(&image, IntRect::new(0, 0, x as i32, y as i32))
                    .ok()?;

                Some(Image { image, texture })
            })
        }

        pub fn size(&self) -> Vector2<u32> {
            self.image.size()
        }

        pub fn pixels(&self) -> &[u8] {
            self.image.pixel_data()
        }

        pub fn texture(&self) -> &Texture {
            &self.texture
        }
    }
}
pub use image::*;

mod audio {
    pub struct Audio;

    impl Audio {
        pub fn new(_path: &str) -> Option<Self> {
            unimplemented!("Audio is not implemented yet");
        }
    }
}
pub use audio::*;

mod font {
    use sfml::graphics::Font as SfmlFont;

    use super::*;

    pub struct Font {
        sfml_font: SfBox<SfmlFont>,
    }

    impl Font {
        pub fn new(path: &str) -> Option<Self> {
            SfmlFont::from_file(path).map(|sfml_font| Font { sfml_font })
        }

        pub fn sfml_font(&self) -> &SfmlFont {
            &self.sfml_font
        }
    }
}
pub use font::*;

mod resource_pool {
    use std::{collections::HashMap, hash::Hash, rc::Rc};

    use once_cell::unsync::OnceCell;
    use strum::IntoEnumIterator;

    use super::*;

    pub trait Key: IntoEnumIterator + Eq + Hash {}
    impl<T> Key for T where T: IntoEnumIterator + Eq + Hash {}

    pub struct ResourcePool<I: Key, A: Key, F: Key> {
        images: HashMap<I, OnceCell<Rc<Image>>>,
        audios: HashMap<A, OnceCell<Rc<Audio>>>,
        fonts: HashMap<F, OnceCell<Rc<Font>>>,
        load_image: fn(&I) -> Image,
        load_audio: fn(&A) -> Audio,
        load_font: fn(&F) -> Font,
    }

    impl<I: Key, A: Key, F: Key> ResourcePool<I, A, F> {
        pub fn new(
            load_image: fn(&I) -> Image,
            load_audio: fn(&A) -> Audio,
            load_font: fn(&F) -> Font,
        ) -> Self {
            let mut images = HashMap::new();
            let mut audios = HashMap::new();
            let mut fonts = HashMap::new();

            for id in I::iter() {
                images.insert(id, OnceCell::new());
            }

            for id in A::iter() {
                audios.insert(id, OnceCell::new());
            }

            for id in F::iter() {
                fonts.insert(id, OnceCell::new());
            }

            ResourcePool {
                images,
                audios,
                fonts,
                load_image,
                load_audio,
                load_font,
            }
        }

        pub fn get_image(&self, id: I) -> Rc<Image> {
            let cell = self.images.get(&id).unwrap();
            Rc::clone(cell.get_or_init(|| Rc::new((self.load_image)(&id))))
        }

        pub fn get_audio(&self, id: A) -> Rc<Audio> {
            let cell = self.audios.get(&id).unwrap();
            Rc::clone(cell.get_or_init(|| Rc::new((self.load_audio)(&id))))
        }

        pub fn get_font(&self, id: F) -> Rc<Font> {
            let cell = self.fonts.get(&id).unwrap();
            Rc::clone(cell.get_or_init(|| Rc::new((self.load_font)(&id))))
        }

        pub fn try_clear(&mut self) {
            for (_, cell) in &mut self.images {
                let cell: &mut OnceCell<Rc<Image>> = cell;

                if let Some(rc) = cell.get() {
                    if Rc::strong_count(rc) == 1 {
                        cell.take();
                    }
                }
            }

            for (_, cell) in &mut self.audios {
                let cell: &mut OnceCell<Rc<Audio>> = cell;

                if let Some(rc) = cell.get() {
                    if Rc::strong_count(rc) == 1 {
                        cell.take();
                    }
                }
            }

            for (_, cell) in &mut self.fonts {
                let cell: &mut OnceCell<Rc<Font>> = cell;

                if let Some(rc) = cell.get() {
                    if Rc::strong_count(rc) == 1 {
                        cell.take();
                    }
                }
            }
        }

        pub fn full_clear(&mut self) {
            for (_, cell) in &mut self.images {
                let cell: &mut OnceCell<Rc<Image>> = cell;
                cell.take();
            }

            for (_, cell) in &mut self.audios {
                let cell: &mut OnceCell<Rc<Audio>> = cell;
                cell.take();
            }

            for (_, cell) in &mut self.fonts {
                let cell: &mut OnceCell<Rc<Font>> = cell;
                cell.take();
            }
        }
    }
}
pub use resource_pool::*;

mod cursor {
    use sfml::window::Cursor as SfmlCursor;

    use super::*;

    pub struct Cursor {
        _image: Rc<Image>,
        cursor: SfBox<SfmlCursor>,
    }

    impl Cursor {
        pub fn from_image(image: &Rc<Image>) -> Option<Self> {
            unsafe { SfmlCursor::from_pixels(image.pixels(), image.size(), image.size() / 2) }.map(
                |cursor| Cursor {
                    _image: Rc::clone(image),
                    cursor,
                },
            )
        }

        pub fn cursor(&self) -> &SfmlCursor {
            &self.cursor
        }
    }
}
pub use cursor::*;

mod icon {
    use sfml::system::Vector2u;

    use super::*;

    pub struct Icon {
        image: Rc<Image>,
    }

    impl Icon {
        pub fn from_image(image: &Rc<Image>) -> Self {
            Icon {
                image: Rc::clone(image),
            }
        }

        pub fn size(&self) -> Vector2u {
            self.image.size()
        }

        pub fn pixels(&self) -> &[u8] {
            self.image.pixels()
        }
    }
}
pub use icon::*;
