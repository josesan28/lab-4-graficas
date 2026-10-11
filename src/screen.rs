use raylib::prelude::{Vector2, Vector3};

pub fn ajustar_a_ventana(
    vertices: &[Vector3],
    ancho_ventana: i32,
    alto_ventana: i32,
    margen: f32,
) -> Vec<Vector2> {
    if vertices.is_empty() {
        return Vec::new();
    }

    let mut minimo_x = f32::INFINITY;
    let mut maximo_x = f32::NEG_INFINITY;
    let mut minimo_y = f32::INFINITY;
    let mut maximo_y = f32::NEG_INFINITY;

    for vertice in vertices {
        minimo_x = minimo_x.min(vertice.x);
        maximo_x = maximo_x.max(vertice.x);
        minimo_y = minimo_y.min(vertice.y);
        maximo_y = maximo_y.max(vertice.y);
    }

    let ancho_modelo = maximo_x - minimo_x;
    let alto_modelo = maximo_y - minimo_y;
    let ancho_disponible = ancho_ventana as f32 - margen * 2.0;
    let alto_disponible = alto_ventana as f32 - margen * 2.0;

    let escala_x = if ancho_modelo > 0.0 {
        ancho_disponible / ancho_modelo
    } else {
        f32::INFINITY
    };
    let escala_y = if alto_modelo > 0.0 {
        alto_disponible / alto_modelo
    } else {
        f32::INFINITY
    };
    let escala = escala_x.min(escala_y);
    let escala = if escala.is_finite() { escala } else { 1.0 };

    let centro_modelo_x = (minimo_x + maximo_x) / 2.0;
    let centro_modelo_y = (minimo_y + maximo_y) / 2.0;
    let centro_pantalla_x = ancho_ventana as f32 / 2.0;
    let centro_pantalla_y = alto_ventana as f32 / 2.0;

    vertices
        .iter()
        .map(|vertice| {
            Vector2::new(
                (vertice.x - centro_modelo_x) * escala + centro_pantalla_x,
                centro_pantalla_y - (vertice.y - centro_modelo_y) * escala,
            )
        })
        .collect()
}
