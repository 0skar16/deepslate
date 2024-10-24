mod section;
mod block;
use std::collections::HashMap;

use bitcode::{Decode, Encode};
pub use section::*;
pub use block::*;

#[derive(Clone, PartialEq, Debug, Encode, Decode, Default, Copy)]
pub struct ChunkEntry {
    pub data_start: u64,
    pub len: u32,
    pub original_len: u32,
    pub compression: ChunkCompression,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode, Default, Copy)]
pub enum ChunkCompression {
    #[default]
    None,
    LZ4,
    Zstd,
}

#[derive(Debug, Encode, Decode, Clone, PartialEq)]
pub struct Chunk {
    pub sections: Vec<Option<Section>>,
    pub block_entities: HashMap<i32, BlockEntity>,
    pub heightmap_mask: u32,
    pub heightmaps: Vec<Vec<u8>>,
}

#[derive(Debug, Encode, Decode, Clone, PartialEq)]
pub struct BlockEntity {
    pub block_entity_id: Option<String>,
    pub nbt_data: Vec<u8>,
}
