use lyon_tessellation::{
    self as tess, FillVertex, FillVertexConstructor, StrokeVertex, StrokeVertexConstructor,
};
use makepad_widgets::Vec2d;

use crate::utils::Convert;

/// The index type of a mesh.
type IndexType = usize;
/// Lyon's [`VertexBuffers`] generic data type defined for [`Vec2d`].
pub type VertexBuffers = tess::VertexBuffers<Vec2d, IndexType>;

/// Zero-sized type used to implement various vertex construction traits from Lyon.
pub struct VertexConstructor;

/// Enables the construction of a [`Vec2d`] when using a `FillTessellator`.
impl FillVertexConstructor<Vec2d> for VertexConstructor {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vec2d {
        vertex.position().convert()
    }
}

/// Enables the construction of a [`Vec2d`] when using a `StrokeTessellator`.
impl StrokeVertexConstructor<Vec2d> for VertexConstructor {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vec2d {
        vertex.position().convert()
    }
}
