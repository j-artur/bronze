use std::fmt::Display;

use bronze::resources::{Audio, Font, Image};
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum Images {
    Icon,
    Cursor,
    Background,
}

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum Audios {}

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum Fonts {
    Debug,
}

pub fn load_image(id: &Images) -> Option<Image> {
    use Images::*;
    match id {
        Icon => Image::new("assets/images/icon.png"),
        Cursor => Image::new("assets/images/cursor.png"),
        Background => Image::new("assets/images/bg.jpg"),
    }
}

pub fn load_audio(_id: &Audios) -> Option<Audio> {
    None
}

pub fn load_font(id: &Fonts) -> Option<Font> {
    use Fonts::*;
    match id {
        Debug => Font::new("assets/fonts/JetBrainsMono[wght].ttf"),
    }
}

// Debugging

impl Display for Images {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Images::{:?}", self)
    }
}

impl Display for Audios {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Audios::{:?}", self)
    }
}

impl Display for Fonts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fonts::{:?}", self)
    }
}
