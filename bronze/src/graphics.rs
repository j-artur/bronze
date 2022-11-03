use sfml::{
    graphics::{Color, Drawable, Sprite as SfmlSprite, Texture, Transformable},
    system::{Vector2, Vector2f},
};

use crate::resources::Image;

pub trait Canvas {
    fn draw(&mut self, drawable: &dyn Drawable);
}

pub struct Sprite<'a> {
    sfml_sprite: SfmlSprite<'a>,
}

impl<'a> Sprite<'a> {
    pub fn new(image: &'a Image) -> Self {
        Sprite {
            sfml_sprite: SfmlSprite::with_texture(image.texture()),
        }
    }

    pub fn texture(&self) -> &Texture {
        self.sfml_sprite.texture().unwrap()
    }

    pub fn position(&self) -> Vector2f {
        self.sfml_sprite.position()
    }

    pub fn set_position(&mut self, position: Vector2f) {
        self.sfml_sprite.set_position(position);
    }

    pub fn rotation(&self) -> f32 {
        self.sfml_sprite.rotation()
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.sfml_sprite.set_rotation(rotation);
    }

    pub fn scale(&self) -> Vector2f {
        self.sfml_sprite.get_scale()
    }

    pub fn scale_by(&mut self, scale: Vector2f) {
        self.sfml_sprite.scale(scale);
    }

    pub fn set_scale(&mut self, scale: Vector2f) {
        self.sfml_sprite.set_scale(scale);
    }

    pub fn color(&self) -> Color {
        self.sfml_sprite.color()
    }

    pub fn set_color(&mut self, color: Color) {
        self.sfml_sprite.set_color(color);
    }

    pub fn flip(&mut self, Vector2 { x, y }: Vector2<bool>) {
        self.sfml_sprite.set_scale(Vector2f {
            x: if x { -1.0 } else { 1.0 },
            y: if y { -1.0 } else { 1.0 },
        });
    }

    pub fn draw(&self, target: &mut dyn Canvas) {
        target.draw(&self.sfml_sprite);
    }
}
