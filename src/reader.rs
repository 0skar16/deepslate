use crate::{chunk::{Chunk, ChunkCompression}, Region, CURRENT_VERSION, MAGIC_NUMBER};
use anyhow::{anyhow, bail, Result};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{self, Cursor, Read, Seek};

pub struct DeepslateReader<R> {
    world: Region,
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
    pub fn chunk_by_pos(&mut self, pos: (u32, u32)) -> Result<Chunk> {
        let mut data_start = self.data_start;
        for z in 0..pos.1 {
            for x in 0..pos.0 {
                data_start += self.world.chunks[z as usize][x as usize].map(|c| c.len).unwrap_or(0) as u64;
            }
        }
        let entry = self
            .world
            .chunks[pos.1 as usize][pos.0 as usize]
            .as_ref()
            .ok_or_else(|| anyhow!("Couldn't get entry!"))?;
        
        let mut buf = vec![0u8; entry.len as usize];
        self.reader
            .seek(io::SeekFrom::Start(data_start))?;
        self.reader.read_exact(&mut buf)?;

        let buf = match entry.compression {
            ChunkCompression::None => buf,
            ChunkCompression::LZ4 => {
                let mut uncompressed_buf = vec![0u8; entry.original_len as usize];
                let mut dec = lz4::Decoder::new(Cursor::new(buf))?;
                dec.read_exact(&mut uncompressed_buf)?;
                uncompressed_buf
            }
            ChunkCompression::Zstd => zstd::decode_all(Cursor::new(buf))?,
        };

        Ok(bitcode::decode(&buf)?)
    }
    pub fn world(&self) -> Region {
        self.world.clone()
    }
}