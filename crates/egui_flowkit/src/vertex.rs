use egui::{
    Color32,
    epaint::{Vertex, WHITE_UV},
};
use lyon_tessellation::{
    self as tess, FillVertex, FillVertexConstructor, StrokeVertex, StrokeVertexConstructor,
};

use crate::Convert;

/// The index type of a epaint [`Mesh`](epaint::Mesh).
type IndexType = u32;
/// Lyon's [`VertexBuffers`] generic data type defined for [`Vertex`].
pub type VertexBuffers = tess::VertexBuffers<Vertex, IndexType>;

/// Zero-sized type used to implement various vertex construction traits from Lyon.
pub struct VertexConstructor {
    pub color: Color32,
}

/// Enables the construction of a [`Vertex`] when using a `FillTessellator`.
impl FillVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        Vertex {
            uv: WHITE_UV,
            color: self.color,
            pos: vertex.position().convert(),
        }
    }
}

/// Enables the construction of a [`Vertex`] when using a `StrokeTessellator`.
impl StrokeVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
        Vertex {
            uv: WHITE_UV,
            color: self.color,
            pos: vertex.position().convert(),
        }
    }
}
