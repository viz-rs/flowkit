use makepad_widgets::{Cx2d, DrawLine, Vec4};

use crate::vertex::VertexBuffers;

/// Draws a path with indices and vertices.
///
/// How does it work?
///
/// 1. Extracts 4 vertices' positions from two triangles.
///
/// | Indices   | Ordered       | Sequence      |
/// | --------- | ------------- | ------------- |
/// | `quad_2d` | `0 1 2 2 3 0` | `a b c c d a` |
/// | `buffers` | `3 2 0 3 0 1` | `d c a d a b` |
///
/// ```text
/// 0┌─────┐1   a┌─────┐b
///  │     │     │     │
/// 3└─────┘2   d└─────┘c
/// ```
///
/// | Index | Seq   | Indice | Vertex        | Corner       |
/// | ----- | ----- | ------ | ------------- | ------------ |
/// | 0     | 2,4   | a      | `vertices[a]` | top-left     |
/// | 1     | 5     | b      | `vertices[b]` | top-right    |
/// | 2     | 1     | c      | `vertices[c]` | bottom-right |
/// | 3     | 0,3   | d      | `vertices[d]` | bottom-left  |
///
/// 2. Draws two lines:
///
/// | Index | Line     |
/// | ----- | -------- |
/// | 0     | `a -> c` |
/// | 1     | `b -> d` |
pub trait DrawPath {
    fn draw_with(&mut self, cx: &mut Cx2d, buffers: VertexBuffers, color: Vec4, width: f64);
}

impl DrawPath for DrawLine {
    fn draw_with(&mut self, cx: &mut Cx2d, buffers: VertexBuffers, color: Vec4, width: f64) {
        let VertexBuffers { indices, vertices } = buffers;

        debug_assert!(vertices.len() >= 3);
        debug_assert_eq!(indices.len() % 3, 0);

        for chunks in indices.chunks(6) {
            let [d, c, a, _, _, b] = chunks[..] else {
                break;
            };

            let ap = vertices[a];
            let bp = vertices[b];
            let cp = vertices[c];
            let dp = vertices[d];

            self.draw_line_abs(cx, ap, cp, color, width);
            self.draw_line_abs(cx, bp, dp, color, width);
        }
    }
}
