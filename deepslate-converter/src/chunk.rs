use std::collections::HashMap;

use deepslate::{
    chunk::{BlockState, Section, SectionBlockStates},
    Biome, PropName, PropValue,
};
use simdnbt::borrow::NbtCompound;

pub fn parse_chunk(nbt: NbtCompound) -> Option<((i32, i32), deepslate::chunk::Chunk)> {
    let _data_version = nbt.int("DataVersion")?;

    let x = nbt.int("xPos")?;
    let z = nbt.int("zPos")?;
    let _min_section = nbt.int("yPos")?;

    let _status = nbt.string("Status")?;

    let sections_list = nbt.list("sections")?;

    if sections_list.id() == 0 {
        assert!(sections_list.empty());
    }

    let sections = if !sections_list.empty() {
        nbt
            .list("sections")?
            .compounds()?
            .into_iter()
            .map(parse_section)
            .collect()
    } else {
        vec![None; 4+19+1]
    };

    

    Some((
        (x, z),
        deepslate::chunk::Chunk {
            sections,
            block_entities: HashMap::new(),
            heightmap_mask: 0x0,
            heightmaps: vec![],
        },
    ))
}

pub fn parse_section(nbt: NbtCompound) -> Option<deepslate::chunk::Section> {
    let y = nbt.byte("Y")?;

    let block_states = parse_block_states(nbt.compound("block_states")?)?;
    Some(Section {
        y,
        block_states,
        biome_palette: vec![Biome::Plains],
        block_light: /*nbt.byte_array("BlockLight").map(parse_light)*/ None,
        sky_light: /*nbt.byte_array("SkyLight").map(parse_light)*/ None,
    })
}
fn parse_light(byte_array: &[u8]) -> Vec<u8> {
    byte_array
        .iter()
        .map(|b| [*b & 0x0f, (*b >> 4) & 0x0f])
        .flatten()
        .collect()
}

fn parse_block_states(nbt: NbtCompound) -> Option<deepslate::chunk::SectionBlockStates> {
    let palette = nbt
        .list("palette")?
        .compounds()?
        .into_iter()
        .map(parse_block)
        .collect::<Option<Vec<BlockState>>>()?;

    assert!(palette.len() > 0);

    let indicies = nbt.long_array("data");

    if indicies.is_none() && palette.len() == 1 {
        Some(SectionBlockStates {
            palette,
            block_data: [0; 4096],
        })
    } else {
        let block_data = parse_packed_indicies(indicies?, palette.len())?;
        Some(SectionBlockStates {
            palette,
            block_data,
        })
    }
}

pub fn parse_block(nbt: NbtCompound) -> Option<BlockState> {
    let block = nbt.string("Name")?;
    let properties = nbt
        .compound("Properties").and_then(|props| {
            props.iter()
            .map(|(name, value)| Some((name.to_str(), value.string()?.to_str())))
            .map(|prop| {
                if let Some((name, value)) = prop {
                    Some((PropName::from_str(&name), PropValue::from_str(&value)))
                } else {
                    None
                }
            })
            .collect::<Option<HashMap<PropName, PropValue>>>()
        }).unwrap_or_default();
    Some(BlockState {
        block: block.to_string(),
        properties,
    })
}

fn parse_packed_indicies<const INIDICIES: usize>(data: Vec<i64>, palette_len: usize) -> Option<[u16; INIDICIES]> {
    debug_assert!(palette_len > 1);

    let bits_per_idx = bit_width(palette_len - 1).max(4);
    let idxs_per_long = 64 / bits_per_idx;
    let long_count = INIDICIES.div_ceil(idxs_per_long);
    let mask = 2_u64.pow(bits_per_idx as u32) - 1;

    if long_count != data.len() {
        return None;
    };

    let mut i: u32 = 0;

    let mut indicies = [0; INIDICIES];

    for long in data {
        let u64 = long as u64;

        for j in 0..idxs_per_long {
            if i >= INIDICIES as u32 {
                break;
            }

            let idx = (u64 >> (bits_per_idx * j)) & mask;

            indicies[i as usize] = idx as u16;

            i += 1;
        }
    }

    Some(indicies)
}
const fn bit_width(n: usize) -> usize {
    (usize::BITS - n.leading_zeros()) as usize
}