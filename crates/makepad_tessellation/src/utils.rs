use makepad_widgets::Vec2d;

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

impl Convert<Vec2d> for lyon_path::math::Point {
    fn convert(self) -> Vec2d {
        Vec2d {
            x: self.x as f64,
            y: self.y as f64,
        }
    }
}
