use super::bit_map::{BitItem, BitMap};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Universe {
    width: usize,
    height: usize,
    cell: BitMap,
}

#[wasm_bindgen]
impl Universe {
    fn _get_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    pub fn get_index(&self, row: usize, col: usize) -> BitItem {
        let index = self._get_index(row, col);
        self.cell.get_index(index)
    }

    pub fn set_index(&mut self, row: usize, col: usize, value: BitItem) {
        let index = self._get_index(row, col);
        self.cell.set_index(index, value);
    }

    pub fn toggle(&mut self, row: usize, col: usize) {
        let state = self.get_index(row, col);
        match state {
            BitItem::ONE => self.set_index(row, col, BitItem::ZERO),
            BitItem::ZERO => self.set_index(row, col, BitItem::ONE),
        }
    }

    pub fn live_neighbor_count(&self, row: usize, column: usize) -> usize {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;

                count += match self.get_index(neighbor_row, neighbor_col) {
                    BitItem::ZERO => 0,
                    BitItem::ONE => 1,
                }
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cell.clone();
        for row in 0..self.height as usize {
            for col in 0..self.width as usize {
                let idx = self._get_index(row, col);
                let state = next.get_index(idx);
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_state = match (state, live_neighbors) {
                    (BitItem::ONE, x) if x < 2 => BitItem::ZERO,
                    (BitItem::ONE, 2 | 3) => BitItem::ONE,
                    (BitItem::ONE, x) if x > 3 => BitItem::ZERO,
                    (BitItem::ZERO, 3) => BitItem::ONE,
                    (s, _) => s,
                };
                next.set_index(idx, next_state);
            }
        }
        self.cell = next;
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new(height: usize, width: usize) -> Self {
        let mut s = Self {
            height,
            width,
            cell: BitMap::new(height * width),
        };
        s.init();
        s
    }

    pub fn init(&mut self) {
        for r in 0..self.width {
            for c in 0..self.height {
                // let idx = self._get_index(r, c);
                if js_sys::Math::random() >= 0.5 {
                    self.set_index(r, c, BitItem::ONE);
                } else {
                    self.set_index(r, c, BitItem::ZERO);
                }
            }
        }
    }

    pub fn reset(&mut self){
        self.cell.reset();
    }
}
