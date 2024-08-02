// Importa las funciones y estructuras necesarias desde los módulos locales y bibliotecas externas
use crate::app::{run_loop, App};
use crate::celulas::LifeCell;
use opengl_graphics::GlGraphics;
use piston_window::*;

// Declara los módulos locales
mod app;
mod grid;
mod celulas;

// Función para contar el número de células vivas en un vector de LifeCell
fn get_num_alive(cells: &Vec<LifeCell>) -> usize {
    cells
        .iter() // Itera sobre el vector de células
        .filter(|c| c.alive) // Filtra solo las células que están vivas
        .collect::<Vec<&LifeCell>>() // Recoge las células vivas en un nuevo vector
        .len() // Cuenta el número de células vivas
}

fn main() {
    // Define la versión de OpenGL a utilizar
    let opengl = OpenGL::V4_1;

    // Crea una ventana de Piston con el título "RGoL" y tamaño 800x600 píxeles
    // Usa la versión de OpenGL definida y configura la ventana para salir al presionar 'Esc'
    let mut window: PistonWindow = WindowSettings::new("RGoL", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Inicializa un objeto GlGraphics con la configuración de OpenGL
    let gl = GlGraphics::new(opengl);

    // Define la cantidad de células en el eje x e y (400x400 células)
    let x_cells = 400;
    let y_cells = 400;
    
    // Crea una nueva instancia de App con las dimensiones de células y la configuración gráfica
    let mut app = App::new(gl, x_cells, y_cells);
    
    // Define la probabilidad de que una célula esté viva al azar (50%)
    let live_prob: f64 = 0.50;
    
    // Imprime el número de células vivas antes de la randomización
    println!(
        "Num alive before randomize: {}",
        get_num_alive(&app.grid.cells)
    );
    
    // Randomiza las células en la cuadrícula según la probabilidad definida
    app.grid.randomize(live_prob);
    
    // Imprime el número de células vivas después de la randomización
    println!(
        "Num alive after randomize: {}",
        get_num_alive(&app.grid.cells)
    );
    
    // Llama a la función run_loop para iniciar el bucle principal del juego,
    // actualizando y renderizando el estado del juego en la ventana
    run_loop(&mut app, &mut window);
}
