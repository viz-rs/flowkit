use egui::{Color32, Mesh, TextureId};
use lyon_path::Path;
use lyon_tessellation::{
    BuffersBuilder, FillOptions, FillTessellator, StrokeOptions, StrokeTessellator,
};

use crate::vertex::{VertexBuffers, VertexConstructor};

/// A `FillOptions` or `StrokeOptions` wrapper with a color.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Mode<T> {
    pub options: T,
    pub color: Color32,
}

/// A `FillTessellator` or `StrokeTessellator` wrapper.
pub struct Tessellator<T> {
    inner: T,
}

impl<T: Default> Default for Tessellator<T> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl Tessellator<()> {
    pub fn build_mesh(buffers: VertexBuffers) -> Mesh {
        Mesh {
            indices: buffers.indices,
            vertices: buffers.vertices,
            texture_id: TextureId::default(),
        }
    }
}

impl Tessellator<FillTessellator> {
    pub fn fill(&mut self, path: &Path, mode: Mode<FillOptions>, buffers: &mut VertexBuffers) {
        let Mode { options, color } = mode;
        if let Err(e) = self.inner.tessellate_path(
            path,
            &options,
            &mut BuffersBuilder::new(buffers, VertexConstructor { color }),
        ) {
            tracing::error!("FillTessellator error: {:?}", e);
        }
    }
}

impl Tessellator<StrokeTessellator> {
    pub fn stroke(&mut self, path: &Path, mode: Mode<StrokeOptions>, buffers: &mut VertexBuffers) {
        let Mode { options, color } = mode;
        if let Err(e) = self.inner.tessellate_path(
            path,
            &options,
            &mut BuffersBuilder::new(buffers, VertexConstructor { color }),
        ) {
            tracing::error!("StrokeTessellator error: {:?}", e);
        }
    }
}
