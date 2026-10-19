use crate::fragment::Fragment;
use crate::vertex::Vertex;

pub fn line(v0: &Vertex, v1: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let mut x0 = v0.transformed_position.x.round() as i32;
    let mut y0 = v0.transformed_position.y.round() as i32;
    let x1 = v1.transformed_position.x.round() as i32;
    let y1 = v1.transformed_position.y.round() as i32;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx - dy;
    let total_steps = dx.max(dy);
    let mut step = 0;

    loop {
        let t = if total_steps == 0 {
            0.0
        } else {
            step as f32 / total_steps as f32
        };

        let depth = v0.transformed_position.z + (v1.transformed_position.z - v0.transformed_position.z) * t;
        let color = v0.color * (1.0 - t) + v1.color * t;

        fragments.push(Fragment::new(x0 as f32, y0 as f32, color, depth));

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;

        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }

        if e2 < dx {
            err += dx;
            y0 += sy;
        }

        step += 1;
    }

    fragments
}
