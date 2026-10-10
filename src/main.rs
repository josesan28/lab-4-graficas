use raylib::prelude::*;

mod line;
mod obj;

use line::triangle;
use obj::load_obj;

const ANCHO_VENTANA: i32 = 800;
const ALTO_VENTANA: i32 = 600;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let modelo = load_obj("assets/nave-graficas.obj")?;

    for (indice, vertice) in modelo.vertices.iter().enumerate() {
        println!(
            "Vértice {}: ({}, {}, {})",
            indice + 1,
            vertice.x,
            vertice.y,
            vertice.z
        );
    }

    println!("Vértices cargados: {}", modelo.vertices.len());
    println!("Índices cargados: {}", modelo.indices.len());

    if let [indice_a, indice_b, indice_c, ..] = modelo.indices.as_slice() {
        println!(
            "Primer triángulo: {:?}, {:?}, {:?}",
            modelo.vertices[*indice_a], modelo.vertices[*indice_b], modelo.vertices[*indice_c]
        );
    }

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

    Ok(())
}
