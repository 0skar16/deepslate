#![feature(iter_array_chunks)]
use bitcode::{Decode, Encode};

pub mod reader;
pub mod writer;
pub mod chunk;
mod static_enums;
use chunk::ChunkEntry;
pub use static_enums::*;

pub(crate) const MAGIC_NUMBER: u64 = 0x6574616c73706400; // \0dpslate
pub(crate) const CURRENT_VERSION: u16 = 0x5;
pub(crate) const CHUNK_COMPRESSION_THRESHOLD: usize = 0;
#[derive(Clone, Debug, Encode, Decode)]
pub struct DeepslateRegion {
    pub min_section: i8,
    pub max_section: i8,
    pub chunks: [[Option<ChunkEntry>; 12]; 12],
}