use lyon_path::Path;
use lyon_tessellation::{
    BuffersBuilder, FillOptions, FillTessellator, StrokeOptions, StrokeTessellator,
};

use crate::vertex::{VertexBuffers, VertexConstructor};

/// A `FillOptions` or `StrokeOptions` wrapper.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Mode<T> {
    pub options: T,
}

/// A `FillTessellator` or `StrokeTessellator` wrapper.
pub struct Tessellator<T> {
    inner: T,
}

impl<T: Default> Default for Tessellator<T> {
    fn default() -> Self {
        Self {
            inner: T::default(),
        }
    }
}

impl Tessellator<FillTessellator> {
    // Computes the tessellation from a path slice.
    pub fn fill(&mut self, path: Path, mode: Mode<FillOptions>, buffers: &mut VertexBuffers) {
        let Mode { options } = mode;
        if let Err(e) = self.inner.tessellate_path(
            &path,
            &options,
            &mut BuffersBuilder::new(buffers, VertexConstructor),
        ) {
            tracing::error!("FillTessellator error: {:?}", e);
        }
    }
}

impl Tessellator<StrokeTessellator> {
    // Computes the tessellation from a path slice.
    pub fn stroke(&mut self, path: Path, mode: Mode<StrokeOptions>, buffers: &mut VertexBuffers) {
        let Mode { options } = mode;
        if let Err(e) = self.inner.tessellate_path(
            &path,
            &options,
            &mut BuffersBuilder::new(buffers, VertexConstructor),
        ) {
            tracing::error!("StrokeTessellator error: {:?}", e);
        }
    }
}
