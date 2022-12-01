use bronze::resources::{Audio, Font, Image};
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum Images {
    Icon,
    Cursor,
    Level1Bg,
    Level2Bg,
    TitleScreenBg,
    PacmanUp,
    PacmanDown,
    PacmanLeft,
    PacmanRight,
    GhostBlueUp,
    GhostBlueDown,
    GhostBlueLeft,
    GhostBlueRight,
    GhostOrangeUp,
    GhostOrangeDown,
    GhostOrangeLeft,
    GhostOrangeRight,
    GhostPinkUp,
    GhostPinkDown,
    GhostPinkLeft,
    GhostPinkRight,
    GhostRedUp,
    GhostRedDown,
    GhostRedLeft,
    GhostRedRight,
    GhostShock,
    Food,
    Special,
}

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum Audios {}

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum Fonts {}

pub fn load_image(id: &Images) -> Image {
    use Images::*;

    if let Some(image) = match id {
        Icon => todo!("create icon"),
        Cursor => todo!("create cursor"),
        Level1Bg => Image::new("examples/pacman/assets/images/level1_bg.jpg"),
        Level2Bg => Image::new("examples/pacman/assets/images/level2_bg.jpg"),
        TitleScreenBg => Image::new("examples/pacman/assets/images/title_screen.jpg"),
        PacmanUp => Image::new("examples/pacman/assets/images/pacman_up.png"),
        PacmanDown => Image::new("examples/pacman/assets/images/pacman_down.png"),
        PacmanLeft => Image::new("examples/pacman/assets/images/pacman_left.png"),
        PacmanRight => Image::new("examples/pacman/assets/images/pacman_right.png"),
        GhostBlueUp => Image::new("examples/pacman/assets/images/ghost_blue_up.png"),
        GhostBlueDown => Image::new("examples/pacman/assets/images/ghost_blue_down.png"),
        GhostBlueLeft => Image::new("examples/pacman/assets/images/ghost_blue_left.png"),
        GhostBlueRight => Image::new("examples/pacman/assets/images/ghost_blue_right.png"),
        GhostOrangeUp => Image::new("examples/pacman/assets/images/ghost_orange_up.png"),
        GhostOrangeDown => Image::new("examples/pacman/assets/images/ghost_orange_down.png"),
        GhostOrangeLeft => Image::new("examples/pacman/assets/images/ghost_orange_left.png"),
        GhostOrangeRight => Image::new("examples/pacman/assets/images/ghost_orange_right.png"),
        GhostPinkUp => Image::new("examples/pacman/assets/images/ghost_pink_up.png"),
        GhostPinkDown => Image::new("examples/pacman/assets/images/ghost_pink_down.png"),
        GhostPinkLeft => Image::new("examples/pacman/assets/images/ghost_pink_left.png"),
        GhostPinkRight => Image::new("examples/pacman/assets/images/ghost_pink_right.png"),
        GhostRedUp => Image::new("examples/pacman/assets/images/ghost_red_up.png"),
        GhostRedDown => Image::new("examples/pacman/assets/images/ghost_red_down.png"),
        GhostRedLeft => Image::new("examples/pacman/assets/images/ghost_red_left.png"),
        GhostRedRight => Image::new("examples/pacman/assets/images/ghost_red_right.png"),
        GhostShock => Image::new("examples/pacman/assets/images/ghost_shock.png"),
        Food => Image::new("examples/pacman/assets/images/food.png"),
        Special => Image::new("examples/pacman/assets/images/special.png"),
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

pub fn load_font(_id: &Fonts) -> Font {
    panic!("Font not implemented");
}
