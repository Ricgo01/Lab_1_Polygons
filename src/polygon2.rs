use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::fillpolygon::fill_polygon;
use crate::poligon::draw_poligon;

pub fn polygon_2(
    framebuffer: &mut Framebuffer,
) {
    let start_vertices = vec![
            Vector2::new(321.0, 335.0),
            Vector2::new(288.0, 286.0),
            Vector2::new(339.0, 251.0),
            Vector2::new(374.0, 302.0),
        ];

        framebuffer.set_current_color(Color::BLUE);
        fill_polygon(framebuffer, &start_vertices, &[]);

        framebuffer.set_current_color(Color::WHITE);
        draw_poligon(framebuffer, &start_vertices);
}