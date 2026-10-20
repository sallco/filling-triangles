mod framebuffer;
mod fragment;
mod line;
mod primitive;
mod rasterizer;
mod shaders;
mod triangle;
mod uniforms;
mod vertex;

use framebuffer::Framebuffer;
use primitive::primitive_assembly;
use rasterizer::rasterize;
use raylib::prelude::*;
use shaders::{fragment_shader, vertex_shader};
use triangle::TriangleMode;
use uniforms::Uniforms;
use vertex::Vertex;

fn render(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    mode: TriangleMode,
) {
    // 1. Procesamiento de vértices
    let transformed_vertices: Vec<Vertex> = vertex_array
        .iter()
        .map(|vertex| vertex_shader(vertex, uniforms))
        .collect();

    // 2. Ensamblaje de primitivas
    let triangles = primitive_assembly(&transformed_vertices);

    // 3. Rasterización
    let fragments = rasterize(&triangles, mode);

    // 4. Procesamiento de fragmentos
    for fragment in fragments {
        let color = fragment_shader(&fragment);

        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            color,
        );
    }
}

fn main() {
    let (mut window, thread) = raylib::init()
        .size(800, 600)
        .title("Primer triangulo")
        .build();

    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(800, 600);
    framebuffer.set_background_color(Vector3::new(0.0, 0.0, 0.0));
    framebuffer.init_texture(&mut window, &thread);

    let positions = [
        Vector3::new(300.0, 200.0, 0.0),
        Vector3::new(400.0, 400.0, 0.0),
        Vector3::new(500.0, 200.0, 0.0),
    ];

    let vertex_array: Vec<Vertex> = positions
        .into_iter()
        .map(|position| {
            let mut vertex = Vertex::new_with_color(
                position,
                Vector3::new(1.0, 1.0, 1.0),
            );

            vertex.transformed_position = position;
            vertex
        })
        .collect();

    let uniforms = Uniforms {
        model_matrix: Matrix::identity(),
    };

    let mut current_mode = TriangleMode::Wireframe;

    while !window.window_should_close() {
        if window.is_key_pressed(KeyboardKey::KEY_ONE) {
            current_mode = TriangleMode::Wireframe;
        } else if window.is_key_pressed(KeyboardKey::KEY_TWO) {
            current_mode = TriangleMode::FilledBarycentric;
        } else if window.is_key_pressed(KeyboardKey::KEY_THREE) {
            current_mode = TriangleMode::FilledScanline;
        }

        framebuffer.clear();
        render(&mut framebuffer, &uniforms, &vertex_array, current_mode);
        framebuffer.swap_buffers(&mut window, &thread);
    }
}
