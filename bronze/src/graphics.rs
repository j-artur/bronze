use sfml::{
    graphics::{Color, Sprite as SfmlSprite, Texture, Transformable},
    system::{Vector2, Vector2f},
};

use crate::{resources::Image, window::Canvas};

pub struct Sprite<'r> {
    sfml_sprite: SfmlSprite<'r>,
}

impl<'r> Sprite<'r> {
    #[inline]
    pub fn new(image: &'r Image) -> Self {
        Sprite {
            sfml_sprite: SfmlSprite::with_texture(image.texture()),
        }
    }
    #[inline]
    pub fn texture(&self) -> &Texture {
        self.sfml_sprite.texture().unwrap()
    }
    #[inline]
    pub fn color(&self) -> Color {
        self.sfml_sprite.color()
    }
    #[inline]
    pub fn set_color(&mut self, color: Color) {
        self.sfml_sprite.set_color(color);
    }
    #[inline]
    pub fn flip(&mut self, Vector2 { x, y }: Vector2<bool>) {
        self.sfml_sprite.set_scale(Vector2f {
            x: if x { -1.0 } else { 1.0 },
            y: if y { -1.0 } else { 1.0 },
        });
    }
    #[inline]
    pub fn draw(&self, target: &mut Canvas) {
        target.draw(&self.sfml_sprite);
    }

    #[inline]
    pub fn position(&self) -> Vector2f {
        self.sfml_sprite.position()
    }
    #[inline]
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.sfml_sprite.set_position(Vector2f { x, y });
    }
    #[inline]
    pub fn move_by(&mut self, x: f32, y: f32) {
        self.sfml_sprite.move_(Vector2f { x, y });
    }

    #[inline]
    pub fn width(&self) -> f32 {
        self.texture().size().x as f32
    }
    #[inline]
    pub fn height(&self) -> f32 {
        self.texture().size().y as f32
    }
    #[inline]
    pub fn size(&self) -> Vector2f {
        let Vector2 { x, y } = self.texture().size();
        Vector2f::new(x as f32, y as f32)
    }

    #[inline]
    pub fn rotation(&self) -> f32 {
        self.sfml_sprite.rotation()
    }
    #[inline]
    pub fn set_rotation(&mut self, rotation: f32) {
        self.sfml_sprite.set_rotation(rotation);
    }
    #[inline]
    pub fn rotate_by(&mut self, rotation: f32) {
        self.sfml_sprite.rotate(rotation);
    }

    #[inline]
    pub fn scale(&self) -> Vector2f {
        self.sfml_sprite.get_scale()
    }
    #[inline]
    pub fn set_scale(&mut self, x: f32, y: f32) {
        self.sfml_sprite.set_scale(Vector2f { x, y });
    }
    #[inline]
    pub fn scale_by(&mut self, x: f32, y: f32) {
        self.sfml_sprite.scale(Vector2f { x, y });
    }
}
