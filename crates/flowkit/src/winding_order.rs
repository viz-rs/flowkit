use glam::Vec2;

/// The winding order for a set of points.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindingOrder {
    /// A clockwise winding order.
    Clockwise,
    /// A counterclockwise winding order.
    CounterClockwise,
    /// An invalid winding order indicating that it could not be computed reliably.
    /// This often happens in *degenerate cases* where the points lie on the same line.
    Invalid,
}

impl WindingOrder {
    /// Calculates the winding order based on the previous, current, and next points.
    ///
    /// <https://en.wikipedia.org/wiki/Curve_orientation>
    #[inline]
    pub fn calculate(prev: Vec2, current: Vec2, next: Vec2) -> Self {
        let area = (current - prev).perp_dot(next - prev);

        if area > f32::EPSILON {
            Self::CounterClockwise
        } else if area < -f32::EPSILON {
            Self::Clockwise
        } else {
            Self::Invalid
        }
    }

    /// Casts the winding order to a floating-point value.
    ///
    /// If the winding order is `CounterClockwise`, returns `1.0`.
    /// If the winding order is `Clockwise`, returns `-1.0`.
    /// If the winding order is `Invalid`, returns `0.0`.
    #[inline]
    pub const fn as_f32(self) -> f32 {
        match self {
            Self::CounterClockwise => 1.0,
            Self::Clockwise => -1.0,
            Self::Invalid => 0.0,
        }
    }
}
