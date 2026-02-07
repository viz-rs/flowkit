use std::ops::Sub;

use glam::Vec2;
use lyon_path::{BuilderImpl, builder::WithSvg, math::Angle};

use crate::{utils::Convert, winding_order::WindingOrder};

/// Identifies a corner of a rectangle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Corner {
    TopRight,
    TopLeft,
    BottomLeft,
    BottomRight,
    // An invalid corner indicator.
    Invalid,
}

impl Corner {
    /// A vector representation of the top-right corner.
    pub const TOP_RIGHT: Vec2 = Vec2::ONE;

    /// A vector representation of the top-left corner.
    pub const TOP_LEFT: Vec2 = Vec2::new(-1.0, 1.0);

    /// A vector representation of the bottom-left corner.
    pub const BOTTOM_LEFT: Vec2 = Vec2::NEG_ONE;

    /// A vector representation of the bottom-right corner.
    pub const BOTTOM_RIGHT: Vec2 = Vec2::new(1.0, -1.0);
}

impl Corner {
    /// Creates a new corner from a vector.
    #[inline]
    pub const fn new(value: Vec2) -> Self {
        match value {
            Self::TOP_RIGHT => Self::TopRight,
            Self::TOP_LEFT => Self::TopLeft,
            Self::BOTTOM_LEFT => Self::BottomLeft,
            Self::BOTTOM_RIGHT => Self::BottomRight,
            _ => Self::Invalid,
        }
    }

    /// Calculates the corner based on the previous, current, and next points.
    #[inline]
    pub fn calculate(prev: Vec2, current: Vec2, next: Vec2) -> Self {
        let center = prev.midpoint(next);
        let sign = current.sub(center).signum();
        Self::new(sign)
    }

    /// Casts a corner to a vector.
    #[inline]
    pub const fn as_vec2(&self) -> Vec2 {
        match self {
            Self::TopRight => Self::TOP_RIGHT,
            Self::TopLeft => Self::TOP_LEFT,
            Self::BottomLeft => Self::BOTTOM_LEFT,
            Self::BottomRight => Self::BOTTOM_RIGHT,
            Self::Invalid => Vec2::ZERO,
        }
    }
}

/// Calculates the parameters for a corner path.
///
/// Links:
/// * [Desperately seeking squircles](https://www.figma.com/blog/desperately-seeking-squircles/)
/// * [Flutter implementation](https://github.com/aloisdeniel/figma_squircle)
/// * [JavaScript implementation](https://github.com/phamfoo/figma-squircle)
#[derive(Clone, Copy)]
pub struct CornerPathParams {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub p: f32,
    pub corner_radius: f32,
    pub arc_section_length: f32,
    pub arc_theta: f32,
}

impl CornerPathParams {
    #[inline]
    pub fn new(mut corner_radius: f32, max_radius: f32, smoothness: f32) -> Self {
        // From figure 12.2 in the article
        let mut p = (1.0 + smoothness) * corner_radius;

        if p > max_radius {
            p = max_radius;
            corner_radius = p / (1.0 + smoothness);
        }

        let quarter = f32::to_radians(45.0);
        let angle_alpha = quarter * smoothness;
        let angle_beta = (90.0 * (1.0 - smoothness)).to_radians();
        let arc_theta = quarter - angle_beta * 0.5;

        // This was called `h_longest` in the original code
        // In the article this is the distance between 2 control points: P3 and P4
        let p3_to_p4_distance = corner_radius * arc_theta.tan() * 0.5;

        // This was called `l` in the original code
        let arc_section_length = corner_radius * f32::sqrt(2.0) * angle_beta.sin() * 0.5;

        // From figure 11.1 in the article
        // a, b, c and d
        let c = p3_to_p4_distance * angle_alpha.cos();
        let d = c * angle_alpha.tan();
        let b = (p - arc_section_length - c - d) / 3.0;
        let a = b * 2.0;

        Self {
            a,
            b,
            c,
            d,
            p,
            corner_radius,
            arc_section_length,
            arc_theta,
        }
    }

    /// Generate actions for a smooth corner.
    ///
    /// Clockwise and horizontal by default.
    /// Uses the `top-right` corner as the base model.
    #[inline]
    pub fn squircle(self, current: Vec2, corner: Corner, winding_order: WindingOrder) -> Squircle {
        let Self {
            a,
            b,
            c,
            d,
            p,
            corner_radius,
            arc_theta,
            ..
        } = self;

        // directions
        let edges = corner.as_vec2();
        let orientation = winding_order.as_f32();
        let Vec2 { x, y } = edges;
        let product = x * y;

        // counter-clockwise
        let is_ccw = orientation == 1.0;
        // top-left or bottom-right
        let is_tl_or_br = product == -1.0;
        // it only takes one swap
        let should_swap = is_ccw ^ is_tl_or_br;

        // arc center
        let radii = Vec2::splat(corner_radius);
        let center = current - radii * edges;
        let sweep_angle = arc_theta * orientation;

        // calculates new `d` by multiplying product with `-d`
        let d = product * -d;

        // horizontal direction
        let mut h = {
            let p0 = current - Vec2::new(p, 0.0) * x;
            let ctrl1 = p0 + Vec2::new(a, 0.0) * x;
            let ctrl2 = p0 + Vec2::new(a + b, 0.0) * x;
            let to0 = p0 + Vec2::new(a + b + c, d) * x;
            [p0, ctrl1, ctrl2, to0]
        };
        // vertical direction
        let mut v = {
            let p0 = current - Vec2::new(0.0, p) * y;
            let ctrl1 = p0 + Vec2::new(0.0, a) * y;
            let ctrl2 = p0 + Vec2::new(0.0, a + b) * y;
            let to0 = p0 + Vec2::new(d, a + b + c) * y;
            [p0, ctrl1, ctrl2, to0]
        };

        if should_swap {
            ::core::mem::swap(&mut h, &mut v);
        }

        Squircle {
            h,
            v,
            center,
            radii,
            sweep_angle,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Squircle {
    pub h: [Vec2; 4],
    pub v: [Vec2; 4],
    pub center: Vec2,
    pub radii: Vec2,
    pub sweep_angle: f32,
}

impl Squircle {
    #[inline]
    pub fn with_svg(self, builder: &mut WithSvg<BuilderImpl>) {
        let Self {
            h,
            v,
            center,
            radii,
            sweep_angle,
        } = self;

        let [p0, ctrl1, ctrl2, to0] = h.convert();
        let [to1, ctrl4, ctrl3, p1] = v.convert();

        builder.line_to(p0);
        builder.cubic_bezier_to(ctrl1, ctrl2, to0);
        builder.arc(
            center.convert(),
            radii.convert(),
            Angle::radians(sweep_angle),
            Angle::radians(0.0),
        );
        builder.line_to(p1);
        builder.cubic_bezier_to(ctrl3, ctrl4, to1);
    }
}
