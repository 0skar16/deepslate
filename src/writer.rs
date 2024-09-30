use std::{
    collections::BTreeSet,
    io::{Seek, Write},
};

use crate::{Chunk, ChunkCompression, ChunkEntry, DeepslateWorld, CHUNK_COMPRESSION_THRESHOLD};
use anyhow::{bail, Result};
use byteorder::{LittleEndian, WriteBytesExt};

pub struct DeepslateWriter<W> {
    writer: W,
    written_chunks: BTreeSet<(i32, i32)>,
    chunks: Vec<ChunkEntry>,
    min_section: i8,
    max_section: i8,
    chunks_len: u32,
}

impl<W: Seek + Write> DeepslateWriter<W> {
    pub fn new(mut writer: W, min_section: i8, max_section: i8) -> Result<Self> {
        writer.write_u64::<LittleEndian>(crate::MAGIC_NUMBER)?;
        writer.write_u16::<LittleEndian>(crate::CURRENT_VERSION)?;
        writer.write_u64::<LittleEndian>(0)?;
        writer.write_u32::<LittleEndian>(0)?;
        Ok(Self {
            writer,
            min_section,
            max_section,
            chunks_len: 0,
            written_chunks: BTreeSet::new(),
            chunks: vec![],
        })
    }

    pub fn insert_chunk(&mut self, pos: (i32, i32), chunk: Chunk) -> Result<()> {
        if self.written_chunks.contains(&pos) {
            bail!("Chunk {:?} has already been written", pos);
        }

        let chunk_buf = bitcode::encode(&chunk);
        let len = chunk_buf.len();
        self.written_chunks.insert(pos);
        let chunk_buf = if chunk_buf.len() > CHUNK_COMPRESSION_THRESHOLD {
            let mut enc = zstd::Encoder::new(vec![], 3)?;
            enc.write_all(&chunk_buf)?;
            enc.finish()?
        } else {
            chunk_buf
        };
        self.chunks.push(ChunkEntry {
            pos,
            len: chunk_buf.len() as u32,
            original_len: len as u32,
            compression: if len > CHUNK_COMPRESSION_THRESHOLD {
                ChunkCompression::Zstd
            } else {
                ChunkCompression::None
            },
        });

        self.chunks_len += chunk_buf.len() as u32;
        self.writer.write_all(&chunk_buf)?;

        Ok(())
    }

    pub fn finalise(&mut self) -> Result<()> {
        let world_buf = bitcode::encode(&DeepslateWorld {
            min_section: self.min_section,
            max_section: self.max_section,
            chunks: self.chunks.clone(),
        });

        self.writer.write_all(&world_buf)?;
        self.writer.seek(std::io::SeekFrom::Start(8 + 2))?;
        self.writer
            .write_u64::<LittleEndian>(self.chunks_len as u64)?;
        self.writer
            .write_u32::<LittleEndian>(world_buf.len() as u32)?;
        Ok(())
    }
    pub fn inner(&mut self) -> &mut W {
        &mut self.writer
    }
}
