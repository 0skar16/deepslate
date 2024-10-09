use std::{collections::HashMap, io::{Cursor, Seek, Write}};

use rand::RngCore;

use crate::{chunk::{BlockEntity, BlockState, Chunk, Section, SectionBlockStates}, reader::DeepslateReader, Biome};

const BLOCKS: [&str; 16] = [
    "cobblestone",
    "stone",
    "dirt",
    "grass_block",
    "deepslate",
    "oak_log",
    "birch_log",
    "iron_ore",
    "diamond_ore",
    "glass",
    "granite",
    "diorite",
    "andesite",
    "air",
    "air",
    "air",
];

#[test]
fn region_encode_decode() {
    let mut buf = Cursor::new(vec![]);
    let mut writer = crate::DeepslateWriter::new(&mut buf, -4, 19).expect("Couldn't initialise writer");
    
    let mut chunks: Vec<Chunk> = vec![];
    for i in 0..crate::REGION_CHUNK_COUNT as usize {
        let mut sections = vec![];
        for y in -4..=19 {
            let mut palette = vec![];
            for _ in 0..=i % BLOCKS.len() {
                palette.push(BlockState {
                    block: BLOCKS[rand::random::<usize>() % BLOCKS.len()].to_string(),
                    properties: HashMap::new(),
                });
            }
            let mut block_data = vec![0; 16*16*16];
            for j in 0..16*16*16 {
                block_data[j] = rand::random::<u64>() % palette.len() as u64;
            }
            sections.push(Some(Section {
                y,
                block_states: SectionBlockStates {
                    palette,
                    block_data,
                },
                biome_palette: vec![Biome::Plains],
                block_light: None,
                sky_light: None,
            }));
        }
        chunks.push(Chunk {
            sections,
            block_entities: {
                let mut block_entities = HashMap::new();
                for j in 0..(rand::random::<u8>()%16) as i32 {
                    let mut nbt_data = vec![0; 2048];
                    rand::thread_rng().fill_bytes(&mut nbt_data);
                    block_entities.insert(j, BlockEntity { block_entity_id: None, nbt_data });
                }
                block_entities
            },
            heightmap_mask: 0x0,
            heightmaps: vec![],
        });

        let x = i.rem_euclid(crate::REGION_EDGE_LENGTH);
        let z = i.div_euclid(crate::REGION_EDGE_LENGTH);
        writer.insert_chunk((x as u32, z as u32), chunks[i].clone()).expect("Couldn't insert chunk");
    }
    writer.finalise().expect("Couldn't finalise");

    buf.flush().expect("Couldn't flush buf");
    buf.seek(std::io::SeekFrom::Start(0)).expect("Couldn't seek in buf");

    let mut reader = DeepslateReader::new(&mut buf).expect("Couldn't initialise reader");
    for i in 0..crate::REGION_CHUNK_COUNT { 
        let x = i.rem_euclid(crate::REGION_EDGE_LENGTH);
        let z = i.div_euclid(crate::REGION_EDGE_LENGTH);
        dbg!(x, z);
        let chunk = reader.chunk_by_pos((x as u32, z as u32)).expect("Couldn't read chunk");
        assert_eq!(chunk, chunks[i], "read chunk equal to chunk we wrote");
    }
}