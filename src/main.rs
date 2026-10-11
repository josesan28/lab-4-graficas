use raylib::prelude::*;

mod line;
mod obj;

use line::triangle;
use obj::load_obj;

const ANCHO_VENTANA: i32 = 800;
const ALTO_VENTANA: i32 = 600;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let modelo = load_obj("assets/nave-graficas.obj")?;

    let (mut raylib, hilo) = raylib::init()
        .size(ANCHO_VENTANA, ALTO_VENTANA)
        .title("Laboratorio 4")
        .build();

    raylib.set_target_fps(60);

    while !raylib.window_should_close() {
        let mut dibujo = raylib.begin_drawing(&hilo);
        dibujo.clear_background(Color::RAYWHITE);

        for indices in modelo.indices.chunks_exact(3) {
            let vertice_a = modelo.vertices[indices[0]];
            let vertice_b = modelo.vertices[indices[1]];
            let vertice_c = modelo.vertices[indices[2]];

            let a = Vector2::new(vertice_a.x, vertice_a.y);
            let b = Vector2::new(vertice_b.x, vertice_b.y);
            let c = Vector2::new(vertice_c.x, vertice_c.y);

            triangle(&mut dibujo, a, b, c, Color::BLACK);
        }
    }

    Ok(())
}
