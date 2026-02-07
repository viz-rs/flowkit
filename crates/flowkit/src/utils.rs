use glam::Vec2;
use lyon_path::math::{Point, Vector};

/// Converts a type to another type.
pub trait Convert<T> {
    #[must_use]
    fn convert(self) -> T;
}

impl<const N: usize, O, I: Convert<O>> Convert<[O; N]> for [I; N] {
    fn convert(self) -> [O; N] {
        self.map(I::convert)
    }
}

impl Convert<Point> for Vec2 {
    fn convert(self) -> Point {
        Point::new(self.x, self.y)
    }
}

impl Convert<Vector> for Vec2 {
    fn convert(self) -> Vector {
        Vector::new(self.x, self.y)
    }
}

/// Calculates the visible area of a rectangle defined by its minimum and maximum coordinates.
#[inline]
pub const fn visible_area(min: Vec2, max: Vec2) -> f32 {
    let x = (max.x - min.x).max(0.0);
    let y = (max.y - min.y).max(0.0);
    x * y
}

/// Selects a vector based on a flag.
///
/// If the flag is 1.0, returns the maximum of the base and max vectors.
/// If the flag is -1.0, returns the minimum of the base and min vectors.
/// Otherwise, returns the base vector.
#[inline]
pub const fn select(flags: Vec2, base: Vec2, min: Vec2, max: Vec2) -> Vec2 {
    Vec2 {
        x: select_single(flags.x, base.x, min.x, max.x),
        y: select_single(flags.y, base.y, min.y, max.y),
    }
}

/// Selects a single value based on a flag.
///
/// If the flag is 1.0, returns the maximum of the base and max values.
/// If the flag is -1.0, returns the minimum of the base and min values.
/// Otherwise, returns the base value.
#[inline]
pub const fn select_single(flag: f32, base: f32, min: f32, max: f32) -> f32 {
    if flag == 1.0 {
        base.max(max)
    } else if flag == -1.0 {
        base.min(min)
    } else {
        base
    }
}
