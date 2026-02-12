use lyon_path::{BuilderImpl, builder::WithSvg, geom::Angle};

use crate::{
    corner::{Corner, CornerPathParams, Squircle},
    edge::EdgeType,
    path::PathBuilder,
    utils::Convert,
    winding_order::WindingOrder,
};

// Appends a value to a builder.
pub trait ExtendFrom<T> {
    fn extend_from(&mut self, value: T);
}

impl ExtendFrom<Squircle> for WithSvg<BuilderImpl> {
    fn extend_from(&mut self, squircle: Squircle) {
        let Squircle {
            h,
            v,
            center,
            radii,
            sweep_angle,
        } = squircle;

        // horizontal
        let [p, ctrl1, ctrl2, to] = h.convert();
        self.line_to(p);
        self.cubic_bezier_to(ctrl1, ctrl2, to);

        // corner
        self.arc(
            center.convert(),
            radii.convert(),
            Angle::radians(sweep_angle),
            Angle::radians(0.0),
        );

        // vertical
        let [to, ctrl2, ctrl1, p] = v.convert();
        self.line_to(p);
        self.cubic_bezier_to(ctrl1, ctrl2, to);
    }
}

impl ExtendFrom<PathBuilder> for WithSvg<BuilderImpl> {
    fn extend_from(&mut self, path: PathBuilder) {
        let PathBuilder {
            points,
            offset,
            edge_type,
            ..
        } = path;

        match edge_type {
            EdgeType::Straight => {
                let [from, to] = points[..] else {
                    panic!("Straight path needs tow points.");
                };
                self.move_to(from.convert());
                self.line_to(to.convert());
            }
            EdgeType::Curve => {
                let [from, ctrl1, ctrl2, to] = points[..] else {
                    panic!("Curve path needs four points.");
                };
                self.move_to(from.convert());
                self.cubic_bezier_to(ctrl1.convert(), ctrl2.convert(), to.convert());
            }
            EdgeType::StraightStep => {
                for point in points {
                    self.line_to(point.convert());
                }
            }
            EdgeType::SmoothStep => {
                let len = points.len();

                if len < 2 {
                    return;
                }

                self.move_to(points[0].convert());

                // @todo(fundon): should be a configuration
                let smoothness = 0.6;
                let half_offset = offset * 0.5;

                for window in points.windows(3) {
                    let [prev, current, next] = window[..] else {
                        break;
                    };

                    let rect = (next - prev).abs();
                    let max_radius = rect.x.min(rect.y) * 0.5;

                    // 5.0 by default
                    // @todo(fundon): should be a configuration
                    let corner_radius = max_radius.min(half_offset);

                    let squircle = CornerPathParams::new(corner_radius, max_radius, smoothness)
                        .squircle(
                            current,
                            Corner::calculate(prev, current, next),
                            WindingOrder::calculate(prev, current, next),
                        );

                    self.extend_from(squircle);
                }

                self.line_to(points[len - 1].convert());
            }
        }
    }
}
