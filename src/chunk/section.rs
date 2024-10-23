use std::collections::HashMap;

use bitcode::{Decode, Encode};

use crate::{static_enums::Biome, PropName, PropValue};

#[derive(Debug, Encode, Decode, Clone, PartialEq)]
pub struct Section {
    pub y: i8,
    pub block_states: SectionBlockStates,
    pub biome_palette: Vec<Biome>,
    pub block_light: Option<Vec<u8>>,
    pub sky_light: Option<Vec<u8>>,
}
#[derive(Debug, Encode, Decode, Clone, PartialEq)]
pub struct BlockState {
    pub block: String,
    pub properties: HashMap<PropName, PropValue>,
}

impl From<String> for BlockState {
    fn from(value: String) -> Self {
        let (block, s_state) = value.split_once("|").expect("Couldn't split blockstateid");
        let mut properties = HashMap::new();
        for prop in s_state.to_string().split(",") {
            let prop = prop.to_string();
            if prop.len() == 0 {
                continue;
            }
            let (name, value) = prop.split_once("=").expect("Couldn't split prop");
            properties.insert(
                PropName::from_str(name),
                PropValue::from_str(value),
            );
        }
        Self { block: block.to_string(), properties }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SectionBlockStates {
    pub palette: Vec<BlockState>,
    pub block_data: [u16; 4096],
}

#[derive(Default)]
pub struct SectionBlockStatesEncoder {
    var1: bitcode::__private::VariantEncoder<2usize>,
    var2: bitcode::__private::VariantEncoder<4usize>,
    palette: <Vec<BlockState> as bitcode::__private::Encode>::Encoder,
    unanymous_palette: <BlockState as bitcode::__private::Encode>::Encoder,
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
                    .chunks(4)
                    .map(|b| (b[3] << 6 | b[2] << 4 | b[1] << 2 | b[0]) as u8)
                    .collect()
            } else if t.palette.len() <= 16 {
                self.var2.encode(&1);
                t.block_data
                    .chunks(2)
                    .map(|b| (b[1] << 4 | b[0]) as u8)
                    .collect()
            } else if t.palette.len() <= u8::MAX as usize + 1 {
                self.var2.encode(&2);
                t.block_data.iter().map(|b| *b as u8).collect()
            } else {
                self.var2.encode(&3);
                t.block_data
                    .chunks(2)
                    .map(|b| {
                        let [_, o3, o2, o1] = (((b[0]  as u32) | (b[1]  as u32) << 12)).to_be_bytes();
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
    unanymous_palette: <BlockState as bitcode::__private::Decode<'de>>::Decoder,
    palette: <Vec<BlockState> as bitcode::__private::Decode<'de>>::Decoder,
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
        let (_block_data, palette) = match self.var1.decode() {
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
                                    q as u16 & 0x3,
                                    (q as u16 >> 2) & 0x3,
                                    (q as u16 >> 4) & 0x3,
                                    (q as u16 >> 6) & 0x3,
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
                            .map(|h| [h as u16 & 0xf, (h as u16 >> 4) & 0xf])
                            .flatten()
                            .collect(),
                        palette,
                    )
                }
                2u8 => {
                    let data: Vec<u8> = self.data.decode();

                    let palette = self.palette.decode();
                    (data.into_iter().map(|b| b as u16).collect(), palette)
                }
                3u8 => {
                    let twelves_data: Vec<u8> = self.data.decode();

                    let palette = self.palette.decode();
                    (
                        twelves_data
                            .chunks(3)
                            .map(|b| {
                                let t = u32::from_be_bytes([0, b[0], b[1], b[2]]);
                                [t as u16 & 0xfff, (t as u16 >> 12) & 0xfff]
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
        let mut block_data = [0; 4096];
        block_data.copy_from_slice(&_block_data);

        out.write(SectionBlockStates {
            palette,
            block_data,
        });
    }
}