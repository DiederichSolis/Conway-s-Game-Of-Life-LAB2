use crate::celulas::LifeCell;
use graphics::rectangle::rectangle_by_corners;
use rand::Rng;
use rayon::iter::*;

/// Espaciado entre las células en la cuadrícula.
const CELL_SPACING: f64 = 1.0;
/// Tamaño de cada célula en la cuadrícula.
const CELL_SIZE: f64 = 5.0;

/// Representa una cuadrícula de células para la simulación del Juego de la Vida.
pub struct Grid {
    /// Número de células en el eje X.
    x_cells: u32,
    /// Número de células en el eje Y.
    y_cells: u32,
    /// Número de generaciones transcurridas en la simulación.
    pub(crate) generation: u64,
    /// Vector que contiene todas las células en la cuadrícula.
    pub(crate) cells: Vec<LifeCell>,
}

impl Grid {
    /// Crea una nueva cuadrícula de células.
    ///
    /// # Parámetros
    /// - `x_cells`: Número de células en el eje X.
    /// - `y_cells`: Número de células en el eje Y.
    ///
    /// # Retorna
    /// Una nueva instancia de `Grid`.
    pub(crate) fn new(x_cells: u32, y_cells: u32) -> Self {
        Self {
            x_cells,
            y_cells,
            generation: 0,
            cells: Grid::create_cell_grid(x_cells, y_cells),
        }
    }

    /// Crea la cuadrícula de células inicial.
    ///
    /// # Parámetros
    /// - `x_cells`: Número de células en el eje X.
    /// - `y_cells`: Número de células en el eje Y.
    ///
    /// # Retorna
    /// Un vector de células (`LifeCell`).
    fn create_cell_grid(x_cells: u32, y_cells: u32) -> Vec<LifeCell> {
        let mut cell_grid = Vec::with_capacity((x_cells * y_cells) as usize);
        for x in 0..x_cells {
            for y in 0..y_cells {
                let top_left_x = (x as f64) * (CELL_SIZE + CELL_SPACING);
                let top_left_y = (y as f64) * (CELL_SIZE + CELL_SPACING);
                let bottom_right_x = top_left_x + CELL_SIZE;
                let bottom_right_y = top_left_y + CELL_SIZE;
                let corners = rectangle_by_corners(top_left_x, top_left_y, bottom_right_x, bottom_right_y);
                let cell = LifeCell::new(false, corners);
                cell_grid.push(cell);
            }
        }
        Grid::set_neighbors(x_cells, y_cells, &mut cell_grid);
        cell_grid
    }

    /// Establece los vecinos de cada célula en la cuadrícula.
    ///
    /// # Parámetros
    /// - `x_cells`: Número de células en el eje X.
    /// - `y_cells`: Número de células en el eje Y.
    /// - `cell_grid`: Vector mutable de células.
    fn set_neighbors(x_cells: u32, y_cells: u32, cell_grid: &mut Vec<LifeCell>) {
        for x in 0..x_cells {
            for y in 0..y_cells {
                let neighbor_idxs = Grid::get_neighbor_indices_for_cell(x, y, x_cells, y_cells);
                cell_grid[(x + y * x_cells) as usize].neighbor_indices = neighbor_idxs;
            }
        }
    }

    /// Obtiene los índices de los vecinos de una célula dada.
    ///
    /// # Parámetros
    /// - `x`: Coordenada X de la célula.
    /// - `y`: Coordenada Y de la célula.
    /// - `x_cells`: Número de células en el eje X.
    /// - `y_cells`: Número de células en el eje Y.
    ///
    /// # Retorna
    /// Un vector de índices de vecinos.
    fn get_neighbor_indices_for_cell(x: u32, y: u32, x_cells: u32, y_cells: u32) -> Vec<usize> {
        let mut neighbor_idxs: Vec<usize> = vec![];

        let left_x: i64 = (x as i64) - 1;
        let right_x: i64 = (x as i64) + 1;
        let top_y: i64 = (y as i64) - 1;
        let bottom_y: i64 = (y as i64) + 1;

        if left_x > -1 {
            neighbor_idxs.push((left_x + (y * x_cells) as i64) as usize);
        }

        if left_x > -1 && top_y > -1 {
            neighbor_idxs.push((left_x + top_y * (x_cells as i64)) as usize);
        }

        if top_y > -1 {
            neighbor_idxs.push((x as i64 + top_y * x_cells as i64) as usize);
        }

        if right_x < x_cells as i64 && top_y > -1 {
            neighbor_idxs.push((right_x + top_y * x_cells as i64) as usize);
        }

        if right_x < x_cells as i64 {
            neighbor_idxs.push((right_x + (y * x_cells) as i64) as usize);
        }

        if right_x < x_cells as i64 && bottom_y < y_cells as i64 {
            neighbor_idxs.push((right_x + bottom_y * x_cells as i64) as usize);
        }

        if bottom_y < y_cells as i64 {
            neighbor_idxs.push((x as i64 + bottom_y * x_cells as i64) as usize);
        }

        if left_x > -1 && bottom_y < y_cells as i64 {
            neighbor_idxs.push((left_x + bottom_y * x_cells as i64) as usize);
        }

        neighbor_idxs
    }

    /// Actualiza el estado de todas las células en la cuadrícula.
    ///
    /// # Retorna
    /// El número de la nueva generación.
    pub(crate) fn update(&mut self) -> u64 {
        let alive_grid = self.cells.par_iter().map(|cell| cell.alive).collect::<Vec<bool>>();

        self.cells.par_iter_mut().for_each(|cell| {
            let neighbor_idxs = cell.get_neighbor_indices();
            let live_neighbors: u8 = neighbor_idxs.iter().filter(|nidx| alive_grid[**nidx]).collect::<Vec<&usize>>().len() as u8;
            cell.prepare_update(live_neighbors);
        });

        self.cells.par_iter_mut().for_each(|cell| {
            cell.update();
        });

        self.generation += 1;
        self.generation
    }

    /// Reinicia la cuadrícula, estableciendo todas las células como muertas.
    pub(crate) fn reset(&mut self) {
        for x in 0..self.x_cells {
            for y in 0..self.y_cells {
                self.cells[(x + y * self.x_cells) as usize].make_dead();
            }
        }

        self.generation = 0;
    }

    /// Randomiza el estado de las células en la cuadrícula basado en una probabilidad.
    ///
    /// # Parámetros
    /// - `live_probability`: La probabilidad de que una célula esté viva.
    pub(crate) fn randomize(&mut self, live_probability: f64) {
        self.reset();

        let mut rng = rand::thread_rng();
        for x in 0..self.x_cells {
            for y in 0..self.y_cells {
                if rng.gen::<f64>() <= live_probability {
                    self.cells[(x + y * self.x_cells) as usize].make_live();
                }
            }
        }
    }
}
