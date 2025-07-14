use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::fillpolygon::fill_polygon;
use crate::polygon::draw_poligon;

pub fn polygon_3(
    framebuffer: &mut Framebuffer,
) {
    let start_vertices = vec![
            Vector2::new(377.0, 249.0),
            Vector2::new(411.0, 197.0),
            Vector2::new(436.0, 249.0),
        ];

        framebuffer.set_current_color(Color::RED);
        fill_polygon(framebuffer, &start_vertices, &[]);

        framebuffer.set_current_color(Color::WHITE);
        draw_poligon(framebuffer, &start_vertices);
}