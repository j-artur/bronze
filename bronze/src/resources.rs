use std::{collections::HashMap, fmt::Debug, hash::Hash};

use once_cell::unsync::OnceCell;
use sfml::{
    graphics::{Font as SfmlFont, Image as SfmlImage, IntRect, Texture},
    system::Vector2,
    SfBox,
};
use strum::IntoEnumIterator;

pub struct Image {
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

            Some(Image { texture })
        })
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }
}

pub struct Audio;

impl Audio {
    pub fn new(path: &str) -> Option<Self> {
        let _ = path;
        Some(Audio)
    }
}

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

pub trait Key: IntoEnumIterator + Debug + Copy + Eq + Hash + PartialEq {}

impl<T> Key for T where T: IntoEnumIterator + Debug + Copy + Eq + Hash + PartialEq {}

pub type Load<K, V> = fn(&K) -> Option<V>;

pub struct ResourcePool<I: Key, A: Key, F: Key> {
    images: HashMap<I, OnceCell<Image>>,
    audios: HashMap<A, OnceCell<Audio>>,
    fonts: HashMap<F, OnceCell<Font>>,
    load_image: Load<I, Image>,
    load_audio: Load<A, Audio>,
    load_font: Load<F, Font>,
}

impl<I: Key, A: Key, F: Key> ResourcePool<I, A, F> {
    pub fn new(
        load_image: Load<I, Image>,
        load_audio: Load<A, Audio>,
        load_font: Load<F, Font>,
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
        self.images.get(&id).unwrap().get_or_init(|| {
            (self.load_image)(&id).expect(&format!("Failed to load image \"{:?}\"", id))
        })
    }

    pub fn get_audio(&self, id: A) -> &Audio {
        self.audios.get(&id).unwrap().get_or_init(|| {
            (self.load_audio)(&id).expect(&format!("Failed to load audio \"{:?}\"", id))
        })
    }

    pub fn get_font(&self, id: F) -> &Font {
        self.fonts.get(&id).unwrap().get_or_init(|| {
            (self.load_font)(&id).expect(&format!("Failed to load font \"{:?}\"", id))
        })
    }
}
