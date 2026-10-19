use crate::fragment::Fragment;
use crate::line::line;
use crate::vertex::Vertex;

pub fn triangle(v0: &Vertex, v1: &Vertex, v2: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    fragments.extend(line(v0, v1));
    fragments.extend(line(v1, v2));
    fragments.extend(line(v2, v0));

    fragments
}
