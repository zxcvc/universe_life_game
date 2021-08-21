use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug)]
pub enum BitItem {
    ZERO = 0,
    ONE = 1,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct BitMap {
    bit_v: Vec<u8>,
}

use std::{mem, ops::Not};
#[wasm_bindgen]
impl BitMap {
    pub fn new(size: usize) -> Self {
        let size = size / (mem::size_of::<u8>() * 8) + 1;
        let v = vec![0; size];
        Self { bit_v: v }
    }
    
    pub fn reset(&mut self){
        for it in 0..self.bit_v.len(){
            self.bit_v[it] = 0;
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.bit_v.as_ptr()
    }

    pub fn get_index(&self, index: usize) -> BitItem {
        let index_cell = index / 8;
        let prx = index % 8;

        if (self.bit_v[index_cell] & (0b10000000u8 >> prx)) == 0 {
            BitItem::ZERO
        } else {
            BitItem::ONE
        }
    }

    pub fn set_index(&mut self, index: usize, value: BitItem) {
        let index_cell = index / 8;
        let prx = index % 8;
        let mut base = 0b10000000u8 >> prx;

        match value {
            BitItem::ZERO => {
                base = base.not();
                self.bit_v[index_cell] &= base;
            }
            BitItem::ONE => {
                self.bit_v[index_cell] |= base;
            }
        }
    }
}
