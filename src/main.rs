use raylib::prelude::*;

const ANCHO_VENTANA: i32 = 800;
const ALTO_VENTANA: i32 = 600;

fn imprimir_vertice(v: Vector3) {
    println!("x = {}", v.x);
    println!("y = {}", v.y);
    println!("z = {}", v.z);
}

fn dibujar_punto(dibujo: &mut RaylibDrawHandle, posicion: Vector2, color: Color) {
    dibujo.draw_pixel_v(posicion, color);
}

fn main() {
    let punto = Vector2::new(100.0, 200.0);
    let vertice = Vector3::new(3.0, 5.0, -2.0);

    imprimir_vertice(vertice);

    let (mut raylib, hilo) = raylib::init()
        .size(ANCHO_VENTANA, ALTO_VENTANA)
        .title("Vértices y vectores")
        .build();

    raylib.set_target_fps(60);

    while !raylib.window_should_close() {
        let mut dibujo = raylib.begin_drawing(&hilo);
        dibujo.clear_background(Color::RAYWHITE);
        dibujar_punto(&mut dibujo, punto, Color::BLACK);
    }
}
