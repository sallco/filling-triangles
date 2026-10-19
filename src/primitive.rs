use crate::vertex::Vertex;

pub fn primitive_assembly(
    transformed_vertices: &[Vertex],
) -> Vec<[Vertex; 3]> {
    assert!(
        transformed_vertices.len() % 3 == 0,
        "La cantidad de vértices debe ser múltiplo de 3",
    );

    transformed_vertices
        .chunks_exact(3)
        .map(|group| {
            [
                group[0],
                group[1],
                group[2],
            ]
        })
        .collect()
}
