use sfml::system::Vector2f;

mod prelude {
    use super::*;

    pub trait BBox
    where
        Self: Sized,
    {
        fn left(&self) -> f32;

        fn top(&self) -> f32;

        fn width(&self) -> f32;

        fn height(&self) -> f32;

        #[inline(always)]
        fn right(&self) -> f32 {
            self.left() + self.width()
        }

        #[inline(always)]
        fn bottom(&self) -> f32 {
            self.top() + self.height()
        }

        #[inline(always)]
        fn position(&self) -> Point {
            Point::new(self.left(), self.top())
        }

        #[inline(always)]
        fn center(&self) -> Point {
            Point::new(self.center_x(), self.center_y())
        }

        #[inline(always)]
        fn center_x(&self) -> f32 {
            (self.left() + self.right()) / 2.0
        }

        #[inline(always)]
        fn center_y(&self) -> f32 {
            (self.top() + self.bottom()) / 2.0
        }

        fn intersects_point(&self, other: &Point) -> bool;

        fn intersects_rect(&self, other: &Rect) -> bool;

        fn intersects_circle(&self, other: &Circle) -> bool;

        fn intersects_mixed(&self, other: &Mixed) -> bool {
            other.shapes.iter().any(|shape| shape.intersects(self))
        }

        fn intersects<B: BBox>(&self, other: &B) -> bool;
    }

    pub trait Movable: BBox {
        fn set_position(&mut self, x: f32, y: f32);

        #[inline]
        fn set_center(&mut self, x: f32, y: f32) {
            self.set_position(x - self.width() / 2.0, y - self.height() / 2.0);
        }

        #[inline]
        fn set_left(&mut self, x: f32) {
            self.set_position(x, self.top());
        }

        #[inline]
        fn set_top(&mut self, y: f32) {
            self.set_position(self.left(), y);
        }

        #[inline]
        fn set_right(&mut self, x: f32) {
            self.set_position(x - self.width(), self.top());
        }

        #[inline]
        fn set_bottom(&mut self, y: f32) {
            self.set_position(self.left(), y - self.height());
        }

        fn move_by(&mut self, x: f32, y: f32);
    }

    pub trait ShapeCollision {
        fn collides(&self, other: &ShapeRef) -> bool;
    }

    pub enum ShapeRef<'b> {
        None,
        Point(&'b Point),
        Rect(&'b Rect),
        Circle(&'b Circle),
        Mixed(&'b Mixed<'b>),
    }

    impl BBox for ShapeRef<'_> {
        fn left(&self) -> f32 {
            match self {
                ShapeRef::None => 0.0,
                ShapeRef::Point(point) => point.left(),
                ShapeRef::Rect(rect) => rect.left(),
                ShapeRef::Circle(circle) => circle.left(),
                ShapeRef::Mixed(mixed) => mixed.left(),
            }
        }

        fn top(&self) -> f32 {
            match self {
                ShapeRef::None => 0.0,
                ShapeRef::Point(point) => point.top(),
                ShapeRef::Rect(rect) => rect.top(),
                ShapeRef::Circle(circle) => circle.top(),
                ShapeRef::Mixed(mixed) => mixed.top(),
            }
        }

        fn width(&self) -> f32 {
            match self {
                ShapeRef::None => 0.0,
                ShapeRef::Point(point) => point.width(),
                ShapeRef::Rect(rect) => rect.width(),
                ShapeRef::Circle(circle) => circle.width(),
                ShapeRef::Mixed(mixed) => mixed.width(),
            }
        }

        fn height(&self) -> f32 {
            match self {
                ShapeRef::None => 0.0,
                ShapeRef::Point(point) => point.height(),
                ShapeRef::Rect(rect) => rect.height(),
                ShapeRef::Circle(circle) => circle.height(),
                ShapeRef::Mixed(mixed) => mixed.height(),
            }
        }

        fn intersects_point(&self, other: &Point) -> bool {
            match self {
                ShapeRef::None => false,
                ShapeRef::Point(point) => point.intersects_point(other),
                ShapeRef::Rect(rect) => rect.intersects_point(other),
                ShapeRef::Circle(circle) => circle.intersects_point(other),
                ShapeRef::Mixed(mixed) => mixed.intersects_point(other),
            }
        }

        fn intersects_rect(&self, other: &Rect) -> bool {
            match self {
                ShapeRef::None => false,
                ShapeRef::Point(point) => point.intersects_rect(other),
                ShapeRef::Rect(rect) => rect.intersects_rect(other),
                ShapeRef::Circle(circle) => circle.intersects_rect(other),
                ShapeRef::Mixed(mixed) => mixed.intersects_rect(other),
            }
        }

        fn intersects_circle(&self, other: &Circle) -> bool {
            match self {
                ShapeRef::None => false,
                ShapeRef::Point(point) => point.intersects_circle(other),
                ShapeRef::Rect(rect) => rect.intersects_circle(other),
                ShapeRef::Circle(circle) => circle.intersects_circle(other),
                ShapeRef::Mixed(mixed) => mixed.intersects_circle(other),
            }
        }

        fn intersects<B: BBox>(&self, other: &B) -> bool {
            match self {
                ShapeRef::None => false,
                ShapeRef::Point(point) => point.intersects(other),
                ShapeRef::Rect(rect) => rect.intersects(other),
                ShapeRef::Circle(circle) => circle.intersects(other),
                ShapeRef::Mixed(mixed) => mixed.intersects(other),
            }
        }
    }
}
pub use prelude::*;

mod point {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct Point {
        pub x: f32,
        pub y: f32,
    }

    impl Point {
        pub fn new(x: f32, y: f32) -> Self {
            Point { x, y }
        }

        pub fn distance(&self, other: &Point) -> f32 {
            let dx = (self.x - other.x).abs();
            let dy = (self.y - other.y).abs();
            (dx * dx + dy * dy).sqrt()
        }

        pub fn angle(&self, other: &Point) -> f32 {
            let dx = other.x - self.x;
            let dy = other.y - self.y;
            dy.atan2(dx).to_degrees().rem_euclid(360.0)
        }

        pub fn as_ref(&self) -> ShapeRef {
            ShapeRef::Point(self)
        }
    }

    impl From<Vector2f> for Point {
        fn from(vector: Vector2f) -> Self {
            Point::new(vector.x, vector.y)
        }
    }

    impl Into<Vector2f> for Point {
        fn into(self) -> Vector2f {
            Vector2f::new(self.x, self.y)
        }
    }

    impl Movable for Point {
        #[inline]
        fn set_position(&mut self, x: f32, y: f32) {
            self.x = x;
            self.y = y;
        }

        #[inline]
        fn move_by(&mut self, x: f32, y: f32) {
            self.x += x;
            self.y += y;
        }
    }

    impl BBox for Point {
        fn left(&self) -> f32 {
            self.x
        }

        fn top(&self) -> f32 {
            self.y
        }

        fn width(&self) -> f32 {
            0.0
        }

        fn height(&self) -> f32 {
            0.0
        }

        fn intersects_point(&self, other: &Point) -> bool {
            self.x as i32 == other.x as i32 && self.y as i32 == other.y as i32
        }

        fn intersects_rect(&self, other: &Rect) -> bool {
            self.x >= other.left()
                && self.x <= other.right()
                && self.y >= other.top()
                && self.y <= other.bottom()
        }

        fn intersects_circle(&self, other: &Circle) -> bool {
            self.distance(&other.center()) <= other.radius
        }

        fn intersects<B: BBox>(&self, other: &B) -> bool {
            other.intersects_point(self)
        }
    }
}
pub use point::*;

mod rect {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct Rect {
        pub x: f32,
        pub y: f32,
        pub width: f32,
        pub height: f32,
    }

    impl Rect {
        pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
            Rect {
                x,
                y,
                width,
                height,
            }
        }

        pub fn as_ref(&self) -> ShapeRef {
            ShapeRef::Rect(self)
        }
    }

    impl Movable for Rect {
        #[inline]
        fn set_position(&mut self, x: f32, y: f32) {
            self.x = x;
            self.y = y;
        }

        #[inline]
        fn move_by(&mut self, x: f32, y: f32) {
            self.x += x;
            self.y += y;
        }
    }

    impl BBox for Rect {
        fn left(&self) -> f32 {
            self.x
        }

        fn top(&self) -> f32 {
            self.y
        }

        fn width(&self) -> f32 {
            self.width
        }

        fn height(&self) -> f32 {
            self.height
        }

        fn intersects_point(&self, other: &Point) -> bool {
            other.intersects_rect(self)
        }

        fn intersects_rect(&self, other: &Rect) -> bool {
            self.left() <= other.right()
                && self.right() >= other.left()
                && self.top() <= other.bottom()
                && self.bottom() >= other.top()
        }

        fn intersects_circle(&self, other: &Circle) -> bool {
            let px = if other.center_x() < self.left() {
                self.left()
            } else if other.center_x() > self.right() {
                self.right()
            } else {
                other.center_x()
            };

            let py = if other.center_y() < self.top() {
                self.top()
            } else if other.center_y() > self.bottom() {
                self.bottom()
            } else {
                other.center_y()
            };

            Point::new(px, py).intersects_circle(other)
        }

        fn intersects<B: BBox>(&self, other: &B) -> bool {
            other.intersects_rect(self)
        }
    }
}
pub use rect::*;

mod circle {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct Circle {
        pub x: f32,
        pub y: f32,
        pub radius: f32,
    }

    impl Circle {
        pub fn new(x: f32, y: f32, radius: f32) -> Self {
            Circle { x, y, radius }
        }

        pub fn from_point(Point { x, y }: Point, radius: f32) -> Self {
            Circle::new(x, y, radius)
        }

        pub fn radius(&self) -> f32 {
            self.radius
        }

        pub fn as_ref(&self) -> ShapeRef {
            ShapeRef::Circle(self)
        }
    }

    impl Movable for Circle {
        #[inline]
        fn set_position(&mut self, x: f32, y: f32) {
            self.x = x;
            self.y = y;
        }

        #[inline]
        fn move_by(&mut self, x: f32, y: f32) {
            self.x += x;
            self.y += y;
        }
    }

    impl BBox for Circle {
        fn left(&self) -> f32 {
            self.x
        }

        fn top(&self) -> f32 {
            self.y
        }

        fn width(&self) -> f32 {
            self.radius * 2.0
        }

        fn height(&self) -> f32 {
            self.radius * 2.0
        }

        fn intersects_point(&self, other: &Point) -> bool {
            other.intersects_circle(self)
        }

        fn intersects_rect(&self, other: &Rect) -> bool {
            other.intersects_circle(self)
        }

        fn intersects_circle(&self, other: &Circle) -> bool {
            let dx = (self.center_x() - other.center_x()).abs();
            let dy = (self.center_y() - other.center_y()).abs();
            let distance = (dx * dx + dy * dy).sqrt();
            distance <= self.radius + other.radius
        }

        fn intersects<B: BBox>(&self, other: &B) -> bool {
            other.intersects_circle(self)
        }
    }
}
pub use circle::*;

mod mixed {
    use super::*;

    pub enum Shape {
        Point(Point),
        Rect(Rect),
        Circle(Circle),
    }

    impl Shape {
        pub fn as_ref(&self) -> ShapeRef {
            match self {
                Shape::Point(point) => ShapeRef::Point(point),
                Shape::Rect(rect) => ShapeRef::Rect(rect),
                Shape::Circle(circle) => ShapeRef::Circle(circle),
            }
        }
    }

    impl Movable for Shape {
        #[inline]
        fn set_position(&mut self, x: f32, y: f32) {
            match self {
                Shape::Point(point) => point.set_position(x, y),
                Shape::Rect(rect) => rect.set_position(x, y),
                Shape::Circle(circle) => circle.set_position(x, y),
            }
        }

        #[inline]
        fn move_by(&mut self, x: f32, y: f32) {
            match self {
                Shape::Point(point) => point.move_by(x, y),
                Shape::Rect(rect) => rect.move_by(x, y),
                Shape::Circle(circle) => circle.move_by(x, y),
            }
        }
    }

    impl BBox for Shape {
        fn left(&self) -> f32 {
            match self {
                Shape::Point(point) => point.left(),
                Shape::Rect(rect) => rect.left(),
                Shape::Circle(circle) => circle.left(),
            }
        }

        fn top(&self) -> f32 {
            match self {
                Shape::Point(point) => point.top(),
                Shape::Rect(rect) => rect.top(),
                Shape::Circle(circle) => circle.top(),
            }
        }

        fn width(&self) -> f32 {
            match self {
                Shape::Point(point) => point.width(),
                Shape::Rect(rect) => rect.width(),
                Shape::Circle(circle) => circle.width(),
            }
        }

        fn height(&self) -> f32 {
            match self {
                Shape::Point(point) => point.height(),
                Shape::Rect(rect) => rect.height(),
                Shape::Circle(circle) => circle.height(),
            }
        }

        fn intersects_point(&self, other: &Point) -> bool {
            match self {
                Shape::Point(point) => point.intersects_point(other),
                Shape::Rect(rect) => rect.intersects_point(other),
                Shape::Circle(circle) => circle.intersects_point(other),
            }
        }

        fn intersects_rect(&self, other: &Rect) -> bool {
            match self {
                Shape::Point(point) => point.intersects_rect(other),
                Shape::Rect(rect) => rect.intersects_rect(other),
                Shape::Circle(circle) => circle.intersects_rect(other),
            }
        }

        fn intersects_circle(&self, other: &Circle) -> bool {
            match self {
                Shape::Point(point) => point.intersects_circle(other),
                Shape::Rect(rect) => rect.intersects_circle(other),
                Shape::Circle(circle) => circle.intersects_circle(other),
            }
        }

        fn intersects<B: BBox>(&self, other: &B) -> bool {
            match self {
                Shape::Point(point) => point.intersects(other),
                Shape::Rect(rect) => rect.intersects(other),
                Shape::Circle(circle) => circle.intersects(other),
            }
        }
    }

    pub struct Mixed<'b> {
        pub shapes: &'b [&'b Shape],
    }

    impl BBox for Mixed<'_> {
        fn left(&self) -> f32 {
            self.shapes
                .iter()
                .map(|shape| shape.left())
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.0)
        }

        fn top(&self) -> f32 {
            self.shapes
                .iter()
                .map(|shape| shape.top())
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.0)
        }

        fn right(&self) -> f32 {
            self.shapes
                .iter()
                .map(|shape| shape.right())
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.0)
        }

        fn bottom(&self) -> f32 {
            self.shapes
                .iter()
                .map(|shape| shape.bottom())
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.0)
        }

        fn width(&self) -> f32 {
            self.right() - self.left()
        }

        fn height(&self) -> f32 {
            self.bottom() - self.top()
        }

        fn intersects_point(&self, other: &Point) -> bool {
            self.shapes
                .iter()
                .any(|shape| shape.intersects_point(other))
        }

        fn intersects_rect(&self, other: &Rect) -> bool {
            self.shapes.iter().any(|shape| shape.intersects_rect(other))
        }

        fn intersects_circle(&self, other: &Circle) -> bool {
            self.shapes
                .iter()
                .any(|shape| shape.intersects_circle(other))
        }

        fn intersects<B: BBox>(&self, other: &B) -> bool {
            self.shapes.iter().any(|shape| shape.intersects(other))
        }
    }
}
pub use mixed::*;

mod draw {
    use super::*;

    use sfml::graphics::{CircleShape, Color, RectangleShape, Shape as SfmlShape, Transformable};

    use crate::window::Canvas;

    pub trait DrawBBox {
        fn draw(&self, target: &mut Canvas);
    }

    impl DrawBBox for ShapeRef<'_> {
        fn draw(&self, target: &mut Canvas) {
            match self {
                ShapeRef::Rect(rect) => {
                    let mut rect_data = RectangleShape::new();
                    rect_data.set_position((rect.left(), rect.top()));
                    rect_data.set_size((rect.width(), rect.height()));
                    rect_data.set_outline_thickness(1.0);
                    rect_data.set_outline_color(Color::MAGENTA);
                    rect_data.set_fill_color(Color::TRANSPARENT);
                    target.draw(&rect_data);
                }
                ShapeRef::Circle(circle) => {
                    let mut circle_data = CircleShape::new(circle.radius(), 24);
                    circle_data.set_position(circle.center());
                    circle_data.set_outline_thickness(1.0);
                    circle_data.set_outline_color(Color::MAGENTA);
                    circle_data.set_fill_color(Color::TRANSPARENT);
                    target.draw(&circle_data);
                }
                ShapeRef::Point(point) => {
                    let mut circle_data = CircleShape::new(1.0, 1);
                    circle_data.set_position(point.position());
                    circle_data.set_outline_thickness(1.0);
                    circle_data.set_outline_color(Color::MAGENTA);
                    circle_data.set_fill_color(Color::TRANSPARENT);
                    target.draw(&circle_data);
                }
                ShapeRef::Mixed(mixed) => {
                    for shape in mixed.shapes {
                        match shape {
                            Shape::Rect(rect) => rect.as_ref().draw(target),
                            Shape::Circle(circle) => circle.as_ref().draw(target),
                            Shape::Point(point) => point.as_ref().draw(target),
                        }
                    }
                }
                ShapeRef::None => {}
            }
        }
    }
}
pub use draw::*;
