#![feature(iter_array_chunks)]
use std::{
    collections::HashMap,
    fs::File,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use clap::Parser;
use deepslate::{writer::DeepslateWriter, Chunk, Section, SectionBlockStates};
use fastanvil::{JavaChunk, RegionFileLoader, RegionLoader};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Parser, Debug)]
#[command(author = None, version, about = None, long_about = None)]
pub struct Cli {
    anvil_world: PathBuf,
    deepslate_world: PathBuf,
    #[arg(required = false, long)]
    min_x: Option<isize>,
    #[arg(required = false, long)]
    min_z: Option<isize>,
    #[arg(required = false, long)]
    max_x: Option<isize>,
    #[arg(required = false, long)]
    max_z: Option<isize>,
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

    let writer = DeepslateWriter::new(File::create(&deepslate_world)?, -4, 19)?;
    let writer_rw = Arc::new(RwLock::new(writer));

    let region_file_loader = Arc::new(RegionFileLoader::new(region_dir.clone()));
    let region_list = region_file_loader.list()?;

    let _region_file_loader = region_file_loader.clone();

    let region_writer_rw = writer_rw.clone();
    region_list
        .par_iter()
        .for_each(move |(region_x, region_z)| {
            let mut region = region_file_loader
                .region(*region_x, *region_z)
                .expect("Error loading region")
                .expect("Region not found");
            for chunk_data in region.iter() {
                if let Ok(chunk_data) = chunk_data {
                    let chunk_abs_x = region_x.0 * 32 + (chunk_data.x as isize);
                    let chunk_abs_z = region_z.0 * 32 + (chunk_data.z as isize);
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

                    match JavaChunk::from_bytes(&chunk_data.data).expect("Couldn't decode chunk") {
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
                                if let Ok(mut writer) = region_writer_rw.write() {
                                    writer
                                        .insert_chunk(
                                            (chunk_abs_x, chunk_abs_z),
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
                        }
                        _ => continue,
                    };
                }
            }
        });
    writer_rw
        .write()
        .expect("Couldn't lock writer")
        .finalise()?;
    Ok(())
}
pub fn anvil_section_to_deepslate_section(section: &fastanvil::Section) -> Option<Section> {
    let block_palette: Vec<String> = section
        .block_states
        .palette()
        .into_iter()
        .map(|block| {
            block
                .encoded_description()
                .to_string()
                .replace("minecraft:", "")
        })
        .collect();
    let Some(block_data_iter) = section.block_states.try_iter_indices() else {
        return None;
    };
    let block_data: Vec<u64> = block_data_iter.map(|b| b as u64).collect();
    let block_data = SectionBlockStates {
        block_data,
        pallette: block_palette,
    };
    let biome_pallete = section
        .biomes
        .palette()
        .into_iter()
        .map(|biome| deepslate::Biome::from(*biome))
        .collect();
    Some(deepslate::Section {
        block_states: block_data,
        biome_pallete,
        block_light: None,
        sky_light: None,
        y: section.y,
    })
}
