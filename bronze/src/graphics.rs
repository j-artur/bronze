use std::rc::Rc;

pub use sfml::graphics::Color;
use sfml::{graphics::Transformable, system::Vector2f};

use crate::{shape::Point, window::Canvas};

type Position = Point;
type X = f32;
type Y = f32;
type Rotation = f32;
type Scale = Vector2f;

mod sprite {
    use super::*;

    use sfml::graphics::{Sprite as SfmlSprite, Texture};

    use crate::{
        resources::Image,
        shape::{BBox, Point},
    };

    pub trait DrawArgs {
        fn position(&self) -> Position;
        fn rotation(&self) -> Rotation;
        fn scale(&self) -> Scale;
        fn color(&self) -> Color;
    }

    impl DrawArgs for (Position, Rotation, Scale, Color) {
        fn position(&self) -> Position {
            self.0.clone()
        }

        fn rotation(&self) -> Rotation {
            self.1
        }

        fn scale(&self) -> Scale {
            self.2
        }

        fn color(&self) -> Color {
            self.3
        }
    }

    impl DrawArgs for (Position, Rotation, Scale) {
        fn position(&self) -> Position {
            self.0.clone()
        }

        fn rotation(&self) -> Rotation {
            self.1
        }

        fn scale(&self) -> Scale {
            self.2
        }

        fn color(&self) -> Color {
            Color::WHITE
        }
    }

    impl DrawArgs for (Position, Rotation, Color) {
        fn position(&self) -> Position {
            self.0.clone()
        }

        fn rotation(&self) -> Rotation {
            self.1
        }

        fn scale(&self) -> Scale {
            Scale::new(1.0, 1.0)
        }

        fn color(&self) -> Color {
            self.2
        }
    }

    impl DrawArgs for (Position, Scale, Color) {
        fn position(&self) -> Position {
            self.0.clone()
        }

        fn rotation(&self) -> Rotation {
            0.0
        }

        fn scale(&self) -> Scale {
            self.1
        }

        fn color(&self) -> Color {
            self.2
        }
    }

    impl DrawArgs for (Position, Rotation) {
        fn position(&self) -> Position {
            self.0.clone()
        }

        fn rotation(&self) -> Rotation {
            self.1
        }

        fn scale(&self) -> Scale {
            Scale::new(1.0, 1.0)
        }

        fn color(&self) -> Color {
            Color::WHITE
        }
    }

    impl DrawArgs for (Position, Scale) {
        fn position(&self) -> Position {
            self.0.clone()
        }

        fn rotation(&self) -> Rotation {
            0.0
        }

        fn scale(&self) -> Scale {
            self.1
        }

        fn color(&self) -> Color {
            Color::WHITE
        }
    }

    impl DrawArgs for (Position, Color) {
        fn position(&self) -> Position {
            self.0.clone()
        }

        fn rotation(&self) -> Rotation {
            0.0
        }

        fn scale(&self) -> Scale {
            Scale::new(1.0, 1.0)
        }

        fn color(&self) -> Color {
            self.1
        }
    }

    impl DrawArgs for Position {
        fn position(&self) -> Position {
            self.clone()
        }

        fn rotation(&self) -> Rotation {
            0.0
        }

        fn scale(&self) -> Scale {
            Scale::new(1.0, 1.0)
        }

        fn color(&self) -> Color {
            Color::WHITE
        }
    }

    impl DrawArgs for (X, Y) {
        fn position(&self) -> Position {
            Point::new(self.0, self.1)
        }

        fn rotation(&self) -> Rotation {
            0.0
        }

        fn scale(&self) -> Scale {
            Scale::new(1.0, 1.0)
        }

        fn color(&self) -> Color {
            Color::WHITE
        }
    }

    #[derive(Clone)]
    pub struct Sprite {
        image: Rc<Image>,
    }

    impl Sprite {
        #[inline]
        pub fn new(image: &Rc<Image>) -> Self {
            Sprite {
                image: Rc::clone(image),
            }
        }
        #[inline]
        pub fn texture(&self) -> &Texture {
            &self.image.texture()
        }

        #[inline]
        pub fn draw<Args: DrawArgs>(&self, target: &mut Canvas, args: Args) {
            let mut sprite_data = SfmlSprite::with_texture(self.image.texture());
            sprite_data.set_position(args.position());
            sprite_data.set_rotation(args.rotation());
            sprite_data.set_scale(args.scale());
            sprite_data.set_color(args.color());
            target.draw(&sprite_data);
        }

        #[inline]
        pub fn draw_centered<Args: DrawArgs>(&self, target: &mut Canvas, args: Args) {
            let size = self.image.size();

            let x = args.position().x - size.x as f32 / 2.0;
            let y = args.position().y - size.y as f32 / 2.0;

            let mut sprite_data = SfmlSprite::with_texture(self.image.texture());
            sprite_data.set_position(Vector2f::new(x, y));
            sprite_data.set_rotation(args.rotation());
            sprite_data.set_scale(args.scale());
            sprite_data.set_color(args.color());
            target.draw(&sprite_data);
        }

        #[inline]
        pub fn draw_on_bbox<B: BBox>(&self, target: &mut Canvas, bbox: &B) {
            let x = bbox.left();
            let y = bbox.top();
            let size = self.image.size();

            let scale_x = bbox.width() / size.x as f32;
            let scale_y = bbox.height() / size.y as f32;

            let mut sprite_data = SfmlSprite::with_texture(self.image.texture());
            sprite_data.set_position(Vector2f::new(x, y));
            sprite_data.set_scale(Vector2f::new(scale_x, scale_y));
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
            let size = self.texture().size();
            Vector2f::new(size.x as f32, size.y as f32)
        }
    }
}
pub use sprite::*;

mod text {
    use sfml::graphics::Text as SfmlText;

    use crate::resources::Font;

    use super::*;

    pub trait WriteArgs {
        fn position(&self) -> Position;
        fn color(&self) -> Color;
        fn rotation(&self) -> f32;
    }

    impl WriteArgs for (Position, Color, Rotation) {
        fn position(&self) -> Position {
            self.0.clone()
        }

        fn color(&self) -> Color {
            self.1
        }

        fn rotation(&self) -> Rotation {
            self.2
        }
    }

    impl WriteArgs for (Position, Color) {
        fn position(&self) -> Position {
            self.0.clone()
        }

        fn color(&self) -> Color {
            self.1
        }

        fn rotation(&self) -> Rotation {
            0.0
        }
    }

    impl WriteArgs for (X, Y, Color) {
        fn position(&self) -> Position {
            Point::new(self.0, self.1)
        }

        fn color(&self) -> Color {
            self.2
        }

        fn rotation(&self) -> Rotation {
            0.0
        }
    }

    pub struct Text {
        font: Rc<Font>,
        font_size: u32,
        string: String,
    }

    impl Text {
        #[inline]
        pub fn new(font: &Rc<Font>, font_size: u32, string: String) -> Self {
            Text {
                font: Rc::clone(font),
                font_size,
                string,
            }
        }

        #[inline]
        pub fn string(&self) -> &str {
            &self.string
        }

        #[inline]
        pub fn set_string(&mut self, string: String) {
            self.string = string;
        }

        #[inline]
        pub fn font_size(&self) -> u32 {
            self.font_size
        }

        #[inline]
        pub fn set_font_size(&mut self, font_size: u32) {
            self.font_size = font_size;
        }

        #[inline]
        pub fn draw<Args: WriteArgs>(&self, target: &mut Canvas, args: Args) {
            let mut text_data = SfmlText::new(&self.string, &self.font.sfml_font(), self.font_size);
            text_data.set_position(args.position());
            text_data.set_rotation(args.rotation());
            text_data.set_fill_color(args.color());
            target.draw(&text_data);
        }
    }
}
pub use text::*;
