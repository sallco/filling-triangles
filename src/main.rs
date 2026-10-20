mod benchmark;
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
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--benchmark-triangles" || arg == "-b" || arg == "--benchmark") {
        benchmark::run_benchmark();
        return;
    }

    let (mut window, thread) = raylib::init()
        .size(800, 600)
        .title("Laboratorio 5: Relleno de Triangulos")
        .build();

    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(800, 600);
    framebuffer.set_background_color(Vector3::new(0.05, 0.05, 0.08));
    framebuffer.init_texture(&mut window, &thread);

    let uniforms = Uniforms {
        model_matrix: Matrix::identity(),
    };

    let test_cases = benchmark::get_test_cases();
    let mut case_index = 0;
    let mut current_mode = TriangleMode::FilledBarycentric;

    while !window.window_should_close() {
        if window.is_key_pressed(KeyboardKey::KEY_ONE) {
            current_mode = TriangleMode::Wireframe;
        } else if window.is_key_pressed(KeyboardKey::KEY_TWO) {
            current_mode = TriangleMode::FilledBarycentric;
        } else if window.is_key_pressed(KeyboardKey::KEY_THREE) {
            current_mode = TriangleMode::FilledScanline;
        }

        if window.is_key_pressed(KeyboardKey::KEY_RIGHT) || window.is_key_pressed(KeyboardKey::KEY_SPACE) {
            case_index = (case_index + 1) % test_cases.len();
        } else if window.is_key_pressed(KeyboardKey::KEY_LEFT) {
            case_index = (case_index + test_cases.len() - 1) % test_cases.len();
        }

        let current_case = &test_cases[case_index];
        let vertex_array = current_case.vertices.to_vec();

        window.set_window_title(&thread, &format!(
            "Filling Triangles | Modo: {:?} | Caso [{}]: {}",
            current_mode,
            case_index + 1,
            current_case.name
        ));

        framebuffer.clear();
        render(&mut framebuffer, &uniforms, &vertex_array, current_mode);
        framebuffer.swap_buffers(&mut window, &thread);
    }
}

