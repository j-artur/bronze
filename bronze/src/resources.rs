use std::{collections::HashMap, fmt::Display, hash::Hash};

use once_cell::unsync::OnceCell;
use sfml::{
    graphics::{Font as SfmlFont, Image as SfmlImage, IntRect, Texture},
    system::Vector2,
    SfBox,
};
use strum::IntoEnumIterator;

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

pub trait Key: IntoEnumIterator + Display + Eq + Hash {}
impl<T> Key for T where T: IntoEnumIterator + Display + Eq + Hash {}
pub type Load<K, V> = fn(&K) -> Option<V>;

pub struct Lazy<K, V> {
    cell: OnceCell<V>,
    load: Load<K, V>,
}

impl<K: Key, V> Lazy<K, V> {
    pub fn new(load: Load<K, V>) -> Self {
        Lazy {
            cell: OnceCell::new(),
            load,
        }
    }

    pub fn get(&self, key: &K) -> &V {
        self.cell.get_or_init(|| match (self.load)(key) {
            Some(resource) => {
                println!("Loaded resource \"{}\"", key);
                resource
            }
            None => panic!("Failed to load resource \"{}\"", key),
        })
    }
}

pub struct ResourcePool<I: Key, A: Key, F: Key> {
    images: HashMap<I, Lazy<I, Image>>,
    audios: HashMap<A, Lazy<A, Audio>>,
    fonts: HashMap<F, Lazy<F, Font>>,
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
            images.insert(id, Lazy::new(load_image));
        }

        for id in A::iter() {
            audios.insert(id, Lazy::new(load_audio));
        }

        for id in F::iter() {
            fonts.insert(id, Lazy::new(load_font));
        }

        ResourcePool {
            images,
            audios,
            fonts,
        }
    }

    pub fn get_image(&self, id: I) -> &Image {
        self.images.get(&id).unwrap().get(&id)
    }

    pub fn get_audio(&self, id: A) -> &Audio {
        self.audios.get(&id).unwrap().get(&id)
    }

    pub fn get_font(&self, id: F) -> &Font {
        self.fonts.get(&id).unwrap().get(&id)
    }
}
