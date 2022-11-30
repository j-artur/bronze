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
    use std::{collections::HashMap, fmt::Debug, hash::Hash};

    use once_cell::unsync::OnceCell;
    use strum::IntoEnumIterator;

    use super::*;

    pub trait Key: IntoEnumIterator + Debug + Eq + Hash {}
    impl<T> Key for T where T: IntoEnumIterator + Debug + Eq + Hash {}

    pub struct ResourcePool<I: Key, A: Key, F: Key> {
        images: HashMap<I, OnceCell<Image>>,
        audios: HashMap<A, OnceCell<Audio>>,
        fonts: HashMap<F, OnceCell<Font>>,
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

        pub fn get_image(&self, id: I) -> &Image {
            self.images
                .get(&id)
                .unwrap()
                .get_or_init(|| (self.load_image)(&id))
        }

        pub fn get_audio(&self, id: A) -> &Audio {
            self.audios
                .get(&id)
                .unwrap()
                .get_or_init(|| (self.load_audio)(&id))
        }

        pub fn get_font(&self, id: F) -> &Font {
            self.fonts
                .get(&id)
                .unwrap()
                .get_or_init(|| (self.load_font)(&id))
        }
    }
}
pub use resource_pool::*;
