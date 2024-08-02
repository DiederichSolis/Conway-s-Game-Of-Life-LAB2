use graphics::color::{BLACK, RED};
use graphics::types::Color;
use graphics::Rectangle;

/// Representa una célula en la cuadrícula de la simulación.
///
/// La célula puede estar viva o muerta y tiene información sobre su estado actual y próximo,
/// así como el color que representa cada estado.
pub struct LifeCell {
    /// Indica si la célula está viva (`true`) o muerta (`false`).
    pub(crate) alive: bool,
    /// El rectángulo que representa visualmente la célula.
    pub(crate) rect: Rectangle,
    /// Coordenadas de las esquinas del rectángulo que representa la célula.
    pub(crate) corners: [f64; 4],
    /// Índices de las celdas vecinas en la cuadrícula.
    pub(crate) neighbor_indices: Vec<usize>,
    /// El estado actual de la célula.
    current_state: bool,
    /// El estado siguiente de la célula después de la actualización.
    next_state: bool,
    /// Color de la célula cuando está viva.
    alive_color: [f32; 4],
    /// Color de la célula cuando está muerta.
    dead_color: [f32; 4],
}

impl LifeCell {
    /// Crea una nueva instancia de `LifeCell`.
    ///
    /// # Parámetros
    /// - `alive`: Estado inicial de la célula (viva o muerta).
    /// - `corners`: Coordenadas de las esquinas del rectángulo que representa la célula.
    ///
    /// # Retorna
    /// Una nueva instancia de `LifeCell`.
    pub(crate) fn new(alive: bool, corners: [f64; 4]) -> Self {
        Self {
            alive,
            rect: Rectangle::new(if alive { RED } else { BLACK }),
            corners,
            neighbor_indices: vec![],
            current_state: alive,
            next_state: alive,
            alive_color: RED,
            dead_color: BLACK,
        }
    }

    /// Establece el estado de la célula (viva o muerta).
    ///
    /// # Parámetros
    /// - `live`: `true` si la célula debe estar viva, `false` si debe estar muerta.
    fn set_state(&mut self, live: bool) {
        self.alive = live;
        self.current_state = live;
        self.next_state = live;
    }

    /// Marca la célula como viva y actualiza el color del rectángulo.
    pub fn make_live(&mut self) {
        self.set_state(true);
        self.rect.color = self.alive_color;
    }

    /// Marca la célula como muerta y actualiza el color del rectángulo.
    pub fn make_dead(&mut self) {
        self.set_state(false);
        self.rect.color = self.dead_color;
    }

    /// Verifica si la célula necesita ser actualizada comparando su estado siguiente y actual.
    ///
    /// # Retorna
    /// `true` si la célula necesita ser actualizada, `false` en caso contrario.
    fn needs_update(&self) -> bool {
        self.next_state != self.current_state
    }

    /// Obtiene los índices de las celdas vecinas.
    ///
    /// # Retorna
    /// Una referencia a un vector de índices de las celdas vecinas.
    pub fn get_neighbor_indices(&self) -> &Vec<usize> {
        &self.neighbor_indices
    }

    /// Prepara la actualización del estado de la célula basado en el número de vecinos vivos.
    ///
    /// # Parámetros
    /// - `live_neighbors`: Número de vecinos vivos de la célula.
    pub fn prepare_update(&mut self, live_neighbors: u8) {
        // Calcula el próximo estado basado en las reglas del Juego de la Vida.
        if !(!self.current_state && live_neighbors < 3) {
            self.next_state = (self.current_state && live_neighbors == 2) || (live_neighbors == 3);
        }
    }

    /// Actualiza el estado de la célula según su estado siguiente.
    pub fn update(&mut self) {
        if self.needs_update() {
            if self.next_state {
                self.make_live();
            } else {
                self.make_dead();
            }
        }
    }
}
