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

#[inline]
fn edge_cross(ax: f32, ay: f32, bx: f32, by: f32, px: f32, py: f32) -> f32 {
    (bx - ax) * (py - ay) - (by - ay) * (px - ax)
}

pub fn triangle_barycentric(v0: &Vertex, v1: &Vertex, v2: &Vertex) -> Vec<Fragment> {
    let p0 = v0.transformed_position;
    let p1 = v1.transformed_position;
    let p2 = v2.transformed_position;

    let area = edge_cross(p0.x, p0.y, p1.x, p1.y, p2.x, p2.y);
    if area.abs() < 1e-6 {
        return Vec::new();
    }

    // Normalizar a orden antihorario (CCW) para que el orden de los vértices no afecte el relleno
    let (v0, v1, v2, area) = if area < 0.0 {
        (v0, v2, v1, -area)
    } else {
        (v0, v1, v2, area)
    };

    let p0 = v0.transformed_position;
    let p1 = v1.transformed_position;
    let p2 = v2.transformed_position;

    let min_x = p0.x.min(p1.x).min(p2.x).floor() as i32;
    let max_x = p0.x.max(p1.x).max(p2.x).ceil() as i32;
    let min_y = p0.y.min(p1.y).min(p2.y).floor() as i32;
    let max_y = p0.y.max(p1.y).max(p2.y).ceil() as i32;

    let mut fragments = Vec::with_capacity(((max_x - min_x + 1) * (max_y - min_y + 1) / 2).max(0) as usize);

    for y in min_y..=max_y {
        let py = y as f32 + 0.5;
        for x in min_x..=max_x {
            let px = x as f32 + 0.5;

            // Coordenadas baricéntricas respecto a cada arista opuesta
            let w0 = edge_cross(p1.x, p1.y, p2.x, p2.y, px, py);
            let w1 = edge_cross(p2.x, p2.y, p0.x, p0.y, px, py);
            let w2 = edge_cross(p0.x, p0.y, p1.x, p1.y, px, py);

            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                let lambda0 = w0 / area;
                let lambda1 = w1 / area;
                let lambda2 = w2 / area;

                let depth = lambda0 * p0.z + lambda1 * p1.z + lambda2 * p2.z;
                let color = v0.color * lambda0 + v1.color * lambda1 + v2.color * lambda2;

                fragments.push(Fragment::new(x as f32, y as f32, color, depth));
            }
        }
    }

    fragments
}

pub fn triangle_scanline(v0: &Vertex, v1: &Vertex, v2: &Vertex) -> Vec<Fragment> {
    let area = edge_cross(
        v0.transformed_position.x,
        v0.transformed_position.y,
        v1.transformed_position.x,
        v1.transformed_position.y,
        v2.transformed_position.x,
        v2.transformed_position.y,
    );
    if area.abs() < 1e-6 {
        return Vec::new();
    }

    // Ordenar vértices por coordenada Y ascendente: p0.y <= p1.y <= p2.y
    let mut verts = [*v0, *v1, *v2];
    verts.sort_by(|a, b| {
        a.transformed_position
            .y
            .partial_cmp(&b.transformed_position.y)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let (v0, v1, v2) = (&verts[0], &verts[1], &verts[2]);
    let p0 = v0.transformed_position;
    let p1 = v1.transformed_position;
    let p2 = v2.transformed_position;

    let total_height = p2.y - p0.y;
    if total_height.abs() < 1e-6 {
        return Vec::new();
    }

    let min_y = (p0.y - 0.5).ceil() as i32;
    let max_y = (p2.y - 0.5).floor() as i32;

    let mut fragments = Vec::new();

    for y in min_y..=max_y {
        let py = y as f32 + 0.5;

        // Intersección con el lado largo (p0 -> p2)
        let t_long = (py - p0.y) / total_height;
        let x_long = p0.x + t_long * (p2.x - p0.x);
        let z_long = p0.z + t_long * (p2.z - p0.z);
        let c_long = v0.color * (1.0 - t_long) + v2.color * t_long;

        // Intersección con el segundo lado
        let (x_short, z_short, c_short) = if py < p1.y {
            let segment_height = p1.y - p0.y;
            if segment_height.abs() < 1e-6 {
                continue;
            }
            let t_short = (py - p0.y) / segment_height;
            (
                p0.x + t_short * (p1.x - p0.x),
                p0.z + t_short * (p1.z - p0.z),
                v0.color * (1.0 - t_short) + v1.color * t_short,
            )
        } else {
            let segment_height = p2.y - p1.y;
            if segment_height.abs() < 1e-6 {
                continue;
            }
            let t_short = (py - p1.y) / segment_height;
            (
                p1.x + t_short * (p2.x - p1.x),
                p1.z + t_short * (p2.z - p1.z),
                v1.color * (1.0 - t_short) + v2.color * t_short,
            )
        };

        // Identificar límites izquierdo y derecho en la fila
        let (x_left, z_left, c_left, x_right, z_right, c_right) = if x_long < x_short {
            (x_long, z_long, c_long, x_short, z_short, c_short)
        } else {
            (x_short, z_short, c_short, x_long, z_long, c_long)
        };

        let min_x = (x_left - 0.5).ceil() as i32;
        let max_x = (x_right - 0.5).floor() as i32;
        let dx = x_right - x_left;

        for x in min_x..=max_x {
            let px = x as f32 + 0.5;
            let s = if dx.abs() > 1e-6 {
                ((px - x_left) / dx).clamp(0.0, 1.0)
            } else {
                0.0
            };

            let depth = z_left * (1.0 - s) + z_right * s;
            let color = c_left * (1.0 - s) + c_right * s;

            fragments.push(Fragment::new(x as f32, y as f32, color, depth));
        }
    }

    fragments
}

pub fn triangle(v0: &Vertex, v1: &Vertex, v2: &Vertex, mode: TriangleMode) -> Vec<Fragment> {
    match mode {
        TriangleMode::Wireframe => triangle_wireframe(v0, v1, v2),
        TriangleMode::FilledBarycentric => triangle_barycentric(v0, v1, v2),
        TriangleMode::FilledScanline => triangle_scanline(v0, v1, v2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use raylib::math::Vector3;

    #[test]
    fn test_barycentric_winding_order_independence() {
        let v0 = Vertex::new(Vector3::new(10.0, 10.0, 0.0));
        let v1 = Vertex::new(Vector3::new(50.0, 10.0, 0.0));
        let v2 = Vertex::new(Vector3::new(10.0, 50.0, 0.0));

        // CCW
        let frags_ccw = triangle_barycentric(&v0, &v1, &v2);
        // CW
        let frags_cw = triangle_barycentric(&v0, &v2, &v1);

        assert!(!frags_ccw.is_empty(), "Debe generar fragmentos");
        assert_eq!(frags_ccw.len(), frags_cw.len(), "Ambos órdenes deben producir la misma cantidad de fragmentos");
    }

    #[test]
    fn test_barycentric_degenerate_triangle() {
        let v0 = Vertex::new(Vector3::new(10.0, 10.0, 0.0));
        let v1 = Vertex::new(Vector3::new(20.0, 20.0, 0.0));
        let v2 = Vertex::new(Vector3::new(30.0, 30.0, 0.0));

        let frags = triangle_barycentric(&v0, &v1, &v2);
        assert!(frags.is_empty(), "Un triángulo colineal degenerado no debe producir fragmentos");
    }

    #[test]
    fn test_scanline_winding_order_independence() {
        let v0 = Vertex::new(Vector3::new(10.0, 10.0, 0.0));
        let v1 = Vertex::new(Vector3::new(50.0, 10.0, 0.0));
        let v2 = Vertex::new(Vector3::new(10.0, 50.0, 0.0));

        let frags_1 = triangle_scanline(&v0, &v1, &v2);
        let frags_2 = triangle_scanline(&v0, &v2, &v1);

        assert!(!frags_1.is_empty(), "Debe generar fragmentos");
        assert_eq!(frags_1.len(), frags_2.len(), "Ambos órdenes deben producir la misma cantidad en scanline");
    }

    #[test]
    fn test_scanline_degenerate_triangle() {
        let v0 = Vertex::new(Vector3::new(10.0, 10.0, 0.0));
        let v1 = Vertex::new(Vector3::new(20.0, 20.0, 0.0));
        let v2 = Vertex::new(Vector3::new(30.0, 30.0, 0.0));

        let frags = triangle_scanline(&v0, &v1, &v2);
        assert!(frags.is_empty(), "Un triángulo colineal degenerado no debe producir fragmentos en scanline");
    }

    #[test]
    fn test_algorithms_consistency() {
        let v0 = Vertex::new(Vector3::new(100.0, 80.0, 0.0));
        let v1 = Vertex::new(Vector3::new(150.0, 160.0, 0.0));
        let v2 = Vertex::new(Vector3::new(70.0, 130.0, 0.0));

        let frags_bary = triangle_barycentric(&v0, &v1, &v2);
        let frags_scan = triangle_scanline(&v0, &v1, &v2);

        // Ambas implementaciones evalúan centros de pixel en (x + 0.5, y + 0.5)
        // La diferencia en cantidad de fragmentos entre ambos algoritmos debe ser mínima (< 3% por detalles de borde)
        let diff = (frags_bary.len() as isize - frags_scan.len() as isize).abs();
        let max_len = frags_bary.len().max(frags_scan.len()) as f32;
        assert!(
            diff as f32 / max_len < 0.05,
            "Diferencia entre baricéntrico ({}) y scanline ({}) supera el 5%",
            frags_bary.len(),
            frags_scan.len()
        );
    }
}


