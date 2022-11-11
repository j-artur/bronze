use bronze::resources::{Audio, Font, Image};
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum Images {
    Icon,
    Cursor,
    Background,
    Player,
    Ball,
    Tile1,
    Tile2,
    Tile3,
    Tile4,
    Tile5,
}

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum Audios {}

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum Fonts {
    Debug,
}

pub fn load_image(id: &Images) -> Image {
    use Images::*;
    if let Some(image) = match id {
        Icon => Image::new("assets/images/icon.png"),
        Cursor => Image::new("assets/images/cursor.png"),
        Background => Image::new("assets/images/bg.jpg"),
        Player => Image::new("assets/images/player.png"),
        Ball => Image::new("assets/images/ball.png"),
        Tile1 => Image::new("assets/images/tile1.png"),
        Tile2 => Image::new("assets/images/tile2.png"),
        Tile3 => Image::new("assets/images/tile3.png"),
        Tile4 => Image::new("assets/images/tile4.png"),
        Tile5 => Image::new("assets/images/tile5.png"),
    } {
        println!("Loaded image {:?}", id);
        image
    } else {
        panic!("Failed to load image {:?}", id)
    }
}

pub fn load_audio(_id: &Audios) -> Audio {
    panic!("Audio not implemented");
}

pub fn load_font(id: &Fonts) -> Font {
    use Fonts::*;
    if let Some(font) = match id {
        Debug => Font::new("assets/fonts/JetBrainsMono[wght].ttf"),
    } {
        println!("Loaded font {:?}", id);
        font
    } else {
        panic!("Failed to load font {:?}", id)
    }
}
