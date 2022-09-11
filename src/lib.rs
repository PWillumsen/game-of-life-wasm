mod utils;

use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

type Cell = (u32, u32);

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    alive_cells: HashSet<Cell>,
    new_alive: Vec<u32>,
    new_dead: Vec<u32>,
}

impl Universe {
    // fn live_neighbor_count(&self, cell: &Cell) -> u8 {
    //     let mut count = 0;
    //     let (row, column) = cell;

    //     for delta_row in [row - 1, *row, row + 1].iter().cloned() {
    //         for delta_col in [column - 1, *column, column + 1].iter().cloned() {
    //             if delta_row == 0 && delta_col == 0 {
    //                 continue;
    //             }
    //             if self.alive_cells.contains(&(delta_row, delta_col)) {
    //                 count += 1;
    //             }
    //         }
    //     }
    //     count
    // }

    pub fn get_cells(&self) -> &HashSet<Cell> {
        &self.alive_cells
        // &self.new_alive
    }
    fn live_neighbor_count(&self, cell: Cell) -> u8 {
        let (row, column) = cell;

        let mut count = 0;

        let north = if row == 0 { self.height - 1 } else { row - 1 };

        let south = if row == self.height - 1 { 0 } else { row + 1 };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        if self.alive_cells.contains(&(north, west)) {
            count += 1;
        }
        if self.alive_cells.contains(&(north, column)) {
            count += 1;
        }
        if self.alive_cells.contains(&(north, east)) {
            count += 1;
        }
        if self.alive_cells.contains(&(row, west)) {
            count += 1;
        }
        if self.alive_cells.contains(&(row, east)) {
            count += 1;
        }
        if self.alive_cells.contains(&(south, west)) {
            count += 1;
        }
        if self.alive_cells.contains(&(south, column)) {
            count += 1;
        }
        if self.alive_cells.contains(&(south, east)) {
            count += 1;
        }

        count
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            self.alive_cells.insert((row, col));
        }
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 256;
        let height = 256;

        Universe {
            width,
            height,
            alive_cells: HashSet::new(),
            new_alive: Vec::new(),
            new_dead: Vec::new(),
        }
    }

    pub fn tick(&mut self) {
        // Split up. Make simpler. Find differences in other function
        // do one loop, dont clear alive_cells just remove

        // let _timer = Timer::new("Universe::tick");
        self.new_alive.clear();
        self.new_dead.clear();

        for row in 0..self.height {
            for col in 0..self.width {
                let live_neighbors = self.live_neighbor_count((row, col));
                if self.alive_cells.contains(&(row, col)) {
                    if (live_neighbors == 2) | (live_neighbors == 3) {
                        continue;
                    } else {
                        self.new_dead.push(row);
                        self.new_dead.push(col);
                    }
                } else {
                    if live_neighbors == 3 {
                        self.new_alive.push(row);
                        self.new_alive.push(col);
                    }
                }
            }
        }

        for i in (0..self.new_alive.len()).step_by(2) {
            let row = self.new_alive[i];
            let col = self.new_alive[i + 1];
            self.alive_cells.insert((row, col));
        }
        for i in (0..self.new_dead.len()).step_by(2) {
            let row = self.new_dead[i];
            let col = self.new_dead[i + 1];
            self.alive_cells.remove(&(row, col));
        }
    }

    pub fn clear(&mut self) {
        self.alive_cells.clear();
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.clear();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.clear();
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        if self.alive_cells.contains(&(row, column)) {
            self.alive_cells.remove(&(row, column));
            self.new_dead.push(row);
            self.new_dead.push(column);
        } else {
            self.alive_cells.insert((row, column));
            self.new_alive.push(row);
            self.new_alive.push(column);
        }
    }

    #[wasm_bindgen(js_name = getNewAlive)]
    pub fn get_new_alive(&self) -> *const u32 {
        self.new_alive.as_ptr()
    }

    #[wasm_bindgen(js_name = getAliveLen)]
    pub fn get_new_alive_len(&self) -> usize {
        self.new_alive.len()
    }

    #[wasm_bindgen(js_name = getNewDead)]
    pub fn get_new_dead(&self) -> *const u32 {
        self.new_dead.as_ptr()
    }

    #[wasm_bindgen(js_name = getDeadLen)]
    pub fn get_new_dead_len(&self) -> usize {
        self.new_dead.len()
    }

    #[wasm_bindgen(js_name = getWidth)]
    pub fn get_width(&self) -> u32 {
        self.width
    }

    #[wasm_bindgen(js_name = getHeight)]
    pub fn get_height(&self) -> u32 {
        self.height
    }
}

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}
