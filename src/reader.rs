use crate::{Chunk, DeepslateWorld, CURRENT_VERSION, MAGIC_NUMBER};
use anyhow::{anyhow, bail, Result};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{self, Cursor, Read, Seek};

pub struct DeepslateReader<R> {
    world: DeepslateWorld,
    data_start: u64,
    reader: R,
}
impl<R: Read + Seek> DeepslateReader<R> {
    pub fn new(mut reader: R) -> Result<Self> {
        if reader.read_u64::<LittleEndian>()? != MAGIC_NUMBER {
            bail!("Doesn't start with magic number");
        }
        if reader.read_u16::<LittleEndian>()? != CURRENT_VERSION {
            bail!("Wrong version!");
        }
        let _chunks_len = reader.read_u64::<LittleEndian>()? as usize;
        let world_len = reader.read_u32::<LittleEndian>()? as usize;
        reader.seek(io::SeekFrom::End(-(world_len as i64)))?;
        let mut world_buf = vec![0u8; world_len];
        reader.read_exact(&mut world_buf)?;

        let world = bitcode::decode(&world_buf)?;

        let data_start: u64 = 8 + 2 + 8 + 4;
        reader.seek(io::SeekFrom::Start(data_start))?;
        Ok(Self {
            world,
            reader,
            data_start,
        })
    }
    pub fn chunk(&mut self, chunk_id: u32) -> Result<Chunk> {
        let mut data_start = self.data_start;
        for chunk in &(&self.world.chunks)[0..chunk_id as usize] {
            data_start += chunk.len as u64;
        }
        let entry = self
            .world
            .chunks
            .get(chunk_id as usize)
            .ok_or_else(|| anyhow!("Couldn't get entry!"))?;
        
        let mut buf = vec![0u8; entry.len as usize];
        self.reader
            .seek(io::SeekFrom::Start(data_start))?;
        self.reader.read_exact(&mut buf)?;

        let buf = match entry.compression {
            crate::ChunkCompression::None => buf,
            crate::ChunkCompression::LZ4 => {
                let mut uncompressed_buf = vec![0u8; entry.original_len as usize];
                let mut dec = lz4::Decoder::new(Cursor::new(buf))?;
                dec.read_exact(&mut uncompressed_buf)?;
                uncompressed_buf
            }
            crate::ChunkCompression::Zstd => zstd::decode_all(Cursor::new(buf))?,
        };

        Ok(bitcode::decode(&buf)?)
    }
    pub fn chunk_by_pos(&mut self, pos: (i32, i32)) -> Result<Chunk> {
        let mut id = 0;
        for chunk in &self.world.chunks {
            if chunk.pos == pos {
                return self.chunk(id);
            }
            id += 1;
        }
        bail!("Couldn't get entry!");
    }
    pub fn world(&self) -> DeepslateWorld {
        self.world.clone()
    }
}