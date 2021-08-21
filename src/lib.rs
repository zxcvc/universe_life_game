mod bit_map;
mod universe;

pub use bit_map::{BitItem, BitMap};
pub use universe::Universe;


#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;