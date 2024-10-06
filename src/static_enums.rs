use bitcode::{Decode, Encode};

#[derive(Debug, Encode, Decode, Clone, PartialEq)]
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
#[cfg(feature = "fastanvil")]
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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Decode, Encode)]
pub enum PropName {
    Age,
    Attached,
    Attachment,
    Axis,
    Berries,
    Bites,
    Bloom,
    Bottom,
    CanSummon,
    Candles,
    Charges,
    Conditional,
    Cracked,
    Delay,
    Disarmed,
    Distance,
    Down,
    Drag,
    Dusted,
    East,
    Eggs,
    Enabled,
    Extended,
    Eye,
    Face,
    Facing,
    FlowerAmount,
    Half,
    Hanging,
    HasBook,
    HasBottle0,
    HasBottle1,
    HasBottle2,
    HasRecord,
    Hatch,
    Hinge,
    HoneyLevel,
    InWall,
    Instrument,
    Inverted,
    Layers,
    Leaves,
    Level,
    Lit,
    Locked,
    Mode,
    Moisture,
    North,
    Note,
    Occupied,
    Open,
    Orientation,
    Part,
    Persistent,
    Pickles,
    Power,
    Powered,
    Rotation,
    SculkSensorPhase,
    Shape,
    Short,
    Shrieking,
    SignalFire,
    Slot0Occupied,
    Slot1Occupied,
    Slot2Occupied,
    Slot3Occupied,
    Slot4Occupied,
    Slot5Occupied,
    Snowy,
    South,
    Stage,
    Thickness,
    Tilt,
    Triggered,
    Type,
    Unstable,
    Up,
    VerticalDirection,
    Waterlogged,
    West,
    Other(String),
}
impl PropName {
    pub fn from_str(name: &str) -> Self {
        match name {
            "age" => PropName::Age,
            "attached" => PropName::Attached,
            "attachment" => PropName::Attachment,
            "axis" => PropName::Axis,
            "berries" => PropName::Berries,
            "bites" => PropName::Bites,
            "bloom" => PropName::Bloom,
            "bottom" => PropName::Bottom,
            "can_summon" => PropName::CanSummon,
            "candles" => PropName::Candles,
            "charges" => PropName::Charges,
            "conditional" => PropName::Conditional,
            "cracked" => PropName::Cracked,
            "delay" => PropName::Delay,
            "disarmed" => PropName::Disarmed,
            "distance" => PropName::Distance,
            "down" => PropName::Down,
            "drag" => PropName::Drag,
            "dusted" => PropName::Dusted,
            "east" => PropName::East,
            "eggs" => PropName::Eggs,
            "enabled" => PropName::Enabled,
            "extended" => PropName::Extended,
            "eye" => PropName::Eye,
            "face" => PropName::Face,
            "facing" => PropName::Facing,
            "flower_amount" => PropName::FlowerAmount,
            "half" => PropName::Half,
            "hanging" => PropName::Hanging,
            "has_book" => PropName::HasBook,
            "has_bottle_0" => PropName::HasBottle0,
            "has_bottle_1" => PropName::HasBottle1,
            "has_bottle_2" => PropName::HasBottle2,
            "has_record" => PropName::HasRecord,
            "hatch" => PropName::Hatch,
            "hinge" => PropName::Hinge,
            "honey_level" => PropName::HoneyLevel,
            "in_wall" => PropName::InWall,
            "instrument" => PropName::Instrument,
            "inverted" => PropName::Inverted,
            "layers" => PropName::Layers,
            "leaves" => PropName::Leaves,
            "level" => PropName::Level,
            "lit" => PropName::Lit,
            "locked" => PropName::Locked,
            "mode" => PropName::Mode,
            "moisture" => PropName::Moisture,
            "north" => PropName::North,
            "note" => PropName::Note,
            "occupied" => PropName::Occupied,
            "open" => PropName::Open,
            "orientation" => PropName::Orientation,
            "part" => PropName::Part,
            "persistent" => PropName::Persistent,
            "pickles" => PropName::Pickles,
            "power" => PropName::Power,
            "powered" => PropName::Powered,
            "rotation" => PropName::Rotation,
            "sculk_sensor_phase" => PropName::SculkSensorPhase,
            "shape" => PropName::Shape,
            "short" => PropName::Short,
            "shrieking" => PropName::Shrieking,
            "signal_fire" => PropName::SignalFire,
            "slot_0_occupied" => PropName::Slot0Occupied,
            "slot_1_occupied" => PropName::Slot1Occupied,
            "slot_2_occupied" => PropName::Slot2Occupied,
            "slot_3_occupied" => PropName::Slot3Occupied,
            "slot_4_occupied" => PropName::Slot4Occupied,
            "slot_5_occupied" => PropName::Slot5Occupied,
            "snowy" => PropName::Snowy,
            "south" => PropName::South,
            "stage" => PropName::Stage,
            "thickness" => PropName::Thickness,
            "tilt" => PropName::Tilt,
            "triggered" => PropName::Triggered,
            "type" => PropName::Type,
            "unstable" => PropName::Unstable,
            "up" => PropName::Up,
            "vertical_direction" => PropName::VerticalDirection,
            "waterlogged" => PropName::Waterlogged,
            "west" => PropName::West,
            s => PropName::Other(s.to_string()),
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            PropName::Age => "age",
            PropName::Attached => "attached",
            PropName::Attachment => "attachment",
            PropName::Axis => "axis",
            PropName::Berries => "berries",
            PropName::Bites => "bites",
            PropName::Bloom => "bloom",
            PropName::Bottom => "bottom",
            PropName::CanSummon => "can_summon",
            PropName::Candles => "candles",
            PropName::Charges => "charges",
            PropName::Conditional => "conditional",
            PropName::Cracked => "cracked",
            PropName::Delay => "delay",
            PropName::Disarmed => "disarmed",
            PropName::Distance => "distance",
            PropName::Down => "down",
            PropName::Drag => "drag",
            PropName::Dusted => "dusted",
            PropName::East => "east",
            PropName::Eggs => "eggs",
            PropName::Enabled => "enabled",
            PropName::Extended => "extended",
            PropName::Eye => "eye",
            PropName::Face => "face",
            PropName::Facing => "facing",
            PropName::FlowerAmount => "flower_amount",
            PropName::Half => "half",
            PropName::Hanging => "hanging",
            PropName::HasBook => "has_book",
            PropName::HasBottle0 => "has_bottle_0",
            PropName::HasBottle1 => "has_bottle_1",
            PropName::HasBottle2 => "has_bottle_2",
            PropName::HasRecord => "has_record",
            PropName::Hatch => "hatch",
            PropName::Hinge => "hinge",
            PropName::HoneyLevel => "honey_level",
            PropName::InWall => "in_wall",
            PropName::Instrument => "instrument",
            PropName::Inverted => "inverted",
            PropName::Layers => "layers",
            PropName::Leaves => "leaves",
            PropName::Level => "level",
            PropName::Lit => "lit",
            PropName::Locked => "locked",
            PropName::Mode => "mode",
            PropName::Moisture => "moisture",
            PropName::North => "north",
            PropName::Note => "note",
            PropName::Occupied => "occupied",
            PropName::Open => "open",
            PropName::Orientation => "orientation",
            PropName::Part => "part",
            PropName::Persistent => "persistent",
            PropName::Pickles => "pickles",
            PropName::Power => "power",
            PropName::Powered => "powered",
            PropName::Rotation => "rotation",
            PropName::SculkSensorPhase => "sculk_sensor_phase",
            PropName::Shape => "shape",
            PropName::Short => "short",
            PropName::Shrieking => "shrieking",
            PropName::SignalFire => "signal_fire",
            PropName::Slot0Occupied => "slot_0_occupied",
            PropName::Slot1Occupied => "slot_1_occupied",
            PropName::Slot2Occupied => "slot_2_occupied",
            PropName::Slot3Occupied => "slot_3_occupied",
            PropName::Slot4Occupied => "slot_4_occupied",
            PropName::Slot5Occupied => "slot_5_occupied",
            PropName::Snowy => "snowy",
            PropName::South => "south",
            PropName::Stage => "stage",
            PropName::Thickness => "thickness",
            PropName::Tilt => "tilt",
            PropName::Triggered => "triggered",
            PropName::Type => "type",
            PropName::Unstable => "unstable",
            PropName::Up => "up",
            PropName::VerticalDirection => "vertical_direction",
            PropName::Waterlogged => "waterlogged",
            PropName::West => "west",
            PropName::Other(s) => s.as_str(),
        }
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Decode, Encode)]
pub enum PropValue {
    _0,
    _1,
    _10,
    _11,
    _12,
    _13,
    _14,
    _15,
    _16,
    _17,
    _18,
    _19,
    _2,
    _20,
    _21,
    _22,
    _23,
    _24,
    _25,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    Active,
    AscendingEast,
    AscendingNorth,
    AscendingSouth,
    AscendingWest,
    Banjo,
    Base,
    Basedrum,
    Bass,
    Bell,
    Bit,
    Bottom,
    Ceiling,
    Chime,
    Compare,
    Cooldown,
    Corner,
    CowBell,
    Creeper,
    CustomHead,
    Data,
    Didgeridoo,
    Double,
    DoubleWall,
    Down,
    DownEast,
    DownNorth,
    DownSouth,
    DownWest,
    Dragon,
    East,
    EastUp,
    EastWest,
    False,
    Floor,
    Flute,
    Foot,
    Frustum,
    Full,
    Guitar,
    Harp,
    Hat,
    Head,
    Inactive,
    InnerLeft,
    InnerRight,
    IronXylophone,
    Large,
    Left,
    Load,
    Low,
    Lower,
    Middle,
    None,
    Normal,
    North,
    NorthEast,
    NorthSouth,
    NorthUp,
    NorthWest,
    OuterLeft,
    OuterRight,
    Partial,
    Piglin,
    Pling,
    Right,
    Save,
    Side,
    Single,
    SingleWall,
    Skeleton,
    Small,
    Snare,
    South,
    SouthEast,
    SouthUp,
    SouthWest,
    Sticky,
    Straight,
    Subtract,
    Tall,
    Tip,
    TipMerge,
    Top,
    True,
    Unstable,
    Up,
    UpEast,
    UpNorth,
    UpSouth,
    UpWest,
    Upper,
    Wall,
    West,
    WestUp,
    WitherSkeleton,
    X,
    Xylophone,
    Y,
    Z,
    Zombie,
    Other(String),
}
impl PropValue {
    pub fn from_str(name: &str) -> Self {
        match name {
            "0" => PropValue::_0,
            "1" => PropValue::_1,
            "10" => PropValue::_10,
            "11" => PropValue::_11,
            "12" => PropValue::_12,
            "13" => PropValue::_13,
            "14" => PropValue::_14,
            "15" => PropValue::_15,
            "16" => PropValue::_16,
            "17" => PropValue::_17,
            "18" => PropValue::_18,
            "19" => PropValue::_19,
            "2" => PropValue::_2,
            "20" => PropValue::_20,
            "21" => PropValue::_21,
            "22" => PropValue::_22,
            "23" => PropValue::_23,
            "24" => PropValue::_24,
            "25" => PropValue::_25,
            "3" => PropValue::_3,
            "4" => PropValue::_4,
            "5" => PropValue::_5,
            "6" => PropValue::_6,
            "7" => PropValue::_7,
            "8" => PropValue::_8,
            "9" => PropValue::_9,
            "active" => PropValue::Active,
            "ascending_east" => PropValue::AscendingEast,
            "ascending_north" => PropValue::AscendingNorth,
            "ascending_south" => PropValue::AscendingSouth,
            "ascending_west" => PropValue::AscendingWest,
            "banjo" => PropValue::Banjo,
            "base" => PropValue::Base,
            "basedrum" => PropValue::Basedrum,
            "bass" => PropValue::Bass,
            "bell" => PropValue::Bell,
            "bit" => PropValue::Bit,
            "bottom" => PropValue::Bottom,
            "ceiling" => PropValue::Ceiling,
            "chime" => PropValue::Chime,
            "compare" => PropValue::Compare,
            "cooldown" => PropValue::Cooldown,
            "corner" => PropValue::Corner,
            "cow_bell" => PropValue::CowBell,
            "creeper" => PropValue::Creeper,
            "custom_head" => PropValue::CustomHead,
            "data" => PropValue::Data,
            "didgeridoo" => PropValue::Didgeridoo,
            "double" => PropValue::Double,
            "double_wall" => PropValue::DoubleWall,
            "down" => PropValue::Down,
            "down_east" => PropValue::DownEast,
            "down_north" => PropValue::DownNorth,
            "down_south" => PropValue::DownSouth,
            "down_west" => PropValue::DownWest,
            "dragon" => PropValue::Dragon,
            "east" => PropValue::East,
            "east_up" => PropValue::EastUp,
            "east_west" => PropValue::EastWest,
            "false" => PropValue::False,
            "floor" => PropValue::Floor,
            "flute" => PropValue::Flute,
            "foot" => PropValue::Foot,
            "frustum" => PropValue::Frustum,
            "full" => PropValue::Full,
            "guitar" => PropValue::Guitar,
            "harp" => PropValue::Harp,
            "hat" => PropValue::Hat,
            "head" => PropValue::Head,
            "inactive" => PropValue::Inactive,
            "inner_left" => PropValue::InnerLeft,
            "inner_right" => PropValue::InnerRight,
            "iron_xylophone" => PropValue::IronXylophone,
            "large" => PropValue::Large,
            "left" => PropValue::Left,
            "load" => PropValue::Load,
            "low" => PropValue::Low,
            "lower" => PropValue::Lower,
            "middle" => PropValue::Middle,
            "none" => PropValue::None,
            "normal" => PropValue::Normal,
            "north" => PropValue::North,
            "north_east" => PropValue::NorthEast,
            "north_south" => PropValue::NorthSouth,
            "north_up" => PropValue::NorthUp,
            "north_west" => PropValue::NorthWest,
            "outer_left" => PropValue::OuterLeft,
            "outer_right" => PropValue::OuterRight,
            "partial" => PropValue::Partial,
            "piglin" => PropValue::Piglin,
            "pling" => PropValue::Pling,
            "right" => PropValue::Right,
            "save" => PropValue::Save,
            "side" => PropValue::Side,
            "single" => PropValue::Single,
            "single_wall" => PropValue::SingleWall,
            "skeleton" => PropValue::Skeleton,
            "small" => PropValue::Small,
            "snare" => PropValue::Snare,
            "south" => PropValue::South,
            "south_east" => PropValue::SouthEast,
            "south_up" => PropValue::SouthUp,
            "south_west" => PropValue::SouthWest,
            "sticky" => PropValue::Sticky,
            "straight" => PropValue::Straight,
            "subtract" => PropValue::Subtract,
            "tall" => PropValue::Tall,
            "tip" => PropValue::Tip,
            "tip_merge" => PropValue::TipMerge,
            "top" => PropValue::Top,
            "true" => PropValue::True,
            "unstable" => PropValue::Unstable,
            "up" => PropValue::Up,
            "up_east" => PropValue::UpEast,
            "up_north" => PropValue::UpNorth,
            "up_south" => PropValue::UpSouth,
            "up_west" => PropValue::UpWest,
            "upper" => PropValue::Upper,
            "wall" => PropValue::Wall,
            "west" => PropValue::West,
            "west_up" => PropValue::WestUp,
            "wither_skeleton" => PropValue::WitherSkeleton,
            "x" => PropValue::X,
            "xylophone" => PropValue::Xylophone,
            "y" => PropValue::Y,
            "z" => PropValue::Z,
            "zombie" => PropValue::Zombie,
            s => PropValue::Other(s.to_string()),
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            PropValue::_0 => "0",
            PropValue::_1 => "1",
            PropValue::_10 => "10",
            PropValue::_11 => "11",
            PropValue::_12 => "12",
            PropValue::_13 => "13",
            PropValue::_14 => "14",
            PropValue::_15 => "15",
            PropValue::_16 => "16",
            PropValue::_17 => "17",
            PropValue::_18 => "18",
            PropValue::_19 => "19",
            PropValue::_2 => "2",
            PropValue::_20 => "20",
            PropValue::_21 => "21",
            PropValue::_22 => "22",
            PropValue::_23 => "23",
            PropValue::_24 => "24",
            PropValue::_25 => "25",
            PropValue::_3 => "3",
            PropValue::_4 => "4",
            PropValue::_5 => "5",
            PropValue::_6 => "6",
            PropValue::_7 => "7",
            PropValue::_8 => "8",
            PropValue::_9 => "9",
            PropValue::Active => "active",
            PropValue::AscendingEast => "ascending_east",
            PropValue::AscendingNorth => "ascending_north",
            PropValue::AscendingSouth => "ascending_south",
            PropValue::AscendingWest => "ascending_west",
            PropValue::Banjo => "banjo",
            PropValue::Base => "base",
            PropValue::Basedrum => "basedrum",
            PropValue::Bass => "bass",
            PropValue::Bell => "bell",
            PropValue::Bit => "bit",
            PropValue::Bottom => "bottom",
            PropValue::Ceiling => "ceiling",
            PropValue::Chime => "chime",
            PropValue::Compare => "compare",
            PropValue::Cooldown => "cooldown",
            PropValue::Corner => "corner",
            PropValue::CowBell => "cow_bell",
            PropValue::Creeper => "creeper",
            PropValue::CustomHead => "custom_head",
            PropValue::Data => "data",
            PropValue::Didgeridoo => "didgeridoo",
            PropValue::Double => "double",
            PropValue::DoubleWall => "double_wall",
            PropValue::Down => "down",
            PropValue::DownEast => "down_east",
            PropValue::DownNorth => "down_north",
            PropValue::DownSouth => "down_south",
            PropValue::DownWest => "down_west",
            PropValue::Dragon => "dragon",
            PropValue::East => "east",
            PropValue::EastUp => "east_up",
            PropValue::EastWest => "east_west",
            PropValue::False => "false",
            PropValue::Floor => "floor",
            PropValue::Flute => "flute",
            PropValue::Foot => "foot",
            PropValue::Frustum => "frustum",
            PropValue::Full => "full",
            PropValue::Guitar => "guitar",
            PropValue::Harp => "harp",
            PropValue::Hat => "hat",
            PropValue::Head => "head",
            PropValue::Inactive => "inactive",
            PropValue::InnerLeft => "inner_left",
            PropValue::InnerRight => "inner_right",
            PropValue::IronXylophone => "iron_xylophone",
            PropValue::Large => "large",
            PropValue::Left => "left",
            PropValue::Load => "load",
            PropValue::Low => "low",
            PropValue::Lower => "lower",
            PropValue::Middle => "middle",
            PropValue::None => "none",
            PropValue::Normal => "normal",
            PropValue::North => "north",
            PropValue::NorthEast => "north_east",
            PropValue::NorthSouth => "north_south",
            PropValue::NorthUp => "north_up",
            PropValue::NorthWest => "north_west",
            PropValue::OuterLeft => "outer_left",
            PropValue::OuterRight => "outer_right",
            PropValue::Partial => "partial",
            PropValue::Piglin => "piglin",
            PropValue::Pling => "pling",
            PropValue::Right => "right",
            PropValue::Save => "save",
            PropValue::Side => "side",
            PropValue::Single => "single",
            PropValue::SingleWall => "single_wall",
            PropValue::Skeleton => "skeleton",
            PropValue::Small => "small",
            PropValue::Snare => "snare",
            PropValue::South => "south",
            PropValue::SouthEast => "south_east",
            PropValue::SouthUp => "south_up",
            PropValue::SouthWest => "south_west",
            PropValue::Sticky => "sticky",
            PropValue::Straight => "straight",
            PropValue::Subtract => "subtract",
            PropValue::Tall => "tall",
            PropValue::Tip => "tip",
            PropValue::TipMerge => "tip_merge",
            PropValue::Top => "top",
            PropValue::True => "true",
            PropValue::Unstable => "unstable",
            PropValue::Up => "up",
            PropValue::UpEast => "up_east",
            PropValue::UpNorth => "up_north",
            PropValue::UpSouth => "up_south",
            PropValue::UpWest => "up_west",
            PropValue::Upper => "upper",
            PropValue::Wall => "wall",
            PropValue::West => "west",
            PropValue::WestUp => "west_up",
            PropValue::WitherSkeleton => "wither_skeleton",
            PropValue::X => "x",
            PropValue::Xylophone => "xylophone",
            PropValue::Y => "y",
            PropValue::Z => "z",
            PropValue::Zombie => "zombie",
            PropValue::Other(s) => s.as_str(),
        }
    }
}