use raylib::prelude::*;

mod line;
mod obj;
mod screen;

use line::triangle;
use obj::load_obj;
use screen::ajustar_a_ventana;

const ANCHO_VENTANA: i32 = 800;
const ALTO_VENTANA: i32 = 600;
const MARGEN: f32 = 50.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let modelo = load_obj("assets/nave-graficas.obj")?;
    let vertices_pantalla =
        ajustar_a_ventana(&modelo.vertices, ANCHO_VENTANA, ALTO_VENTANA, MARGEN);

    let (mut raylib, hilo) = raylib::init()
        .size(ANCHO_VENTANA, ALTO_VENTANA)
        .title("Laboratorio 4")
        .build();

    raylib.set_target_fps(60);

    while !raylib.window_should_close() {
        let mut dibujo = raylib.begin_drawing(&hilo);
        dibujo.clear_background(Color::RAYWHITE);

        for indices in modelo.indices.chunks_exact(3) {
            let a = vertices_pantalla[indices[0]];
            let b = vertices_pantalla[indices[1]];
            let c = vertices_pantalla[indices[2]];

            triangle(&mut dibujo, a, b, c, Color::BLACK);
        }
    }

    Ok(())
}
