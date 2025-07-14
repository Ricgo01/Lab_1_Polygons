mod framebuffer;
mod line;
mod polygon;
mod fillpolygon;
mod polygon1;
mod polygon2;
mod polygon3;
mod polygon4;

use framebuffer::Framebuffer;
use polygon1::polygon_1;
use polygon2::polygon_2;
use polygon3::polygon_3;
use polygon4::polygon_4;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    polygon_1(&mut framebuffer);
    polygon_2(&mut framebuffer);
    polygon_3(&mut framebuffer);
    polygon_4(&mut framebuffer);


    let output_file = "Lab1.bmp";
    println!("Guardando imagen como {}", output_file);
    framebuffer.render_to_file(output_file);
    println!("¡Listo!");
}
