#![feature(iter_array_chunks)]
use std::{fs::File, path::PathBuf};

use anyhow::Result;
use bitcode::{Decode, Encode};

pub mod reader;
pub mod writer;
pub mod chunk;
mod static_enums;
use chunk::ChunkEntry;
use reader::DeepslateReader;
pub use static_enums::*;
use writer::DeepslateWriter;

pub(crate) const MAGIC_NUMBER: u64 = 0x6574616c73706400; // \0dpslate
pub(crate) const CURRENT_VERSION: u16 = 0x5;
pub(crate) const CHUNK_COMPRESSION_THRESHOLD: usize = 0;
pub const REGION_EDGE_LENGTH: usize = 32;
pub const REGION_EDGE_LENGTH_I32: i32 = REGION_EDGE_LENGTH as i32;
pub const REGION_CHUNK_COUNT: usize = REGION_EDGE_LENGTH*REGION_EDGE_LENGTH;

#[derive(Clone, Debug, Encode, Decode)]
pub struct Region {
    pub min_section: i8,
    pub max_section: i8,
    pub chunks: [Option<ChunkEntry>; REGION_CHUNK_COUNT],
}

pub struct DeepslateWorld {
    dir: PathBuf,
}

impl DeepslateWorld {
    pub fn new(dir: impl Into<PathBuf>) -> Result<Self> {
        let dir: PathBuf = dir.into();
        std::fs::create_dir_all(dir.join("reg"))?;
        Ok(Self { dir })
    }

    pub fn list(&self) -> Result<Vec<(i32, i32)>> {
        let paths = std::fs::read_dir(&self.dir.join("reg"))?;

        let regions = paths.into_iter()
            .filter_map(|path| path.ok())
            .map(|path| path.path())
            .filter(|path| path.is_file())
            .filter(|path| {
                let ext = path.extension();
                ext.is_some() && ext.unwrap() == "dpslate"
            })
            .filter(|path| std::fs::metadata(path).unwrap().len() > 0)
            .filter_map(|p| coords_from_region(p))
            .collect();

        Ok(regions)
    }

    pub fn read_region(&self, x: i32, z: i32) -> Result<Option<DeepslateReader<File>>> {
        let path = self.dir.join("reg").join(format!("{x}_{z}.dpslate"));

        if path.exists() {
            let r = DeepslateReader::new(std::fs::File::open(path)?)?;
            Ok(Some(r))
        } else {
            Ok(None)
        }
    }
    
    pub fn write_region(&self, x: i32, z: i32, min_section: i8, max_section: i8) -> Result<DeepslateWriter<File>> {
        let path = self.dir.join("reg").join(format!("{x}_{z}.dpslate"));
        Ok(DeepslateWriter::new(std::fs::OpenOptions::new().create(true).write(true).append(false).truncate(true).open(path)?, min_section, max_section)?)
    }
}

fn coords_from_region(region: PathBuf) -> Option<(i32, i32)> {
    let filename = region.file_name()?.to_str()?;
    let mut parts = filename.split('_');
    let x = parts.next()?.parse::<i32>().ok()?;
    let z = parts.next()?.parse::<i32>().ok()?;
    Some((x, z))
}

