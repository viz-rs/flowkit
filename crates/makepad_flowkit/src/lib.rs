use lyon_path::{BuilderImpl, builder::WithSvg};
use makepad_tessellation::prelude::*;

pub use flowkit::{
    corner::{Corner, CornerPathParams},
    edge::{EdgeAnchor, EdgePath, EdgePoint, EdgeType},
    path::PathBuilder,
};

/// The `EdgePath` wrapper for drawing the connection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Connection(EdgePath);

impl Connection {
    pub fn build_with(
        self,
        mode: Mode<StrokeOptions>,
        tess: &mut Tessellator<StrokeTessellator>,
    ) -> VertexBuffers {
        let builder: WithSvg<BuilderImpl> = PathBuilder::from((self.0, true)).into();
        let path = builder.build();

        let mut buffers = VertexBuffers::new();

        tess.stroke(path, mode, &mut buffers);

        buffers
    }
}

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
