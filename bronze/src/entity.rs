use std::time::Duration;

use sfml::system::Vector2f;

use crate::{graphics::Canvas, input::InputManager};

pub type Position = Vector2f;

pub trait EntityPosition {
    fn position(&self) -> Position;
    fn set_position(&mut self, x: f32, y: f32);
    fn move_by(&mut self, x: f32, y: f32);
}

pub type Size = Vector2f;

pub trait EntitySize {
    fn size(&self) -> Size;
    fn set_size(&mut self, x: f32, y: f32);
}

pub type Rotation = f32;

pub trait EntityRotation {
    fn rotation(&self) -> Rotation;
    fn set_rotation(&mut self, rotation: Rotation);
    fn rotate_by(&mut self, rotation: Rotation);
}

pub type Scale = Vector2f;

pub trait EntityScale {
    fn scale(&self) -> Scale;
    fn set_scale(&mut self, x: f32, y: f32);
    fn scale_by(&mut self, x: f32, y: f32);
}

pub trait Entity<Ctx>: EntityPosition + EntityRotation + EntityScale + EntitySize {
    fn input(&mut self, input: &InputManager) {
        let _ = input;
    }

    fn pre_update(&mut self, ctx: &Ctx) {
        let _ = ctx;
    }

    fn update(&mut self, ctx: &mut Ctx, frame_time: Duration);

    fn post_update(&mut self, ctx: &Ctx) {
        let _ = ctx;
    }

    fn draw(&self, ctx: &Ctx, target: &mut dyn Canvas);
}
