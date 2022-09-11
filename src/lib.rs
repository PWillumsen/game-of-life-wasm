mod utils;

use js_sys;
use std::{collections::HashSet, fmt, usize};
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

type Cell = (i32, i32);

#[wasm_bindgen]
pub struct Universe {
    width: i32,
    height: i32,
    alive_cells: HashSet<Cell>,
    alive_cells_buffer: HashSet<Cell>,
    new_alive: Vec<i32>,
    new_dead: Vec<i32>,
}

impl Universe {
    fn live_neighbor_count(&self, cell: &Cell) -> u8 {
        let mut count = 0;
        let (row, column) = cell;

        for delta_row in [row - 1, *row, row + 1].iter().cloned() {
            for delta_col in [column - 1, *column, column + 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                if self.alive_cells.contains(&(delta_row, delta_col)) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn get_cells(&self) -> &HashSet<Cell> {
        &self.alive_cells
        // &self.new_alive
    }

    pub fn set_cells(&mut self, cells: &[(i32, i32)]) {
        for (row, col) in cells.iter().cloned() {
            self.alive_cells.insert((row, col));
        }
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 32;
        let height = 32;

        let new_alive = vec![10, 13, 11, 13, 12, 13, 11, 11, 12, 12];

        Universe {
            width,
            height,
            alive_cells: HashSet::new(),
            alive_cells_buffer: HashSet::new(),
            // new_alive: Vec::new(),
            new_alive,
            new_dead: Vec::new(),
        }
    }

    pub fn tick(&mut self) {
        // Split up. Make simpler. Find differences in other function
        // do one loop, dont clear alive_cells just remove
        // back to fixedbitset

        let _timer = Timer::new("Universe::tick");
        self.new_alive.clear();
        self.new_dead.clear();
        self.alive_cells_buffer.clear();

        for cell in self.alive_cells.iter() {
            let live_neighbors = self.live_neighbor_count(cell);

            if (live_neighbors == 2) | (live_neighbors == 3) {
                console_log!("Adding ({:?}) neighbors = {}", cell, live_neighbors);
                self.alive_cells_buffer.insert(*cell);
            }
        }

        for row in 0..self.height {
            for col in 0..self.width {
                if row < 0 || row > self.height || col < 0 || col > self.width {
                    continue;
                }

                if self.alive_cells.contains(&(row, col)) {
                    continue;
                }
                let live_neighbors = self.live_neighbor_count(&(row, col));
                if live_neighbors == 3 {
                    console_log!("Adding ({},{}) neighbors = {}", row, col, live_neighbors);
                    self.alive_cells_buffer.insert((row, col));
                }
            }
        }

        for (row, col) in self.alive_cells_buffer.difference(&self.alive_cells) {
            self.new_alive.push(*row);
            self.new_alive.push(*col);
        }

        for (row, col) in self.alive_cells.difference(&self.alive_cells_buffer) {
            self.new_dead.push(*row);
            self.new_dead.push(*col);
        }

        console_log!("new alive: {:?}", self.new_alive);
        console_log!("new dead: {:?}", self.new_dead);

        self.alive_cells.clear();
        self.alive_cells = self.alive_cells_buffer.clone();
        console_log!("alive cells buffer: {:?}", self.alive_cells_buffer);
        console_log!("alive cells: {:?}", self.alive_cells);
    }

    pub fn clear(&mut self) {
        // self.new_dead = self.alive_cells.clone().into_iter().collect();
        self.alive_cells.clear();
    }

    pub fn set_width(&mut self, width: i32) {
        self.width = width;
        self.clear();
    }

    pub fn set_height(&mut self, height: i32) {
        self.height = height;
        self.clear();
    }

    pub fn toggle_cell(&mut self, row: i32, column: i32) {
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
    pub fn get_new_alive(&self) -> *const i32 {
        self.new_alive.as_ptr()
    }

    #[wasm_bindgen(js_name = getAliveLen)]
    pub fn get_new_alive_len(&self) -> usize {
        self.new_alive.len()
    }

    #[wasm_bindgen(js_name = getNewDead)]
    pub fn get_new_dead(&self) -> *const i32 {
        self.new_dead.as_ptr()
    }

    #[wasm_bindgen(js_name = getDeadLen)]
    pub fn get_new_dead_len(&self) -> usize {
        self.new_dead.len()
    }

    #[wasm_bindgen(js_name = getWidth)]
    pub fn get_width(&self) -> i32 {
        self.width
    }

    #[wasm_bindgen(js_name = getHeight)]
    pub fn get_height(&self) -> i32 {
        self.height
    }

    // #[wasm_bindgen(js_name = getAllAlive)]
    // pub fn cells(&self) -> *const Cell {
    //     self.alive_cells.as_ptr()
    // }
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
