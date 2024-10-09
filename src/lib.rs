#![feature(iter_array_chunks)]
use std::{fs::{File, OpenOptions}, num::NonZeroUsize, path::PathBuf};

use anyhow::Result;
use bitcode::{Decode, Encode};

pub mod reader;
pub mod writer;
pub mod chunk;

#[cfg(test)]
mod tests;

mod static_enums;
use chunk::{Chunk, ChunkEntry};
use lru::LruCache;
use reader::DeepslateReader;
pub use static_enums::*;
use writer::DeepslateWriter;

pub(crate) const MAGIC_NUMBER: u64 = 0x6574616c73706400; // \0dpslate
pub(crate) const CURRENT_VERSION: u16 = 0x9;
pub(crate) const CHUNK_COMPRESSION_THRESHOLD: usize = 0;
pub const REGION_EDGE_LENGTH: usize = 32;
pub const REGION_EDGE_LENGTH_I32: i32 = REGION_EDGE_LENGTH as i32;
pub const REGION_CHUNK_COUNT: usize = REGION_EDGE_LENGTH*REGION_EDGE_LENGTH;
pub const LRU_CACHE_SIZE: NonZeroUsize = match NonZeroUsize::new(256) {
    Some(n) => n,
    None => unreachable!(),
};

#[derive(Clone, Debug, Encode, Decode)]
pub struct Region {
    pub min_section: i8,
    pub max_section: i8,
    pub chunks: [Option<ChunkEntry>; REGION_CHUNK_COUNT],
}

pub struct CachedRegion {
    file: File,
    data_start: u64,
    region: Region,
}

impl CachedRegion {
    pub fn new(file: File) -> Result<Self> {
        let (file, region, data_start) = DeepslateReader::new(file)?.deconstruct();
        Ok(Self {
            file,
            region,
            data_start,
        })
    }
    pub fn chunk(&mut self, x: u32, z: u32) -> Result<Chunk> {
        DeepslateReader::reconstruct(&mut self.file, self.region.clone(), self.data_start).chunk_by_pos((x, z))
    }
}

pub enum CacheEntry {
    Occupied(CachedRegion),
    Vacant,
}

pub struct DeepslateWorld {
    dir: PathBuf,
    cache: LruCache<(i32, i32), CacheEntry>,
}

impl DeepslateWorld {
    pub fn new(dir: impl Into<PathBuf>) -> Result<Self> {
        let dir: PathBuf = dir.into();
        std::fs::create_dir_all(dir.join("reg"))?;
        Ok(Self { dir, cache: LruCache::new(LRU_CACHE_SIZE) })
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

    pub fn region(&mut self, x: i32, z: i32) -> Result<Option<&mut CachedRegion>> {
        if self.cache.get(&(x, z)).is_some() {
            match self.cache.get_mut(&(x, z)) {
                None => unreachable!(),
                Some(CacheEntry::Occupied(r)) => return Ok(Some(r)),
                Some(CacheEntry::Vacant) => return Ok(None),
            }
        } else {
            let path = self.dir.join("reg").join(format!("{x}_{z}.dpslate"));

            if path.exists() {
                let f = OpenOptions::new().read(true).write(true).open(path)?;
                self.cache.put((x, z), CacheEntry::Occupied(CachedRegion::new(f)?));
            } else {
                self.cache.put((x, z), CacheEntry::Vacant);
            }
            match self.cache.get_mut(&(x, z)) {
                None => unreachable!(),
                Some(CacheEntry::Occupied(r)) => return Ok(Some(r)),
                Some(CacheEntry::Vacant) => return Ok(None),
            }
        }
    }

    pub fn chunk(&mut self, x: i32, z: i32) -> Result<Option<Chunk>> {
        let reg_x = x.div_euclid(REGION_EDGE_LENGTH_I32);
        let reg_z = z.div_euclid(REGION_EDGE_LENGTH_I32);

        let Some(reg) = self.region(reg_x, reg_z)? else { return Ok(None) };

        let chk_x = x.rem_euclid(REGION_EDGE_LENGTH_I32) as u32;
        let chk_z = z.rem_euclid(REGION_EDGE_LENGTH_I32) as u32;

        reg.chunk(chk_x, chk_z).map(|c| Some(c))
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

