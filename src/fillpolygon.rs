use crate::framebuffer::Framebuffer;
use raylib::prelude::Vector2;

pub fn fill_polygon(
    framebuffer: &mut Framebuffer,
    outer_polygon: &[Vector2],
    holes: &[&[Vector2]],
) {
    if outer_polygon.len() < 3 { return; }

    let mut min_y = outer_polygon[0].y;
    let mut max_y = outer_polygon[0].y;
    for vertex in &outer_polygon[1..] {
        min_y = min_y.min(vertex.y);
        max_y = max_y.max(vertex.y);
    }

    for y_f in min_y.floor() as i32..=max_y.ceil() as i32 {
        let y = y_f as f32;
        let mut all_intersections: Vec<f32> = Vec::new();
        
        let all_polygons = std::iter::once(outer_polygon).chain(holes.iter().cloned());

        for polygon in all_polygons {
            for i in 0..polygon.len() {
                let p1 = polygon[i];
                let p2 = polygon[(i + 1) % polygon.len()];

                if (p1.y <= y && p2.y > y) || (p2.y <= y && p1.y > y) {
                    if p1.y != p2.y {
                        let x_intersect = (y - p1.y) * (p2.x - p1.x) / (p2.y - p1.y) + p1.x;
                        all_intersections.push(x_intersect);
                    }
                }
            }
        }
        
        all_intersections.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        for i in (0..all_intersections.len()).step_by(2) {
            if i + 1 < all_intersections.len() {
                let x_start = all_intersections[i].round() as i32;
                let x_end = all_intersections[i + 1].round() as i32;

                for x in x_start..x_end {
                    framebuffer.set_pixel(x, y_f);
                }
            }
        }
    }
}