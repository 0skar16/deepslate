use std::{io::{self, Cursor, Read, Seek}, ops::Range};
use byteorder::{LittleEndian, ReadBytesExt};
use anyhow::{anyhow, bail, Result};
use crate::{Chunk, ChunkEntry, DeepslateWorld, CURRENT_VERSION, MAGIC_NUMBER};

pub struct DeepslateReader<R> {
    world: DeepslateWorld,
    data_start: u64,
    reader: R,
}
impl<'a, R: Read + Seek> DeepslateReader<R> {
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
    pub fn chunk(&'a mut self, chunk_id: usize) -> Result<ChunkReader<'a, R>> {
        let mut c = self.data_start;
        for chunk in &(&self.world.chunks)[0..chunk_id] {
            c+=chunk.len as u64;
        }
        let chunk = self.world.chunks.get(chunk_id).ok_or_else(|| anyhow!("Couldn't get entry!"))?;
        Ok(ChunkReader::new(&mut self.reader, c, *chunk))
    }
    pub fn chunk_by_pos(&'a mut self, pos: (isize, isize)) -> Result<ChunkReader<'a, R>> {
        let mut id = 0;
        for chunk in self.world.chunks.clone() {
            if chunk.pos == pos {
                return self.chunk(id);
            }
            id += 1;
        }
        bail!("Couldn't get entry!");
    }
    pub fn decoded_chunk_by_pos(&mut self, pos: (isize, isize)) -> Result<Chunk> {
        let chunk_reader = self.chunk_by_pos(pos)?;
        chunk_reader.decode()
    }
    pub fn world(&self) -> DeepslateWorld {
        self.world.clone()
    }
}

pub struct ChunkReader<'a, R> {
    reader: &'a mut R,
    data_pos_start: u64,
    entry: ChunkEntry,
}
impl<'a, R: Read + Seek> ChunkReader<'a, R> {
    fn new(reader: &'a mut R, start: u64, entry: ChunkEntry) -> Self {
        Self { reader, data_pos_start: start, entry }
    }
    pub fn read(self, range: Option<Range<u64>>) -> Result<Vec<u8>> {
        let range = range.clone().unwrap_or(0..self.entry.len as u64);
        let rl = range.end-range.start;
        let mut buf = vec![0u8; rl as usize];
        self.reader.seek(io::SeekFrom::Start(self.data_pos_start + range.start))?;
        self.reader.read_exact(&mut buf)?;
        match self.entry.compression {
            crate::ChunkCompression::None => Ok(buf),
            crate::ChunkCompression::LZ4 => {
                let mut uncompressed_buf = vec![0u8; self.entry.original_len];
                let mut dec = lz4::Decoder::new(Cursor::new(buf))?;
                dec.read_exact(&mut uncompressed_buf)?;
                Ok(uncompressed_buf)
            },
        }
    }
    pub fn decode(self) -> Result<Chunk> {
        let buf = self.read(None)?;
        let chunk = bitcode::decode(&buf)?;
        Ok(chunk)
    }
}