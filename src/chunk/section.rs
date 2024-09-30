use bitcode::{Decode, Encode};

use crate::static_enums::Biome;

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