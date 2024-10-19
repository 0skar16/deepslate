pub mod chunk;

use std::{io::Cursor, path::PathBuf, sync::Arc};

use chunk::parse_chunk;
use clap::Parser;
use deepslate::DeepslateWorld;
use fastanvil::{RegionFileLoader, RegionLoader};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Parser, Debug)]
#[command(author = None, version, about = None, long_about = None)]
pub struct Cli {
    anvil_world: PathBuf,
    deepslate_world: PathBuf,
    #[arg(required = false, long)]
    min_x: Option<i32>,
    #[arg(required = false, long)]
    min_z: Option<i32>,
    #[arg(required = false, long)]
    max_x: Option<i32>,
    #[arg(required = false, long)]
    max_z: Option<i32>,
}

fn main() -> anyhow::Result<()> {
    let Cli {
        anvil_world,
        deepslate_world,
        min_x,
        min_z,
        max_x,
        max_z,
    } = Cli::parse();

    let region_dir = anvil_world.join("region");

    //let writer = DeepslateWriter::new(File::create(&deepslate_world)?, -4, 19)?;
    //let writer_rw = Arc::new(RwLock::new(writer));

    let regions = Arc::new(DeepslateWorld::new(&deepslate_world)?);

    let region_file_loader = Arc::new(RegionFileLoader::new(region_dir.clone()));
    let region_list = region_file_loader.list()?;

    let _region_file_loader = region_file_loader.clone();

    //let region_writer_rw = writer_rw.clone();
    region_list
        .par_iter()
        .for_each(move |(region_x, region_z)| {
            let mut region = region_file_loader
                .region(*region_x, *region_z)
                .expect("Error loading region")
                .expect("Region not found");
            let mut writer = regions
                .write_region(region_x.0 as i32, region_z.0 as i32, -4, 19)
                .expect("Couldn't write to region");
            for chunk_data in region.iter() {
                if let Ok(chunk_data) = chunk_data {
                    let chunk_abs_x = region_x.0 as i32 * 32 + (chunk_data.x as i32);
                    let chunk_abs_z = region_z.0 as i32 * 32 + (chunk_data.z as i32);
                    if let Some(min_x) = min_x {
                        if chunk_abs_x < min_x {
                            continue;
                        }
                    }
                    if let Some(max_x) = max_x {
                        if chunk_abs_x > max_x {
                            continue;
                        }
                    }
                    if let Some(min_z) = min_z {
                        if chunk_abs_z < min_z {
                            continue;
                        }
                    }
                    if let Some(max_z) = max_z {
                        if chunk_abs_z > max_z {
                            continue;
                        }
                    }

                    let nbt = simdnbt::borrow::read(&mut Cursor::new(&chunk_data.data))
                        .unwrap()
                        .unwrap();
                    let (_, chunk) = parse_chunk(nbt.as_compound()).unwrap();

                    writer.insert_chunk((chunk_data.x as u32, chunk_data.z as u32), chunk).unwrap();

                    /*match JavaChunk::from_bytes(&chunk_data.data).expect("Couldn't decode chunk") {
                        JavaChunk::Post18(chunk) => {
                            if let Some(sections) = chunk.sections {
                                let sections: Vec<Option<Section>> = sections
                                    .sections()
                                    .par_iter()
                                    .map(|section| {
                                        if let Some(section) =
                                            anvil_section_to_deepslate_section(section)
                                        {
                                            Some(section)
                                        } else {
                                            None
                                        }
                                    })
                                    .collect();
                                writer
                                    .insert_chunk(
                                        (chunk_data.x as u32, chunk_data.z as u32),
                                        Chunk {
                                            sections,
                                            block_entities: HashMap::new(),
                                            heightmap_mask: 0x0,
                                            heightmaps: vec![],
                                        },
                                    )
                                    .expect("Couldn't insert chunk");
                            }

                        }
                        _ => continue,
                    };*/
                }
            }
            writer.finalise().expect("Couldn't finalise a region");
        });
    /*writer_rw
    .write()
    .expect("Couldn't lock writer")
    .finalise()?;*/
    Ok(())
}
/*pub fn anvil_section_to_deepslate_section(section: &fastanvil::Section) -> Option<Section> {
    let block_palette: Vec<BlockState> = section
        .block_states
        .palette()
        .into_iter()
        .map(|block| {
            block
                .encoded_description()
                .to_string()
                .replace("minecraft:", "")
        })
        .map(|block| {
            BlockState::from(block)
        })
        .collect();
    let Some(block_data_iter) = section.block_states.try_iter_indices() else {
        return None;
    };
    let _block_data: Vec<u16> = block_data_iter.map(|b| b as u16).collect();
    let mut block_data = [0u16; 4096];
    block_data.copy_from_slice(&_block_data);

    let block_states = SectionBlockStates {
        block_data,
        palette: block_palette,
    };
    let biome_palette = section
        .biomes
        .palette()
        .into_iter()
        .map(|biome| deepslate::Biome::from(*biome))
        .collect();
    Some(deepslate::chunk::Section {
        block_states,
        biome_palette,
        block_light: None,
        sky_light: None,
        y: section.y,
    })
}*/
