#![allow(non_camel_case_types)]
use std::collections::HashMap;

use convert_case::{Case, Casing};
use egui_extras::RetainedImage;
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;
use unordered_pair::UnorderedPair;

// The number of different gems
const NUM_GEMS: usize = 102;

/// Describes the gems a tile can have
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Gem {
    AlexandritePearShape,
    Alexandrite,
    AlmandineGarnet,
    Amethyst,
    Ametrine,
    Ammolite,
    Apatite,
    Aplite,
    ApricotSquareRadiant,
    Aquamarine,
    AustralianMarquise,
    Aventurine,
    Azurite,
    Beryl,
    BlackObsidian,
    BlackOnyx,
    BlackSpinelCushion,
    BlueCeylonSapphire,
    BlueCushion,
    BluePearShape,
    BlueSpinelHeart,
    BullsEye,
    Carnelian,
    ChromeDiopside,
    ChrysoberylCushion,
    Chrysolite,
    CitrineCheckerboard,
    Citrine,
    Clinohumite,
    ColorChangeOval,
    Cordierite,
    Diamond,
    Dumortierite,
    Emerald,
    FancySpinelMarquise,
    Garnet,
    GoldenDiamondCut,
    Goldstone,
    Grandidierite,
    GrayAgate,
    GreenAventurine,
    GreenBerylAntique,
    GreenBeryl,
    GreenPrincessCut,
    GrossularGarnet,
    Hackmanite,
    Heliotrope,
    Hematite,
    IoliteEmeraldCut,
    Jasper,
    Jaspilite,
    KunziteOval,
    Kunzite,
    Labradorite,
    LapisLazuli,
    LemonQuartzBriolette,
    Magnesite,
    MexicanOpal,
    Moonstone,
    MorganiteOval,
    MossAgate,
    OrangeRadiant,
    PadparadschaOval,
    PadparadschaSapphire,
    Peridot,
    PinkEmeraldCut,
    PinkOpal,
    PinkRound,
    PinkSpinelCushion,
    Prasiolite,
    Prehnite,
    PurpleCabochon,
    PurpleOval,
    PurpleSpinelTrillion,
    PurpleSquareCushion,
    RawBeryl,
    RawCitrine,
    RedDiamond,
    RedSpinelSquareEmeraldCut,
    Rhodonite,
    RockQuartz,
    RoseQuartz,
    RubyDiamondProfile,
    Ruby,
    Sphalerite,
    Spinel,
    StarCabochon,
    Stilbite,
    Sunstone,
    SuperSeven,
    TanzaniteTrillion,
    TigersEye,
    TourmalineLaserCut,
    Tourmaline,
    Unakite,
    WhiteSquare,
    YellowBaguette,
    YellowBerylOval,
    YellowHeart,
    YellowJasper,
    Zircon,
    Zoisite,
}

impl Gem {
    /// Creates an unordered pair of distinct `Gem`s. This function will produce unique pairs of
    /// gems until `num >= NUM_GEMS.pow(2))`
    pub fn pair_from_num(num: usize) -> UnorderedPair<Gem> {
        let left = Gem::from_num(num / NUM_GEMS);
        let right = Gem::from_num(num % NUM_GEMS);
        UnorderedPair(left, right)
    }

    pub fn from_num(num: usize) -> Gem {
        use Gem::*;
        match num % NUM_GEMS {
            0 => AlexandritePearShape,
            1 => Alexandrite,
            2 => AlmandineGarnet,
            3 => Amethyst,
            4 => Ametrine,
            5 => Ammolite,
            6 => Apatite,
            7 => Aplite,
            8 => ApricotSquareRadiant,
            9 => Aquamarine,
            10 => AustralianMarquise,
            11 => Aventurine,
            12 => Azurite,
            13 => Beryl,
            14 => BlackObsidian,
            15 => BlackOnyx,
            16 => BlackSpinelCushion,
            17 => BlueCeylonSapphire,
            18 => BlueCushion,
            19 => BluePearShape,
            20 => BlueSpinelHeart,
            21 => BullsEye,
            22 => Carnelian,
            23 => ChromeDiopside,
            24 => ChrysoberylCushion,
            25 => Chrysolite,
            26 => CitrineCheckerboard,
            27 => Citrine,
            28 => Clinohumite,
            29 => ColorChangeOval,
            30 => Cordierite,
            31 => Diamond,
            32 => Dumortierite,
            33 => Emerald,
            34 => FancySpinelMarquise,
            35 => Garnet,
            36 => GoldenDiamondCut,
            37 => Goldstone,
            38 => Grandidierite,
            39 => GrayAgate,
            40 => GreenAventurine,
            41 => GreenBerylAntique,
            42 => GreenBeryl,
            43 => GreenPrincessCut,
            44 => GrossularGarnet,
            45 => Hackmanite,
            46 => Heliotrope,
            47 => Hematite,
            48 => IoliteEmeraldCut,
            49 => Jasper,
            50 => Jaspilite,
            51 => KunziteOval,
            52 => Kunzite,
            53 => Labradorite,
            54 => LapisLazuli,
            55 => LemonQuartzBriolette,
            56 => Magnesite,
            57 => MexicanOpal,
            58 => Moonstone,
            59 => MorganiteOval,
            60 => MossAgate,
            61 => OrangeRadiant,
            62 => PadparadschaOval,
            63 => PadparadschaSapphire,
            64 => Peridot,
            65 => PinkEmeraldCut,
            66 => PinkOpal,
            67 => PinkRound,
            68 => PinkSpinelCushion,
            69 => Prasiolite,
            70 => Prehnite,
            71 => PurpleCabochon,
            72 => PurpleOval,
            73 => PurpleSpinelTrillion,
            74 => PurpleSquareCushion,
            75 => RawBeryl,
            76 => RawCitrine,
            77 => RedDiamond,
            78 => RedSpinelSquareEmeraldCut,
            79 => Rhodonite,
            80 => RockQuartz,
            81 => RoseQuartz,
            82 => RubyDiamondProfile,
            83 => Ruby,
            84 => Sphalerite,
            85 => Spinel,
            86 => StarCabochon,
            87 => Stilbite,
            88 => Sunstone,
            89 => SuperSeven,
            90 => TanzaniteTrillion,
            91 => TigersEye,
            92 => TourmalineLaserCut,
            93 => Tourmaline,
            94 => Unakite,
            95 => WhiteSquare,
            96 => YellowBaguette,
            97 => YellowBerylOval,
            98 => YellowHeart,
            99 => YellowJasper,
            100 => Zircon,
            101 => Zoisite,
            _ => unreachable!(),
        }
    }
}

macro_rules! gem_insert {
    ($map: ident, $name: ident) => {
        $map.insert(
            Gem::$name,
            egui_extras::RetainedImage::from_image_bytes(
                format!("{}.png", stringify!($name).to_case(Case::Kebab)),
                GEM_RESOURCE_DIR
                    .get_file(format!("{}.png", stringify!($name).to_case(Case::Kebab)))
                    .unwrap()
                    .contents(),
            )
            .unwrap(),
        )
    };
}

const GEM_RESOURCE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../Resources/gems");

lazy_static! {
    pub static ref GEM_IMGS: HashMap<Gem, RetainedImage> = {
        let mut m = HashMap::new();
        gem_insert!(m, AlexandritePearShape);
        gem_insert!(m, Alexandrite);
        gem_insert!(m, AlmandineGarnet);
        gem_insert!(m, Amethyst);
        gem_insert!(m, Ametrine);
        gem_insert!(m, Ammolite);
        gem_insert!(m, Apatite);
        gem_insert!(m, Aplite);
        gem_insert!(m, ApricotSquareRadiant);
        gem_insert!(m, Aquamarine);
        gem_insert!(m, AustralianMarquise);
        gem_insert!(m, Aventurine);
        gem_insert!(m, Azurite);
        gem_insert!(m, Beryl);
        gem_insert!(m, BlackObsidian);
        gem_insert!(m, BlackOnyx);
        gem_insert!(m, BlackSpinelCushion);
        gem_insert!(m, BlueCeylonSapphire);
        gem_insert!(m, BlueCushion);
        gem_insert!(m, BluePearShape);
        gem_insert!(m, BlueSpinelHeart);
        gem_insert!(m, BullsEye);
        gem_insert!(m, Carnelian);
        gem_insert!(m, ChromeDiopside);
        gem_insert!(m, ChrysoberylCushion);
        gem_insert!(m, Chrysolite);
        gem_insert!(m, CitrineCheckerboard);
        gem_insert!(m, Citrine);
        gem_insert!(m, Clinohumite);
        gem_insert!(m, ColorChangeOval);
        gem_insert!(m, Cordierite);
        gem_insert!(m, Diamond);
        gem_insert!(m, Dumortierite);
        gem_insert!(m, Emerald);
        gem_insert!(m, FancySpinelMarquise);
        gem_insert!(m, Garnet);
        gem_insert!(m, GoldenDiamondCut);
        gem_insert!(m, Goldstone);
        gem_insert!(m, Grandidierite);
        gem_insert!(m, GrayAgate);
        gem_insert!(m, GreenAventurine);
        gem_insert!(m, GreenBerylAntique);
        gem_insert!(m, GreenBeryl);
        gem_insert!(m, GreenPrincessCut);
        gem_insert!(m, GrossularGarnet);
        gem_insert!(m, Hackmanite);
        gem_insert!(m, Heliotrope);
        gem_insert!(m, Hematite);
        gem_insert!(m, IoliteEmeraldCut);
        gem_insert!(m, Jasper);
        gem_insert!(m, Jaspilite);
        gem_insert!(m, KunziteOval);
        gem_insert!(m, Kunzite);
        gem_insert!(m, Labradorite);
        gem_insert!(m, LapisLazuli);
        gem_insert!(m, LemonQuartzBriolette);
        gem_insert!(m, Magnesite);
        gem_insert!(m, MexicanOpal);
        gem_insert!(m, Moonstone);
        gem_insert!(m, MorganiteOval);
        gem_insert!(m, MossAgate);
        gem_insert!(m, OrangeRadiant);
        gem_insert!(m, PadparadschaOval);
        gem_insert!(m, PadparadschaSapphire);
        gem_insert!(m, Peridot);
        gem_insert!(m, PinkEmeraldCut);
        gem_insert!(m, PinkOpal);
        gem_insert!(m, PinkRound);
        gem_insert!(m, PinkSpinelCushion);
        gem_insert!(m, Prasiolite);
        gem_insert!(m, Prehnite);
        gem_insert!(m, PurpleCabochon);
        gem_insert!(m, PurpleOval);
        gem_insert!(m, PurpleSpinelTrillion);
        gem_insert!(m, PurpleSquareCushion);
        gem_insert!(m, RawBeryl);
        gem_insert!(m, RawCitrine);
        gem_insert!(m, RedDiamond);
        gem_insert!(m, RedSpinelSquareEmeraldCut);
        gem_insert!(m, Rhodonite);
        gem_insert!(m, RockQuartz);
        gem_insert!(m, RoseQuartz);
        gem_insert!(m, RubyDiamondProfile);
        gem_insert!(m, Ruby);
        gem_insert!(m, Sphalerite);
        gem_insert!(m, Spinel);
        gem_insert!(m, StarCabochon);
        gem_insert!(m, Stilbite);
        gem_insert!(m, Sunstone);
        gem_insert!(m, SuperSeven);
        gem_insert!(m, TanzaniteTrillion);
        gem_insert!(m, TigersEye);
        gem_insert!(m, TourmalineLaserCut);
        gem_insert!(m, Tourmaline);
        gem_insert!(m, Unakite);
        gem_insert!(m, WhiteSquare);
        gem_insert!(m, YellowBaguette);
        gem_insert!(m, YellowBerylOval);
        gem_insert!(m, YellowHeart);
        gem_insert!(m, YellowJasper);
        gem_insert!(m, Zircon);
        gem_insert!(m, Zoisite);
        m
    };
}
