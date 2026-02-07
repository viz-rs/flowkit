use bevy_prototype_lyon::prelude::Geometry;
use lyon_path::{BuilderImpl, builder::WithSvg};

pub use flowkit::{
    corner::{Corner, CornerPathParams},
    edge::{EdgeAnchor, EdgePath, EdgePoint, EdgeType},
    path::PathBuilder,
};

/// An `EdgePath` wrapper, drawing the connection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Connection(EdgePath);

impl Default for Connection {
    fn default() -> Self {
        Self(EdgePath::DEFAULT)
    }
}

impl From<EdgePath> for Connection {
    fn from(path: EdgePath) -> Self {
        Self(path)
    }
}

impl Geometry<WithSvg<BuilderImpl>> for Connection {
    fn add_geometry(&self, builder: &mut WithSvg<BuilderImpl>) {
        PathBuilder::from((self.0, false)).with_svg(builder);
    }
}
