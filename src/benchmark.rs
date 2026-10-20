use std::hint::black_box;
use std::time::{Duration, Instant};
use raylib::math::Vector3;
use crate::rasterizer::rasterize;
use crate::triangle::TriangleMode;
use crate::vertex::Vertex;

pub struct TestCase {
    pub name: &'static str,
    pub description: &'static str,
    pub vertices: [Vertex; 3],
}

pub fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            name: "Triángulo Estándar",
            description: "Triángulo mediano centrado en pantalla",
            vertices: [
                Vertex::new_with_color(Vector3::new(300.0, 200.0, 0.0), Vector3::new(1.0, 0.0, 0.0)),
                Vertex::new_with_color(Vector3::new(400.0, 450.0, 0.0), Vector3::new(0.0, 1.0, 0.0)),
                Vertex::new_with_color(Vector3::new(500.0, 200.0, 0.0), Vector3::new(0.0, 0.0, 1.0)),
            ],
        },
        TestCase {
            name: "Triángulo Pequeño",
            description: "Triángulo de 20x20 píxeles",
            vertices: [
                Vertex::new(Vector3::new(100.0, 100.0, 0.0)),
                Vertex::new(Vector3::new(120.0, 100.0, 0.0)),
                Vertex::new(Vector3::new(110.0, 120.0, 0.0)),
            ],
        },
        TestCase {
            name: "Triángulo Grande",
            description: "Triángulo de gran cobertura (600x450 píxeles)",
            vertices: [
                Vertex::new(Vector3::new(100.0, 50.0, 0.0)),
                Vertex::new(Vector3::new(700.0, 150.0, 0.0)),
                Vertex::new(Vector3::new(400.0, 550.0, 0.0)),
            ],
        },
        TestCase {
            name: "Borde Horizontal",
            description: "Triángulo con arista superior perfectamente horizontal",
            vertices: [
                Vertex::new(Vector3::new(200.0, 150.0, 0.0)),
                Vertex::new(Vector3::new(600.0, 150.0, 0.0)),
                Vertex::new(Vector3::new(400.0, 400.0, 0.0)),
            ],
        },
        TestCase {
            name: "Borde Vertical",
            description: "Triángulo rectángulo con arista izquierda perfectamente vertical",
            vertices: [
                Vertex::new(Vector3::new(250.0, 150.0, 0.0)),
                Vertex::new(Vector3::new(250.0, 450.0, 0.0)),
                Vertex::new(Vector3::new(550.0, 450.0, 0.0)),
            ],
        },
        TestCase {
            name: "Orden Invertido (CW)",
            description: "Mismos vértices del estándar pero con orientación horaria",
            vertices: [
                Vertex::new_with_color(Vector3::new(300.0, 200.0, 0.0), Vector3::new(1.0, 0.0, 0.0)),
                Vertex::new_with_color(Vector3::new(500.0, 200.0, 0.0), Vector3::new(0.0, 0.0, 1.0)),
                Vertex::new_with_color(Vector3::new(400.0, 450.0, 0.0), Vector3::new(0.0, 1.0, 0.0)),
            ],
        },
        TestCase {
            name: "Parcialmente Fuera",
            description: "Vértices que se extienden más allá de los bordes del framebuffer",
            vertices: [
                Vertex::new(Vector3::new(-100.0, 200.0, 0.0)),
                Vertex::new(Vector3::new(400.0, -50.0, 0.0)),
                Vertex::new(Vector3::new(900.0, 500.0, 0.0)),
            ],
        },
        TestCase {
            name: "Colineal (Degenerado)",
            description: "Tres puntos colineales con área cero",
            vertices: [
                Vertex::new(Vector3::new(100.0, 100.0, 0.0)),
                Vertex::new(Vector3::new(200.0, 200.0, 0.0)),
                Vertex::new(Vector3::new(300.0, 300.0, 0.0)),
            ],
        },
    ]
}

fn measure_mode(
    triangles: &[[Vertex; 3]],
    mode: TriangleMode,
    iterations: usize,
    rounds: usize,
) -> (Duration, usize) {
    let mut round_times = Vec::with_capacity(rounds);
    let mut fragment_count = 0;

    for _ in 0..rounds {
        let start = Instant::now();
        for _ in 0..iterations {
            let frags = rasterize(black_box(triangles), mode);
            fragment_count = frags.len();
            black_box(frags);
        }
        round_times.push(start.elapsed());
    }

    round_times.sort();
    let median = round_times[rounds / 2];
    (median, fragment_count)
}

pub fn run_benchmark() {
    println!("==========================================================================");
    println!("       BENCHMARK DE RASTERIZACIÓN DE TRIÁNGULOS (100 ITERACIONES)        ");
    println!("==========================================================================");

    let cases = get_test_cases();
    let iterations = 100;
    let rounds = 5;

    for case in &cases {
        println!("\n--- Caso: {} ({}) ---", case.name, case.description);
        let triangles = [case.vertices];

        let (time_wire, frags_wire) = measure_mode(&triangles, TriangleMode::Wireframe, iterations, rounds);
        let (time_bary, frags_bary) = measure_mode(&triangles, TriangleMode::FilledBarycentric, iterations, rounds);
        let (time_scan, frags_scan) = measure_mode(&triangles, TriangleMode::FilledScanline, iterations, rounds);

        println!(
            "  - Wireframe:    {:>10.2?} ({} fragmentos)",
            time_wire, frags_wire
        );
        println!(
            "  - Baricéntrico: {:>10.2?} ({} fragmentos)",
            time_bary, frags_bary
        );
        println!(
            "  - Scanline:     {:>10.2?} ({} fragmentos)",
            time_scan, frags_scan
        );

        if frags_bary > 0 && frags_scan > 0 {
            let ratio = time_bary.as_secs_f64() / time_scan.as_secs_f64();
            if ratio > 1.0 {
                println!("  -> Scanline es {:.2}x más rápido que Baricéntrico", ratio);
            } else {
                println!("  -> Baricéntrico es {:.2}x más rápido que Scanline", 1.0 / ratio);
            }
        }
    }

    println!("\n==========================================================================");
    println!("Benchmark finalizado con éxito.");
}
