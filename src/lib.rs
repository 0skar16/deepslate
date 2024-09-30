#![feature(iter_array_chunks)]
use std::collections::HashMap;

use bitcode::{Decode, Encode};

pub mod reader;
pub mod writer;

pub(crate) const MAGIC_NUMBER: u64 = 0x6574616c73706400; // \0dpslate
pub(crate) const CURRENT_VERSION: u16 = 0x5;
pub(crate) const CHUNK_COMPRESSION_THRESHOLD: usize = 0;
#[derive(Clone, Debug, Encode, Decode)]
pub struct DeepslateWorld {
    pub min_section: i8,
    pub max_section: i8,
    pub chunks: Vec<ChunkEntry>,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode, Default, Copy)]
pub struct ChunkEntry {
    pub pos: (i32, i32),
    pub len: u32,
    pub original_len: u32,
    pub compression: ChunkCompression,
}

#[derive(Clone, PartialEq, Debug, Encode, Decode, Default, Copy)]
pub enum ChunkCompression {
    #[default]
    None,
    LZ4,
    Zstd,
}

#[derive(Debug, Encode, Decode)]
pub struct Chunk {
    pub sections: Vec<Option<Section>>,
    pub block_entities: HashMap<i32, BlockEntity>,
    pub heightmap_mask: u32,
    pub heightmaps: Vec<Vec<u8>>,
}

#[derive(Debug, Encode, Decode)]
pub struct Section {
    pub y: i8,
    pub block_states: SectionBlockStates,
    pub biome_palette: Vec<Biome>,
    pub block_light: Option<Vec<u8>>,
    pub sky_light: Option<Vec<u8>>,
}
#[derive(Debug)]
pub struct SectionBlockStates {
    pub palette: Vec<String>,
    pub block_data: Vec<u64>,
}

#[derive(Default)]
pub struct SectionBlockStatesEncoder {
    var1: bitcode::__private::VariantEncoder<2usize>,
    var2: bitcode::__private::VariantEncoder<4usize>,
    palette: <Vec<String> as bitcode::__private::Encode>::Encoder,
    unanymous_palette: <String as bitcode::__private::Encode>::Encoder,
    data: <Vec<u8> as bitcode::__private::Encode>::Encoder,
}

impl bitcode::Encode for SectionBlockStates {
    type Encoder = SectionBlockStatesEncoder;
}

impl bitcode::__private::Encoder<SectionBlockStates> for SectionBlockStatesEncoder {
    fn encode(&mut self, t: &SectionBlockStates) {
        use bitcode::__private::Buffer as _;
        let additional = std::num::NonZeroUsize::MIN;
        if t.palette.len() == 1 {
            self.unanymous_palette.reserve(additional);

            self.var1.encode(&0);
            self.unanymous_palette.encode(&t.palette[0]);
        } else if t.palette.len() == 0 {
            panic!("palette needs to be longer than 0");
        } else {
            self.var1.encode(&1);
            self.var2.reserve(additional);
            let data: Vec<u8> = if t.palette.len() <= 4 {
                self.var2.encode(&0);
                t.block_data
                    .iter()
                    .array_chunks::<4>()
                    .map(|[b1, b2, b3, b4]| (b4 << 6 | b3 << 4 | b2 << 2 | b1) as u8)
                    .collect()
            } else if t.palette.len() <= 16 {
                self.var2.encode(&1);
                t.block_data
                    .iter()
                    .array_chunks::<2>()
                    .map(|[b1, b2]| (b2 << 4 | b1) as u8)
                    .collect()
            } else if t.palette.len() <= u8::MAX as usize + 1 {
                self.var2.encode(&2);
                t.block_data.iter().map(|b| *b as u8).collect()
            } else {
                self.var2.encode(&3);
                t.block_data
                    .iter()
                    .array_chunks::<2>()
                    .map(|[b1, b2]| {
                        let [_, o3, o2, o1] = (((*b1) | (*b2) << 12) as u32).to_be_bytes();
                        [o3, o2, o1]
                    })
                    .flatten()
                    .collect()
            };
            self.data.reserve(additional);
            self.palette.reserve(additional);

            self.data.encode(&data);
            self.palette.encode(&t.palette);
        }
    }
}

impl bitcode::__private::Buffer for SectionBlockStatesEncoder {
    fn collect_into(&mut self, out: &mut Vec<u8>) {
        self.var1.collect_into(out);
        self.var2.collect_into(out);
        self.unanymous_palette.collect_into(out);
        self.data.collect_into(out);
        self.palette.collect_into(out);
    }

    fn reserve(&mut self, additional: std::num::NonZeroUsize) {
        self.var1.reserve(additional);
    }
}

impl<'de> bitcode::__private::Decode<'de> for SectionBlockStates {
    type Decoder = SectionBlockStatesDecoder<'de>;
}

#[derive(Default)]
pub struct SectionBlockStatesDecoder<'de> {
    var1: bitcode::__private::VariantDecoder<'de, 2usize, false>,
    var2: bitcode::__private::VariantDecoder<'de, 4usize, true>,
    unanymous_palette: <String as bitcode::__private::Decode<'de>>::Decoder,
    palette: <Vec<String> as bitcode::__private::Decode<'de>>::Decoder,
    data: <Vec<u8> as bitcode::__private::Decode<'de>>::Decoder,
}

impl<'__de> bitcode::__private::View<'__de> for SectionBlockStatesDecoder<'__de> {
    fn populate(
        &mut self,
        input: &mut &'__de [u8],
        __length: usize,
    ) -> bitcode::__private::Result<()> {
        self.var1.populate(input, __length)?;

        let __length = self.var1.length(0);
        self.unanymous_palette.populate(input, __length)?;

        let __length = self.var1.length(1);
        self.var2.populate(input, __length)?;
        self.data.populate(input, __length)?;
        self.palette.populate(input, __length)?;

        Ok(())
    }
}
impl<'__de> bitcode::__private::Decoder<'__de, SectionBlockStates>
    for SectionBlockStatesDecoder<'__de>
{
    fn decode_in_place(&mut self, out: &mut std::mem::MaybeUninit<SectionBlockStates>) {
        let (block_data, palette) = match self.var1.decode() {
            0u8 => {
                let palette = self.unanymous_palette.decode();
                (vec![0; 16 * 16 * 16], vec![palette])
            }
            1u8 => match self.var2.decode() {
                0u8 => {
                    let quarters_data: Vec<u8> = self.data.decode();
                    let palette = self.palette.decode();
                    (
                        quarters_data
                            .into_iter()
                            .map(|q| {
                                [
                                    q as u64 & 0x3,
                                    (q as u64 >> 2) & 0x3,
                                    (q as u64 >> 4) & 0x3,
                                    (q as u64 >> 6) & 0x3,
                                ]
                            })
                            .flatten()
                            .collect(),
                        palette,
                    )
                }
                1u8 => {
                    let halves_data: Vec<u8> = self.data.decode();

                    let palette = self.palette.decode();
                    (
                        halves_data
                            .into_iter()
                            .map(|h| [h as u64 & 0xf, (h as u64 >> 4) & 0xf])
                            .flatten()
                            .collect(),
                        palette,
                    )
                }
                2u8 => {
                    let data: Vec<u8> = self.data.decode();

                    let palette = self.palette.decode();
                    (data.into_iter().map(|b| b as u64).collect(), palette)
                }
                3u8 => {
                    let twelves_data: Vec<u8> = self.data.decode();

                    let palette = self.palette.decode();
                    (
                        twelves_data
                            .into_iter()
                            .array_chunks::<3>()
                            .map(|[b3, b2, b1]| {
                                let t = u32::from_be_bytes([0, b3, b2, b1]);
                                [t as u64 & 0xfff, (t as u64 >> 12) & 0xfff]
                            })
                            .flatten()
                            .collect(),
                        palette,
                    )
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        out.write(SectionBlockStates {
            palette,
            block_data,
        });
    }
}

#[derive(Debug, Encode, Decode)]
pub struct BlockEntity {
    pub block_entity_id: Option<String>,
    pub nbt_data: Vec<u8>,
}

#[derive(Debug, Encode, Decode)]
#[repr(u8)]
pub enum Biome {
    Ocean,
    Forest,
    River,
    FrozenOcean,
    FrozenRiver,
    Beach,
    DeepOcean,
    StoneShore,
    SnowyBeach,
    WarmOcean,
    LukewarmOcean,
    ColdOcean,
    DeepWarmOcean,
    DeepLukewarmOcean,
    DeepColdOcean,
    DeepFrozenOcean,
    WoodedHills,
    FlowerForest,
    BirchForest,
    BirchForestHills,
    TallBirchForest,
    TallBirchHills,
    DarkForest,
    DarkForestHills,
    Jungle,
    JungleHills,
    ModifiedJungle,
    JungleEdge,
    ModifiedJungleEdge,
    BambooJungle,
    BambooJungleHills,
    Taiga,
    TaigaHills,
    TaigaMountains,
    SnowyTaiga,
    SnowyTaigaHills,
    SnowyTaigaMountains,
    GiantTreeTaiga,
    GiantTreeTaigaHills,
    GiantSpruceTaiga,
    GiantSpruceTaigaHills,
    MushroomFields,
    MushroomFieldShore,
    Swamp,
    SwampHills,
    Savanna,
    SavannaPlateau,
    ShatteredSavanna,
    ShatteredSavannaPlateau,
    Plains,
    SunflowerPlains,
    Desert,
    DesertHills,
    DesertLakes,
    SnowyTundra,
    SnowyMountains,
    IceSpikes,
    Mountains,
    WoodedMountains,
    GravellyMountains,
    ModifiedGravellyMountains,
    MountainEdge,
    Badlands,
    BadlandsPlateau,
    ModifiedBadlandsPlateau,
    WoodedBadlandsPlateau,
    ModifiedWoodedBadlandsPlateau,
    ErodedBadlands,
    Nether,
    TheEnd,
    SmallEndIslands,
    EndMidlands,
    EndHighlands,
    EndBarrens,
    SoulSandValley,
    CrimsonForest,
    WarpedForest,
    TheVoid,
    BasaltDeltas,
    DripstoneCaves,
    FrozenPeaks,
    Grove,
    JaggedPeaks,
    LushCaves,
    Meadow,
    NetherWastes,
    OldGrowthBirchForest,
    OldGrowthPineTaiga,
    OldGrowthSpruceTaiga,
    SnowyPlains,
    SnowySlopes,
    SparseJungle,
    StonyPeaks,
    StonyShore,
    WindsweptForest,
    WindsweptGravellyHills,
    WindsweptHills,
    WindsweptSavanna,
    WoodedBadlands,
    MangroveSwamp,
    DeepDark,
    Custom(String),
    Unknown,
}
#[cfg(feature = "binary")]
impl From<fastanvil::biome::Biome> for Biome {
    fn from(value: fastanvil::biome::Biome) -> Self {
        match value {
            fastanvil::biome::Biome::Ocean => Biome::Ocean,
            fastanvil::biome::Biome::Forest => Biome::Forest,
            fastanvil::biome::Biome::River => Biome::River,
            fastanvil::biome::Biome::FrozenOcean => Biome::FrozenOcean,
            fastanvil::biome::Biome::FrozenRiver => Biome::FrozenRiver,
            fastanvil::biome::Biome::Beach => Biome::Beach,
            fastanvil::biome::Biome::DeepOcean => Biome::DeepOcean,
            fastanvil::biome::Biome::StoneShore => Biome::StoneShore,
            fastanvil::biome::Biome::SnowyBeach => Biome::SnowyBeach,
            fastanvil::biome::Biome::WarmOcean => Biome::WarmOcean,
            fastanvil::biome::Biome::LukewarmOcean => Biome::LukewarmOcean,
            fastanvil::biome::Biome::ColdOcean => Biome::ColdOcean,
            fastanvil::biome::Biome::DeepWarmOcean => Biome::DeepWarmOcean,
            fastanvil::biome::Biome::DeepLukewarmOcean => Biome::DeepLukewarmOcean,
            fastanvil::biome::Biome::DeepColdOcean => Biome::DeepColdOcean,
            fastanvil::biome::Biome::DeepFrozenOcean => Biome::DeepFrozenOcean,
            fastanvil::biome::Biome::WoodedHills => Biome::WoodedHills,
            fastanvil::biome::Biome::FlowerForest => Biome::FlowerForest,
            fastanvil::biome::Biome::BirchForest => Biome::BirchForest,
            fastanvil::biome::Biome::BirchForestHills => Biome::BirchForestHills,
            fastanvil::biome::Biome::TallBirchForest => Biome::TallBirchForest,
            fastanvil::biome::Biome::TallBirchHills => Biome::TallBirchHills,
            fastanvil::biome::Biome::DarkForest => Biome::DarkForest,
            fastanvil::biome::Biome::DarkForestHills => Biome::DarkForestHills,
            fastanvil::biome::Biome::Jungle => Biome::Jungle,
            fastanvil::biome::Biome::JungleHills => Biome::JungleHills,
            fastanvil::biome::Biome::ModifiedJungle => Biome::ModifiedJungle,
            fastanvil::biome::Biome::JungleEdge => Biome::JungleEdge,
            fastanvil::biome::Biome::ModifiedJungleEdge => Biome::ModifiedJungleEdge,
            fastanvil::biome::Biome::BambooJungle => Biome::BambooJungle,
            fastanvil::biome::Biome::BambooJungleHills => Biome::BambooJungleHills,
            fastanvil::biome::Biome::Taiga => Biome::Taiga,
            fastanvil::biome::Biome::TaigaHills => Biome::TaigaHills,
            fastanvil::biome::Biome::TaigaMountains => Biome::TaigaMountains,
            fastanvil::biome::Biome::SnowyTaiga => Biome::SnowyTaiga,
            fastanvil::biome::Biome::SnowyTaigaHills => Biome::SnowyTaigaHills,
            fastanvil::biome::Biome::SnowyTaigaMountains => Biome::SnowyTaigaMountains,
            fastanvil::biome::Biome::GiantTreeTaiga => Biome::GiantTreeTaiga,
            fastanvil::biome::Biome::GiantTreeTaigaHills => Biome::GiantTreeTaigaHills,
            fastanvil::biome::Biome::GiantSpruceTaiga => Biome::GiantSpruceTaiga,
            fastanvil::biome::Biome::GiantSpruceTaigaHills => Biome::GiantSpruceTaigaHills,
            fastanvil::biome::Biome::MushroomFields => Biome::MushroomFields,
            fastanvil::biome::Biome::MushroomFieldShore => Biome::MushroomFieldShore,
            fastanvil::biome::Biome::Swamp => Biome::Swamp,
            fastanvil::biome::Biome::SwampHills => Biome::SwampHills,
            fastanvil::biome::Biome::Savanna => Biome::Savanna,
            fastanvil::biome::Biome::SavannaPlateau => Biome::SavannaPlateau,
            fastanvil::biome::Biome::ShatteredSavanna => Biome::ShatteredSavanna,
            fastanvil::biome::Biome::ShatteredSavannaPlateau => Biome::ShatteredSavannaPlateau,
            fastanvil::biome::Biome::Plains => Biome::Plains,
            fastanvil::biome::Biome::SunflowerPlains => Biome::SunflowerPlains,
            fastanvil::biome::Biome::Desert => Biome::Desert,
            fastanvil::biome::Biome::DesertHills => Biome::DesertHills,
            fastanvil::biome::Biome::DesertLakes => Biome::DesertLakes,
            fastanvil::biome::Biome::SnowyTundra => Biome::SnowyTundra,
            fastanvil::biome::Biome::SnowyMountains => Biome::SnowyMountains,
            fastanvil::biome::Biome::IceSpikes => Biome::IceSpikes,
            fastanvil::biome::Biome::Mountains => Biome::Mountains,
            fastanvil::biome::Biome::WoodedMountains => Biome::WoodedMountains,
            fastanvil::biome::Biome::GravellyMountains => Biome::GravellyMountains,
            fastanvil::biome::Biome::ModifiedGravellyMountains => Biome::ModifiedGravellyMountains,
            fastanvil::biome::Biome::MountainEdge => Biome::MountainEdge,
            fastanvil::biome::Biome::Badlands => Biome::Badlands,
            fastanvil::biome::Biome::BadlandsPlateau => Biome::BadlandsPlateau,
            fastanvil::biome::Biome::ModifiedBadlandsPlateau => Biome::ModifiedBadlandsPlateau,
            fastanvil::biome::Biome::WoodedBadlandsPlateau => Biome::WoodedBadlandsPlateau,
            fastanvil::biome::Biome::ModifiedWoodedBadlandsPlateau => {
                Biome::ModifiedWoodedBadlandsPlateau
            }
            fastanvil::biome::Biome::ErodedBadlands => Biome::ErodedBadlands,
            fastanvil::biome::Biome::Nether => Biome::Nether,
            fastanvil::biome::Biome::TheEnd => Biome::TheEnd,
            fastanvil::biome::Biome::SmallEndIslands => Biome::SmallEndIslands,
            fastanvil::biome::Biome::EndMidlands => Biome::EndMidlands,
            fastanvil::biome::Biome::EndHighlands => Biome::EndHighlands,
            fastanvil::biome::Biome::EndBarrens => Biome::EndBarrens,
            fastanvil::biome::Biome::SoulSandValley => Biome::SoulSandValley,
            fastanvil::biome::Biome::CrimsonForest => Biome::CrimsonForest,
            fastanvil::biome::Biome::WarpedForest => Biome::WarpedForest,
            fastanvil::biome::Biome::TheVoid => Biome::TheVoid,
            fastanvil::biome::Biome::BasaltDeltas => Biome::BasaltDeltas,
            fastanvil::biome::Biome::DripstoneCaves => Biome::DripstoneCaves,
            fastanvil::biome::Biome::FrozenPeaks => Biome::FrozenPeaks,
            fastanvil::biome::Biome::Grove => Biome::Grove,
            fastanvil::biome::Biome::JaggedPeaks => Biome::JaggedPeaks,
            fastanvil::biome::Biome::LushCaves => Biome::LushCaves,
            fastanvil::biome::Biome::Meadow => Biome::Meadow,
            fastanvil::biome::Biome::NetherWastes => Biome::NetherWastes,
            fastanvil::biome::Biome::OldGrowthBirchForest => Biome::OldGrowthBirchForest,
            fastanvil::biome::Biome::OldGrowthPineTaiga => Biome::OldGrowthPineTaiga,
            fastanvil::biome::Biome::OldGrowthSpruceTaiga => Biome::OldGrowthSpruceTaiga,
            fastanvil::biome::Biome::SnowyPlains => Biome::SnowyPlains,
            fastanvil::biome::Biome::SnowySlopes => Biome::SnowySlopes,
            fastanvil::biome::Biome::SparseJungle => Biome::SparseJungle,
            fastanvil::biome::Biome::StonyPeaks => Biome::StonyPeaks,
            fastanvil::biome::Biome::StonyShore => Biome::StonyShore,
            fastanvil::biome::Biome::WindsweptForest => Biome::WindsweptForest,
            fastanvil::biome::Biome::WindsweptGravellyHills => Biome::WindsweptGravellyHills,
            fastanvil::biome::Biome::WindsweptHills => Biome::WindsweptHills,
            fastanvil::biome::Biome::WindsweptSavanna => Biome::WindsweptSavanna,
            fastanvil::biome::Biome::WoodedBadlands => Biome::WoodedBadlands,
            fastanvil::biome::Biome::MangroveSwamp => Biome::MangroveSwamp,
            fastanvil::biome::Biome::DeepDark => Biome::DeepDark,
            fastanvil::biome::Biome::Unknown => Biome::Unknown,
        }
    }
}
