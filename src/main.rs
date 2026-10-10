use raylib::prelude::*;

mod line;

use line::triangle;

const ANCHO_VENTANA: i32 = 800;
const ALTO_VENTANA: i32 = 600;

fn main() {
    let a = Vector2::new(320.0, 100.0);
    let b = Vector2::new(150.0, 350.0);
    let c = Vector2::new(500.0, 350.0);

    let (mut raylib, hilo) = raylib::init()
        .size(ANCHO_VENTANA, ALTO_VENTANA)
        .title("Laboratorio 4")
        .build();

    raylib.set_target_fps(60);

    while !raylib.window_should_close() {
        let mut dibujo = raylib.begin_drawing(&hilo);
        dibujo.clear_background(Color::RAYWHITE);
        triangle(&mut dibujo, a, b, c, Color::BLACK);
    }
}
