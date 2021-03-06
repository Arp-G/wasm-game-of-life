mod utils;

use wasm_bindgen::prelude::*;
extern crate js_sys; // Exposes bindings for all JS global objects

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

extern crate web_sys;
use web_sys::console;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

// Implement a timer struct to check how much time a method took
// W eare using the `console.time` and `console.timeEnd` from javascript here.
pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

// We implement the `Drop` trait here so the `drop` method will be called whenever the `timer` instance
// goes out of scope this will trigger the js function `console.timeEnd` to print the time taken
impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
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
impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

fn get_random_cell() -> Cell {
    if js_sys::Math::random() < 0.5 {
        Cell::Alive
    } else {
        Cell::Dead
    }
}

// This `impl` block is annotated with #[wasm_bindgen] so that it can be called by JavaScript.
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

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }

    /// Sets the universe to a random state
    pub fn randomize(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                self.cells[idx] = get_random_cell();
            }
        }
    }

    /// Resets the universe to all dead cells
    pub fn reset(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                self.cells[idx] = Cell::Dead;
            }
        }
    }

    // Compute the next generation of the universe
    pub fn tick(&mut self) {

        // To track execution time for every tick
        let _timer = Timer::new("Universe::tick");

        let mut next = self.cells.clone(); // Next generation

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_count(row, col);

                // log!(
                //     "cell[{}, {}] is initially {:?} and has {} live neighbors",
                //     row,
                //     col,
                //     cell,
                //     live_neighbours
                // );

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

                // log!("    it becomes {:?}", next_cell);
                next[idx] = next_cell;
            }
        }
        self.cells = next; // Update generation
    }

    // Constructor to initializes the universe with an interesting pattern of live and dead cells
    pub fn new() -> Universe {
        // Init hook to log rust panic to browser console
        utils::set_panic_hook();

        let width = 64;
        let height = 64;

        let cells = (0..width * height).map(|_| get_random_cell()).collect();

        log!("Init Universe from wasm");

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
                let symbol = if cell == Cell::Dead { '???' } else { '???' };
                write!(f, "{}", symbol)?; // '?' unwraps Result<V> and return V or return Err in case of error
            }
            write!(f, "\n")?; // Line break for rows
        }
        Ok(())
    }
}

// Functions for testing
// Rust-generated WebAssembly functions cannot return borrowed references.
// So created a new `impl Universe` without the #[wasm_bindgen] attribute
impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}
