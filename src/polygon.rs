use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::line::line;

pub fn draw_poligon(
    framebuffer: &mut Framebuffer,
    vertices: &[Vector2],
){
    let n = vertices.len();
    if n < 3 {
        return;
    }

    for i in 0..n {
        let start = vertices[i];
        let end = vertices[(i + 1) % n];
        line(framebuffer, start, end);
    }
} 
