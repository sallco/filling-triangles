use raylib::math::Matrix;

#[derive(Debug, Clone)]
pub struct Uniforms {
    pub model_matrix: Matrix,
}

impl Uniforms {
    pub fn new(model_matrix: Matrix) -> Self {
        Self { model_matrix }
    }
}
