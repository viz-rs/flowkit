use glam::Vec2;
use lyon_path::{BuilderImpl, builder::WithSvg};
use smallvec::SmallVec;

use crate::{
    curve::calculate_control_point,
    edge::{EdgePath, EdgePoint, EdgeType},
    extend::ExtendFrom,
    utils::{select, visible_area},
};

/// A path builder.
#[derive(Debug, Clone)]
pub struct PathBuilder {
    pub offset: f32,
    pub curvature: f32,
    pub edge_type: EdgeType,
    pub points: SmallVec<[Vec2; 2]>,
}

impl PathBuilder {
    /// If `Y-Axis` is down, should set `flip_y` to `true`.
    ///
    /// | Y-Axis | Framework                  |
    /// |--------|----------------------------|
    /// | Up     | Bevy world                 |
    /// | Down   | egui, gpui, makepad screen |
    #[inline]
    pub fn new(
        source: EdgePoint,
        target: EdgePoint,
        edge_type: EdgeType,
        curvature: f32,
        offset: f32,
        flip_y: bool,
    ) -> Self {
        let mut source = [source.0, source.1.as_vec2()];
        let mut target = [target.0, target.1.as_vec2()];

        if flip_y {
            source[1].y *= -1.0;
            target[1].y *= -1.0;
        }

        let mut points = SmallVec::new_const();

        match edge_type {
            EdgeType::Straight => {
                points.extend_from_slice(&[source[0], target[0]]);
            }
            EdgeType::Curve => {
                points.extend_from_slice(&Self::calculate_control_points(
                    source, target, curvature, offset,
                ));
            }
            EdgeType::StraightStep | EdgeType::SmoothStep => {
                points.extend_from_slice(&Self::calculate_steps(source, target, offset));
            }
        }

        Self {
            points,
            edge_type,
            curvature,
            offset,
        }
    }

    #[inline]
    pub fn calculate_control_points(
        source: [Vec2; 2],
        target: [Vec2; 2],
        curvature: f32,
        offset: f32,
    ) -> [Vec2; 4] {
        let [source_pos, source_edge] = source;
        let [target_pos, target_edge] = target;

        let source_control_point =
            calculate_control_point(source_pos, target_pos, source_edge, curvature, offset);
        let target_control_point =
            calculate_control_point(target_pos, source_pos, target_edge, curvature, offset);

        [
            source_pos,
            source_control_point,
            target_control_point,
            target_pos,
        ]
    }

    #[inline]
    pub fn calculate_steps(
        source: [Vec2; 2],
        target: [Vec2; 2],
        offset: f32,
    ) -> SmallVec<[Vec2; 3]> {
        let [source_pos, source_edge] = source;
        let [target_pos, target_edge] = target;

        let (rect_min, rect_max) = (source_pos.min(target_pos), source_pos.max(target_pos));
        let area = visible_area(rect_min, rect_max);

        let (source_offset, target_offset) = (source_edge * offset, target_edge * offset);

        let (source_offset_pos, target_offset_pos) =
            (source_pos + source_offset, target_pos + target_offset);

        let (new_rect_min, new_rect_max) = (
            rect_min.min(source_offset_pos).min(target_offset_pos),
            rect_max.max(source_offset_pos).max(target_offset_pos),
        );
        let new_area = visible_area(new_rect_min, new_rect_max);

        let center = new_rect_min.midpoint(new_rect_max);

        let edges = source_edge * target_edge;
        let is_adjacent_edge = edges == Vec2::ZERO;
        let is_same_edge = !is_adjacent_edge && edges.cmpeq(Vec2::ONE).any();
        let is_same_area = area == new_area;

        let mut points = SmallVec::new_const();

        points.push(source_pos);

        if is_same_edge {
            // same edges
            // adds two corner points
            let sc = select(source_edge, source_offset_pos, new_rect_min, new_rect_max);
            let tc = select(target_edge, target_offset_pos, new_rect_min, new_rect_max);
            points.extend_from_slice(&[sc, tc]);
        } else if is_adjacent_edge && is_same_area {
            // adjacent edges and same area
            // adds one corner point
            let c = select(source_edge, source_offset_pos, new_rect_min, new_rect_max);
            points.push(c);
        } else {
            // source offset point
            let sc = select(
                source_edge,
                source_offset_pos,
                source_offset_pos.min(center),
                source_offset_pos.max(center),
            );
            // target offset point
            let tc = select(
                target_edge,
                target_offset_pos,
                target_offset_pos.min(center),
                target_offset_pos.max(center),
            );

            // source middle point
            let mut sm = select(source_edge, center, sc.min(center), sc.max(center));
            // target middle point
            let mut tm = select(target_edge, center, tc.min(center), tc.max(center));

            let mut temp = SmallVec::<[Vec2; 3]>::new_const();

            temp.push(sc);

            if is_adjacent_edge {
                // adjacent edges
                // adds a middle corner point
                // keeps value by multiplying with edge vector length
                sm *= source_edge.abs();
                tm *= target_edge.abs();

                temp.push(sm + tm);
            } else {
                // parallel edges
                // adds two middle corner points
                temp.extend_from_slice(&[sm, tm]);
            }

            temp.push(tc);
            temp.dedup();

            points.extend_from_slice(&temp);
        }

        points.push(target_pos);

        points
    }
}

impl From<(EdgePath, bool)> for PathBuilder {
    /// If `Y-Axis` is down, should set `flip_y` to `true`.
    ///
    /// | Y-Axis | Framework                  |
    /// |--------|----------------------------|
    /// | Up     | Bevy world                 |
    /// | Down   | egui, gpui, makepad screen |
    fn from((path, flip_y): (EdgePath, bool)) -> Self {
        Self::new(
            path.source,
            path.target,
            path.edge_type,
            path.curvature,
            path.offset,
            flip_y,
        )
    }
}

impl From<PathBuilder> for WithSvg<BuilderImpl> {
    fn from(path: PathBuilder) -> Self {
        let mut builder = BuilderImpl::new().with_svg();
        builder.extend(path);
        builder
    }
}
