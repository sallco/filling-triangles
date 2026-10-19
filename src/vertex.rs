use raylib::math::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vector3,
    pub color: Vector3,
    pub transformed_position: Vector3,
}

impl Vertex {
    pub fn new(position: Vector3) -> Self {
        Self {
            position,
            color: Vector3::new(1.0, 1.0, 1.0),
            transformed_position: position,
        }
    }

    pub fn new_with_color(position: Vector3, color: Vector3) -> Self {
        Self {
            position,
            color,
            transformed_position: position,
        }
    }
}
