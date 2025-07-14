use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::fillpolygon::fill_polygon;
use crate::polygon::draw_poligon;
pub fn polygon_1(
    framebuffer: &mut Framebuffer,
) {
    let start_vertices = vec![
            Vector2::new(165.0, 380.0),
            Vector2::new(185.0, 360.0),
            Vector2::new(180.0, 330.0),
            Vector2::new(207.0, 345.0),
            Vector2::new(233.0, 330.0),
            Vector2::new(230.0, 360.0),
            Vector2::new(250.0, 380.0),
            Vector2::new(220.0, 385.0),
            Vector2::new(205.0, 410.0),
            Vector2::new(193.0, 383.0),
        ];

        framebuffer.set_current_color(Color::YELLOW);
        fill_polygon(framebuffer, &start_vertices, &[]);

        framebuffer.set_current_color(Color::WHITE);
        draw_poligon(framebuffer, &start_vertices);
}