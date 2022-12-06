use std::{marker::PhantomData, time::Duration};

use crate::{
    input::InputManager,
    shape::{BBox, DrawBBox, ShapeRef},
    window::Canvas,
};

pub trait Entity<Ctx> {
    #[inline]
    fn bbox(&self) -> ShapeRef {
        ShapeRef::None
    }

    #[inline]
    fn should_be_removed(&self) -> bool {
        false
    }

    #[inline]
    fn input(&mut self, input: &InputManager) {
        let _ = input;
    }

    #[inline]
    fn pre_update(&mut self, ctx: &Ctx) {
        let _ = ctx;
    }

    fn update(&mut self, ctx: &mut Ctx, frame_time: Duration) {
        let _ = (ctx, frame_time);
    }

    #[inline]
    fn post_update(&mut self, ctx: &Ctx) {
        let _ = ctx;
    }

    #[inline]
    fn draw(&self, ctx: &Ctx, target: &mut Canvas) {
        let _ = (ctx, target);
    }
}

pub trait Collision<T: Entity<Ctx>, Ctx> {
    fn on_collision(&mut self, other: &T, ctx: &mut Ctx);
}

pub struct Scene<S, D, Ctx>
where
    S: Entity<Ctx> + Collision<D, Ctx>,
    D: Entity<Ctx> + Collision<D, Ctx> + Collision<S, Ctx>,
{
    static_entities: Vec<S>,
    dynamic_entities: Vec<D>,
    phantom: PhantomData<Ctx>,
}

impl<S, D, Ctx> Scene<S, D, Ctx>
where
    S: Entity<Ctx> + Collision<D, Ctx>,
    D: Entity<Ctx> + Collision<D, Ctx> + Collision<S, Ctx>,
{
    #[inline]
    pub fn new() -> Self {
        Scene {
            static_entities: Vec::new(),
            dynamic_entities: Vec::new(),
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn add_static<E: Into<S>>(&mut self, entity: E) {
        self.static_entities.push(entity.into());
    }

    #[inline]
    pub fn add_dynamic<E: Into<D>>(&mut self, entity: E) {
        self.dynamic_entities.push(entity.into());
    }

    pub fn input(&mut self, input: &crate::input::InputManager) {
        for entity in self.static_entities.iter_mut() {
            entity.input(input);
        }

        for entity in self.dynamic_entities.iter_mut() {
            entity.input(input);
        }
    }

    pub fn pre_update(&mut self, ctx: &Ctx) {
        for entity in self.static_entities.iter_mut() {
            entity.pre_update(ctx);
        }

        for entity in self.dynamic_entities.iter_mut() {
            entity.pre_update(ctx);
        }
    }

    pub fn update(&mut self, ctx: &mut Ctx, frame_time: Duration) {
        for entity in self.static_entities.iter_mut() {
            entity.update(ctx, frame_time);
        }

        for entity in self.dynamic_entities.iter_mut() {
            entity.update(ctx, frame_time);
        }
    }

    pub fn post_update(&mut self, ctx: &Ctx) {
        for entity in self.static_entities.iter_mut() {
            entity.post_update(ctx);
        }

        for entity in self.dynamic_entities.iter_mut() {
            entity.post_update(ctx);
        }

        self.static_entities.retain(|e| !e.should_be_removed());
        self.dynamic_entities.retain(|e| !e.should_be_removed());
    }

    pub fn collisions(&mut self, ctx: &mut Ctx) {
        let mut dynamic_collisions = Vec::new();
        let mut dynamic_static_collisions = Vec::new();

        if self.dynamic_entities.len() > 1 {
            for (i, entity) in self.dynamic_entities.iter().enumerate() {
                for (j, other) in self.dynamic_entities.iter().enumerate().skip(i + 1) {
                    if entity.bbox().intersects(&other.bbox()) {
                        dynamic_collisions.push((i, j));
                    }
                }
            }
        }

        for (i, entity) in self.dynamic_entities.iter().enumerate() {
            for (j, other) in self.static_entities.iter().enumerate() {
                if entity.bbox().intersects(&other.bbox()) {
                    dynamic_static_collisions.push((i, j));
                }
            }
        }

        for (i, j) in dynamic_collisions {
            let (left, b_right) = self.dynamic_entities.split_at_mut(j);
            let (_, a_right) = left.split_at_mut(i);

            let a = &mut a_right[0];
            let b = &mut b_right[0];

            a.on_collision(b, ctx);
            b.on_collision(a, ctx);
        }

        for (i, j) in dynamic_static_collisions {
            let a = &mut self.dynamic_entities[i];
            let b = &mut self.static_entities[j];

            a.on_collision(b, ctx);
            b.on_collision(a, ctx);
        }
    }

    pub fn draw(&self, ctx: &Ctx, canvas: &mut Canvas) {
        for entity in self.static_entities.iter() {
            entity.draw(ctx, canvas);
        }

        for entity in self.dynamic_entities.iter() {
            entity.draw(ctx, canvas);
        }
    }

    pub fn draw_bboxes(&self, canvas: &mut Canvas) {
        for entity in self.static_entities.iter() {
            entity.bbox().draw(canvas);
        }

        for entity in self.dynamic_entities.iter() {
            entity.bbox().draw(canvas);
        }
    }

    pub fn remove_static(&mut self, entity: &S) -> Option<S> {
        let index = self
            .static_entities
            .iter()
            .position(|e| std::ptr::eq(e, entity))?;
        Some(self.static_entities.swap_remove(index))
    }

    pub fn remove_dynamic(&mut self, entity: &D) -> Option<D> {
        let index = self
            .dynamic_entities
            .iter()
            .position(|e| std::ptr::eq(e, entity))?;
        Some(self.dynamic_entities.swap_remove(index))
    }
}
