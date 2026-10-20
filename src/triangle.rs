use crate::fragment::Fragment;
use crate::line::line;
use crate::vertex::Vertex;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TriangleMode {
    Wireframe,
    FilledBarycentric,
    FilledScanline,
}

pub fn triangle_wireframe(v0: &Vertex, v1: &Vertex, v2: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    fragments.extend(line(v0, v1));
    fragments.extend(line(v1, v2));
    fragments.extend(line(v2, v0));

    fragments
}

pub fn triangle(v0: &Vertex, v1: &Vertex, v2: &Vertex, mode: TriangleMode) -> Vec<Fragment> {
    match mode {
        TriangleMode::Wireframe => triangle_wireframe(v0, v1, v2),
        TriangleMode::FilledBarycentric => {
            // Fallback temporal mientras se implementa el algoritmo baricéntrico
            triangle_wireframe(v0, v1, v2)
        }
        TriangleMode::FilledScanline => {
            // Fallback temporal mientras se implementa el algoritmo scanline
            triangle_wireframe(v0, v1, v2)
        }
    }
}

