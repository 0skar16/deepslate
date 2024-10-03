use std::{
    collections::BTreeSet, io::{Seek, Write}
};

use crate::{chunk::{Chunk, ChunkCompression}, ChunkEntry, Region, CHUNK_COMPRESSION_THRESHOLD, REGION_CHUNK_COUNT, REGION_EDGE_LENGTH};
use anyhow::{bail, Result};
use byteorder::{LittleEndian, WriteBytesExt};

pub struct DeepslateWriter<W> {
    writer: W,
    written_chunks: BTreeSet<(u32, u32)>,
    chunks: [Option<ChunkEntry>; REGION_CHUNK_COUNT],
    min_section: i8,
    max_section: i8,
    chunks_len: u64,
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
            chunks: [None; REGION_CHUNK_COUNT],
        })
    }

    pub fn insert_chunk(&mut self, pos: (u32, u32), chunk: Chunk) -> Result<()> {
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
        self.chunks[pos.1 as usize * REGION_EDGE_LENGTH + pos.0 as usize].replace(ChunkEntry {
            len: chunk_buf.len() as u32,
            original_len: len as u32,
            compression: if len > CHUNK_COMPRESSION_THRESHOLD {
                ChunkCompression::Zstd
            } else {
                ChunkCompression::None
            },
            data_start: self.chunks_len,
        });

        self.chunks_len += chunk_buf.len() as u64;
        self.writer.write_all(&chunk_buf)?;

        Ok(())
    }

    pub fn finalise(&mut self) -> Result<()> {
        let region_buf = bitcode::encode(&Region {
            min_section: self.min_section,
            max_section: self.max_section,
            chunks: self.chunks.clone(),
        });

        self.writer.write_all(&region_buf)?;
        self.writer.seek(std::io::SeekFrom::Start(8 + 2))?;
        self.writer
            .write_u64::<LittleEndian>(self.chunks_len as u64)?;
        self.writer
            .write_u32::<LittleEndian>(region_buf.len() as u32)?;
        Ok(())
    }
    pub fn inner(&mut self) -> &mut W {
        &mut self.writer
    }
}