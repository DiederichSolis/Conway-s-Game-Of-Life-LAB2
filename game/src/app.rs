use crate::grid::Grid;
use graphics::color::BLACK;
use graphics::Graphics;
use opengl_graphics::GlGraphics;
use piston::{EventSettings, Events, RenderArgs, UpdateArgs};
use piston_window::{PistonWindow, RenderEvent, UpdateEvent};

// Codigo tomado y referenciado de : https://medium.com/@mfriedrich/get-started-with-graphics-programming-in-rust-d98c26e41e5f

pub(crate) struct App {
    pub(crate) gl: GlGraphics,
    pub(crate) x_cells: u32,
    pub(crate) y_cells: u32,
    pub(crate) grid: Grid, // Cuadrícula que representa el área de dibujo
}

impl App {
    pub(crate) fn new(gl: GlGraphics, x_cells: u32, y_cells: u32) -> Self {
        Self {
            gl,
            x_cells,
            y_cells,
            grid: Grid::new(x_cells, y_cells), // Inicializa la cuadrícula con las dimensiones especificadas
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        self.grid.update(); // Actualiza el estado de la cuadrícula
        self.gl.draw(args.viewport(), |c, g| {
            g.clear_color(GRAY); // Limpia el fondo con color negro

            // Dibuja cada celda en la cuadrícula
            self.grid.cells.iter().for_each(|cell| {
                cell.rect.draw(cell.corners, &c.draw_state, c.transform, g);
            });
        });
    }

    fn update(&mut self, args: &UpdateArgs) {}
}

pub(crate) fn run_loop(app: &mut App, w: &mut PistonWindow) {
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(w) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
