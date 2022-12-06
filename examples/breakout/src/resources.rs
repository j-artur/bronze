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

    let path = match id {
        Icon => "examples/breakout/assets/images/icon.png",
        Cursor => "examples/breakout/assets/images/cursor.png",
        Background => "examples/breakout/assets/images/bg.jpg",
        Player => "examples/breakout/assets/images/player.png",
        Ball => "examples/breakout/assets/images/ball.png",
        Tile1 => "examples/breakout/assets/images/tile1.png",
        Tile2 => "examples/breakout/assets/images/tile2.png",
        Tile3 => "examples/breakout/assets/images/tile3.png",
        Tile4 => "examples/breakout/assets/images/tile4.png",
        Tile5 => "examples/breakout/assets/images/tile5.png",
    };

    if let Some(image) = Image::new(path) {
        println!("Loaded image {:?} from {}", id, path);
        image
    } else {
        panic!("Failed to load image {:?} from {}", id, path)
    }
}

pub fn load_audio(_id: &Audios) -> Audio {
    panic!("Audio not implemented");
}

pub fn load_font(id: &Fonts) -> Font {
    use Fonts::*;

    let path = match id {
        Debug => "examples/breakout/assets/fonts/JetBrainsMono[wght].ttf",
    };

    if let Some(font) = Font::new(path) {
        println!("Loaded font {:?}", id);
        font
    } else {
        panic!("Failed to load font {:?}", id)
    }
}
