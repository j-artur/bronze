use std::time::Duration;

use crate::{entity::Entity, graphics::Canvas};

pub struct Scene<'r, Ctx> {
    pub entities: Vec<Box<(dyn Entity<Ctx> + 'r)>>,
}

impl<'r, Ctx> Scene<'r, Ctx> {
    pub fn new() -> Self {
        Self { entities: vec![] }
    }

    pub fn add_entity(&mut self, entity: Box<(dyn Entity<Ctx> + 'r)>) {
        self.entities.push(entity);
    }

    pub fn remove_entity<E: Entity<Ctx>>(&mut self, entity: &E) {
        self.entities.retain(|e| !std::ptr::eq(e.as_ref(), entity));
    }

    pub fn input(&mut self, input: &crate::input::InputManager) {
        for entity in self.entities.iter_mut() {
            entity.input(input);
        }
    }

    pub fn pre_update(&mut self, ctx: &Ctx) {
        for entity in self.entities.iter_mut() {
            entity.pre_update(ctx);
        }
    }

    pub fn update(&mut self, ctx: &mut Ctx, frame_time: Duration) {
        for entity in self.entities.iter_mut() {
            entity.update(ctx, frame_time);
        }
    }

    pub fn post_update(&mut self, ctx: &Ctx) {
        for entity in self.entities.iter_mut() {
            entity.post_update(ctx);
        }
    }

    pub fn draw<C: Canvas>(&self, ctx: &Ctx, canvas: &mut C) {
        for entity in self.entities.iter() {
            entity.draw(ctx, canvas);
        }
    }
}
