use sfml::{
    graphics::{Color, Sprite as SfmlSprite, Texture, Transformable},
    system::{Vector2, Vector2f, Vector2u},
};

use crate::{resources::Image, window::Canvas};

pub struct Sprite<'r> {
    texture: &'r Texture,
}

pub trait Position {
    fn get(&self) -> Vector2f;
}

impl<V: Into<Vector2f> + Clone> Position for V {
    fn get(&self) -> Vector2f {
        self.clone().into()
    }
}

pub trait Rotation {
    fn get(&self) -> f32;
}

impl Rotation for f32 {
    fn get(&self) -> f32 {
        *self
    }
}

pub trait Scale {
    fn get(&self) -> Vector2f;
}

impl Scale for Vector2f {
    fn get(&self) -> Vector2f {
        *self
    }
}

impl Scale for Vector2u {
    fn get(&self) -> Vector2f {
        Vector2f::new(self.x as f32, self.y as f32)
    }
}

impl Scale for f32 {
    fn get(&self) -> Vector2f {
        Vector2f::new(*self, *self)
    }
}

impl<'r> Sprite<'r> {
    #[inline]
    pub fn new(image: &'r Image) -> Self {
        Sprite {
            texture: image.texture(),
        }
    }
    #[inline]
    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    #[inline]
    pub fn draw<P: Position, R: Rotation, S: Scale>(
        &self,
        target: &mut Canvas,
        position: P,
        rotation: R,
        scale: S,
    ) {
        let mut sprite_data = SfmlSprite::with_texture(self.texture);
        sprite_data.set_position(position.get());
        sprite_data.set_rotation(rotation.get());
        sprite_data.set_scale(scale.get());
        target.draw(&sprite_data);
    }

    #[inline]
    pub fn draw_colorized(
        &self,
        target: &mut Canvas,
        position: Vector2f,
        rotation: f32,
        scale: Vector2f,
        color: Color,
    ) {
        let mut sprite_data = SfmlSprite::with_texture(self.texture);
        sprite_data.set_position(position);
        sprite_data.set_rotation(rotation);
        sprite_data.set_scale(scale);
        sprite_data.set_color(color);
        target.draw(&sprite_data);
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
}
