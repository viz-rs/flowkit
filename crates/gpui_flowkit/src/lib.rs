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

impl From<Connection> for WithSvg<BuilderImpl> {
    fn from(value: Connection) -> Self {
        PathBuilder::from((value.0, true)).into()
    }
}

impl From<Connection> for gpui::PathBuilder {
    fn from(value: Connection) -> Self {
        WithSvg::<BuilderImpl>::from(value).into()
    }
}
