mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(name);
}

/*
Type defination for every Cell in the universe
#[repr(u8)] -> Represent each cell as a single byte
*/
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    // Get 1D array index for a given 2D array index
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    // Return live neighbour count for a given cell
    fn live_neighbour_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        // [self.height - 1, 0, 1] -> Refers to top, itself and bottom rows
        // self.height - 1 instead of (row - 1) is done to avoid (0 - 1) case
        // it works since we have modulo. The module handles wrapping around edges
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbour_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbour_row, neighbor_col);
                count += self.cells[idx] as u8; // If cell at 'idx' is alive this will add 1 to count
            }
        }
        count
    }

    /// Public methods, exported to JavaScript.
    
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    // Compute the next generation of the universe
    pub fn tick(&mut self) {
        let mut next = self.cells.clone(); // Next generation

        for row in 0..self.height {
            for column in 0..self.width {
                let idx = self.get_index(row, column);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_count(row, column);

                let next_cell = match (cell, live_neighbours) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,

                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,

                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,

                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,

                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }
        self.cells = next; // Update generation
    }

    // Constructor to initializes the universe with an interesting pattern of live and dead cells
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    // Will use our implementation of the display trait to render a string
    // representing the universe
    pub fn render(&self) -> String {
        self.to_string()
    }
}

// Here, we implement the Display trait from Rust's standard library
// This allows us the diplay our universe to the user

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Chunk out every row of the universe
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?; // '?' unwraps Result<V> and return V or return Err in case of error
            }
            write!(f, "\n")?; // Line break for rows
        }
        Ok(())
    }
}