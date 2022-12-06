use bronze::{
    graphics::Sprite,
    resources::{Audio, Font, Image},
};
use strum_macros::EnumIter;

use crate::{game::Dir, Pool};

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

    let path = match id {
        Icon => todo!("create icon"),
        Cursor => todo!("create cursor"),
        Level1Bg => "examples/pacman/assets/images/level1_bg.jpg",
        Level2Bg => "examples/pacman/assets/images/level2_bg.jpg",
        TitleScreenBg => "examples/pacman/assets/images/title_screen.jpg",
        PacmanUp => "examples/pacman/assets/images/pacman_up.png",
        PacmanDown => "examples/pacman/assets/images/pacman_down.png",
        PacmanLeft => "examples/pacman/assets/images/pacman_left.png",
        PacmanRight => "examples/pacman/assets/images/pacman_right.png",
        GhostBlueUp => "examples/pacman/assets/images/ghost_blue_up.png",
        GhostBlueDown => "examples/pacman/assets/images/ghost_blue_down.png",
        GhostBlueLeft => "examples/pacman/assets/images/ghost_blue_left.png",
        GhostBlueRight => "examples/pacman/assets/images/ghost_blue_right.png",
        GhostOrangeUp => "examples/pacman/assets/images/ghost_orange_up.png",
        GhostOrangeDown => "examples/pacman/assets/images/ghost_orange_down.png",
        GhostOrangeLeft => "examples/pacman/assets/images/ghost_orange_left.png",
        GhostOrangeRight => "examples/pacman/assets/images/ghost_orange_right.png",
        GhostPinkUp => "examples/pacman/assets/images/ghost_pink_up.png",
        GhostPinkDown => "examples/pacman/assets/images/ghost_pink_down.png",
        GhostPinkLeft => "examples/pacman/assets/images/ghost_pink_left.png",
        GhostPinkRight => "examples/pacman/assets/images/ghost_pink_right.png",
        GhostRedUp => "examples/pacman/assets/images/ghost_red_up.png",
        GhostRedDown => "examples/pacman/assets/images/ghost_red_down.png",
        GhostRedLeft => "examples/pacman/assets/images/ghost_red_left.png",
        GhostRedRight => "examples/pacman/assets/images/ghost_red_right.png",
        GhostShock => "examples/pacman/assets/images/ghost_shock.png",
        Food => "examples/pacman/assets/images/food.png",
        Special => "examples/pacman/assets/images/special.png",
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

pub fn load_font(_id: &Fonts) -> Font {
    panic!("Font not implemented");
}

#[derive(Clone)]
pub struct PacmanSprites {
    up: Sprite,
    down: Sprite,
    left: Sprite,
    right: Sprite,
}

impl PacmanSprites {
    pub fn new(resource_pool: &Pool) -> PacmanSprites {
        PacmanSprites {
            up: Sprite::new(&resource_pool.get_image(Images::PacmanUp)),
            down: Sprite::new(&resource_pool.get_image(Images::PacmanDown)),
            left: Sprite::new(&resource_pool.get_image(Images::PacmanLeft)),
            right: Sprite::new(&resource_pool.get_image(Images::PacmanRight)),
        }
    }

    pub fn sprite(&self, dir: &Dir) -> &Sprite {
        match dir {
            Dir::Up => &self.up,
            Dir::Down => &self.down,
            Dir::Left => &self.left,
            Dir::Right => &self.right,
        }
    }
}

pub enum GhostColor {
    Blue,
    Orange,
    Pink,
    Red,
}

#[derive(Clone)]
pub struct GhostSprites {
    blue_up: Sprite,
    blue_down: Sprite,
    blue_left: Sprite,
    blue_right: Sprite,
    orange_up: Sprite,
    orange_down: Sprite,
    orange_left: Sprite,
    orange_right: Sprite,
    pink_up: Sprite,
    pink_down: Sprite,
    pink_left: Sprite,
    pink_right: Sprite,
    red_up: Sprite,
    red_down: Sprite,
    red_left: Sprite,
    red_right: Sprite,
}

impl GhostSprites {
    pub fn new(resource_pool: &Pool) -> GhostSprites {
        GhostSprites {
            blue_up: Sprite::new(&resource_pool.get_image(Images::GhostBlueUp)),
            blue_down: Sprite::new(&resource_pool.get_image(Images::GhostBlueDown)),
            blue_left: Sprite::new(&resource_pool.get_image(Images::GhostBlueLeft)),
            blue_right: Sprite::new(&resource_pool.get_image(Images::GhostBlueRight)),
            orange_up: Sprite::new(&resource_pool.get_image(Images::GhostOrangeUp)),
            orange_down: Sprite::new(&resource_pool.get_image(Images::GhostOrangeDown)),
            orange_left: Sprite::new(&resource_pool.get_image(Images::GhostOrangeLeft)),
            orange_right: Sprite::new(&resource_pool.get_image(Images::GhostOrangeRight)),
            pink_up: Sprite::new(&resource_pool.get_image(Images::GhostPinkUp)),
            pink_down: Sprite::new(&resource_pool.get_image(Images::GhostPinkDown)),
            pink_left: Sprite::new(&resource_pool.get_image(Images::GhostPinkLeft)),
            pink_right: Sprite::new(&resource_pool.get_image(Images::GhostPinkRight)),
            red_up: Sprite::new(&resource_pool.get_image(Images::GhostRedUp)),
            red_down: Sprite::new(&resource_pool.get_image(Images::GhostRedDown)),
            red_left: Sprite::new(&resource_pool.get_image(Images::GhostRedLeft)),
            red_right: Sprite::new(&resource_pool.get_image(Images::GhostRedRight)),
        }
    }

    pub fn sprite(&self, color: &GhostColor, dir: &Dir) -> &Sprite {
        match (color, dir) {
            (GhostColor::Blue, Dir::Up) => &self.blue_up,
            (GhostColor::Blue, Dir::Down) => &self.blue_down,
            (GhostColor::Blue, Dir::Left) => &self.blue_left,
            (GhostColor::Blue, Dir::Right) => &self.blue_right,
            (GhostColor::Orange, Dir::Up) => &self.orange_up,
            (GhostColor::Orange, Dir::Down) => &self.orange_down,
            (GhostColor::Orange, Dir::Left) => &self.orange_left,
            (GhostColor::Orange, Dir::Right) => &self.orange_right,
            (GhostColor::Pink, Dir::Up) => &self.pink_up,
            (GhostColor::Pink, Dir::Down) => &self.pink_down,
            (GhostColor::Pink, Dir::Left) => &self.pink_left,
            (GhostColor::Pink, Dir::Right) => &self.pink_right,
            (GhostColor::Red, Dir::Up) => &self.red_up,
            (GhostColor::Red, Dir::Down) => &self.red_down,
            (GhostColor::Red, Dir::Left) => &self.red_left,
            (GhostColor::Red, Dir::Right) => &self.red_right,
        }
    }
}
