#![allow(non_camel_case_types)]
use std::collections::HashMap;

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
pub enum Gem {
    #[serde(rename = "alexandrite-pear-shape")]
    alexandrite_pear_shape,
    alexandrite,
    #[serde(rename = "almandine-garnet")]
    almandine_garnet,
    amethyst,
    ametrine,
    ammolite,
    apatite,
    aplite,
    #[serde(rename = "apricot-square-radiant")]
    apricot_square_radiant,
    aquamarine,
    #[serde(rename = "australian-marquise")]
    australian_marquise,
    aventurine,
    azurite,
    beryl,
    #[serde(rename = "black-obsidian")]
    black_obsidian,
    #[serde(rename = "black-onyx")]
    black_onyx,
    #[serde(rename = "black-spinel-cushion")]
    black_spinel_cushion,
    #[serde(rename = "blue-ceylon-sapphire")]
    blue_ceylon_sapphire,
    #[serde(rename = "blue-cushion")]
    blue_cushion,
    #[serde(rename = "blue-pear-shape")]
    blue_pear_shape,
    #[serde(rename = "blue-spinel-heart")]
    blue_spinel_heart,
    #[serde(rename = "bulls-eye")]
    bulls_eye,
    carnelian,
    #[serde(rename = "chrome-diopside")]
    chrome_diopside,
    #[serde(rename = "chrysoberyl-cushion")]
    chrysoberyl_cushion,
    chrysolite,
    #[serde(rename = "citrine-checkerboard")]
    citrine_checkerboard,
    citrine,
    clinohumite,
    #[serde(rename = "color-change-oval")]
    color_change_oval,
    cordierite,
    diamond,
    dumortierite,
    emerald,
    #[serde(rename = "fancy-spinel-marquise")]
    fancy_spinel_marquise,
    garnet,
    #[serde(rename = "golden-diamond-cut")]
    golden_diamond_cut,
    goldstone,
    grandidierite,
    #[serde(rename = "gray-agate")]
    gray_agate,
    #[serde(rename = "green-aventurine")]
    green_aventurine,
    #[serde(rename = "green-beryl-antique")]
    green_beryl_antique,
    #[serde(rename = "green-beryl")]
    green_beryl,
    #[serde(rename = "green-princess-cut")]
    green_princess_cut,
    #[serde(rename = "grossular-garnet")]
    grossular_garnet,
    hackmanite,
    heliotrope,
    hematite,
    #[serde(rename = "iolite-emerald-cut")]
    iolite_emerald_cut,
    jasper,
    jaspilite,
    #[serde(rename = "kunzite-oval")]
    kunzite_oval,
    kunzite,
    labradorite,
    #[serde(rename = "lapis-lazuli")]
    lapis_lazuli,
    #[serde(rename = "lemon-quartz-briolette")]
    lemon_quartz_briolette,
    magnesite,
    #[serde(rename = "mexican-opal")]
    mexican_opal,
    moonstone,
    #[serde(rename = "morganite-oval")]
    morganite_oval,
    #[serde(rename = "moss-agate")]
    moss_agate,
    #[serde(rename = "orange-radiant")]
    orange_radiant,
    #[serde(rename = "padparadscha-oval")]
    padparadscha_oval,
    #[serde(rename = "padparadscha-sapphire")]
    padparadscha_sapphire,
    peridot,
    #[serde(rename = "pink-emerald-cut")]
    pink_emerald_cut,
    #[serde(rename = "pink-opal")]
    pink_opal,
    #[serde(rename = "pink-round")]
    pink_round,
    #[serde(rename = "pink-spinel-cushion")]
    pink_spinel_cushion,
    prasiolite,
    prehnite,
    #[serde(rename = "purple-cabochon")]
    purple_cabochon,
    #[serde(rename = "purple-oval")]
    purple_oval,
    #[serde(rename = "purple-spinel-trillion")]
    purple_spinel_trillion,
    #[serde(rename = "purple-square-cushion")]
    purple_square_cushion,
    #[serde(rename = "raw-beryl")]
    raw_beryl,
    #[serde(rename = "raw-citrine")]
    raw_citrine,
    #[serde(rename = "red-diamond")]
    red_diamond,
    #[serde(rename = "red-spinel-square-emerald-cut")]
    red_spinel_square_emerald_cut,
    rhodonite,
    #[serde(rename = "rock-quartz")]
    rock_quartz,
    #[serde(rename = "rose-quartz")]
    rose_quartz,
    #[serde(rename = "ruby-diamond-profile")]
    ruby_diamond_profile,
    ruby,
    sphalerite,
    spinel,
    #[serde(rename = "star-cabochon")]
    star_cabochon,
    stilbite,
    sunstone,
    #[serde(rename = "super-seven")]
    super_seven,
    #[serde(rename = "tanzanite-trillion")]
    tanzanite_trillion,
    #[serde(rename = "tigers-eye")]
    tigers_eye,
    #[serde(rename = "tourmaline-laser-cut")]
    tourmaline_laser_cut,
    tourmaline,
    unakite,
    #[serde(rename = "white-square")]
    white_square,
    #[serde(rename = "yellow-baguette")]
    yellow_baguette,
    #[serde(rename = "yellow-beryl-oval")]
    yellow_beryl_oval,
    #[serde(rename = "yellow-heart")]
    yellow_heart,
    #[serde(rename = "yellow-jasper")]
    yellow_jasper,
    zircon,
    zoisite,
}

impl Gem {
    /// Creates an unordered pair of distinct `Gem`s. This function will produce unique pairs of
    /// gems until `num >= NUM_GEMS.pow(2))`
    ///
    /// # Errors
    ///
    /// If (num / NUM_GEMS) % NUM_GEMS == num % NUM_GEMS, returns `Err(())`
    pub fn pair_from_num(num: usize) -> UnorderedPair<Gem> {
        let left = Gem::from_num(num);
        let right = Gem::from_num(num * 2 + 1);
        UnorderedPair(left, right)
    }

    pub fn from_num(num: usize) -> Gem {
        match num % NUM_GEMS {
            0 => Gem::alexandrite_pear_shape,
            1 => Gem::alexandrite,
            2 => Gem::almandine_garnet,
            3 => Gem::amethyst,
            4 => Gem::ametrine,
            5 => Gem::ammolite,
            6 => Gem::apatite,
            7 => Gem::aplite,
            8 => Gem::apricot_square_radiant,
            9 => Gem::aquamarine,
            10 => Gem::australian_marquise,
            11 => Gem::aventurine,
            12 => Gem::azurite,
            13 => Gem::beryl,
            14 => Gem::black_obsidian,
            15 => Gem::black_onyx,
            16 => Gem::black_spinel_cushion,
            17 => Gem::blue_ceylon_sapphire,
            18 => Gem::blue_cushion,
            19 => Gem::blue_pear_shape,
            20 => Gem::blue_spinel_heart,
            21 => Gem::bulls_eye,
            22 => Gem::carnelian,
            23 => Gem::chrome_diopside,
            24 => Gem::chrysoberyl_cushion,
            25 => Gem::chrysolite,
            26 => Gem::citrine_checkerboard,
            27 => Gem::citrine,
            28 => Gem::clinohumite,
            29 => Gem::color_change_oval,
            30 => Gem::cordierite,
            31 => Gem::diamond,
            32 => Gem::dumortierite,
            33 => Gem::emerald,
            34 => Gem::fancy_spinel_marquise,
            35 => Gem::garnet,
            36 => Gem::golden_diamond_cut,
            37 => Gem::goldstone,
            38 => Gem::grandidierite,
            39 => Gem::gray_agate,
            40 => Gem::green_aventurine,
            41 => Gem::green_beryl_antique,
            42 => Gem::green_beryl,
            43 => Gem::green_princess_cut,
            44 => Gem::grossular_garnet,
            45 => Gem::hackmanite,
            46 => Gem::heliotrope,
            47 => Gem::hematite,
            48 => Gem::iolite_emerald_cut,
            49 => Gem::jasper,
            50 => Gem::jaspilite,
            51 => Gem::kunzite_oval,
            52 => Gem::kunzite,
            53 => Gem::labradorite,
            54 => Gem::lapis_lazuli,
            55 => Gem::lemon_quartz_briolette,
            56 => Gem::magnesite,
            57 => Gem::mexican_opal,
            58 => Gem::moonstone,
            59 => Gem::morganite_oval,
            60 => Gem::moss_agate,
            61 => Gem::orange_radiant,
            62 => Gem::padparadscha_oval,
            63 => Gem::padparadscha_sapphire,
            64 => Gem::peridot,
            65 => Gem::pink_emerald_cut,
            66 => Gem::pink_opal,
            67 => Gem::pink_round,
            68 => Gem::pink_spinel_cushion,
            69 => Gem::prasiolite,
            70 => Gem::prehnite,
            71 => Gem::purple_cabochon,
            72 => Gem::purple_oval,
            73 => Gem::purple_spinel_trillion,
            74 => Gem::purple_square_cushion,
            75 => Gem::raw_beryl,
            76 => Gem::raw_citrine,
            77 => Gem::red_diamond,
            78 => Gem::red_spinel_square_emerald_cut,
            79 => Gem::rhodonite,
            80 => Gem::rock_quartz,
            81 => Gem::rose_quartz,
            82 => Gem::ruby_diamond_profile,
            83 => Gem::ruby,
            84 => Gem::sphalerite,
            85 => Gem::spinel,
            86 => Gem::star_cabochon,
            87 => Gem::stilbite,
            88 => Gem::sunstone,
            89 => Gem::super_seven,
            90 => Gem::tanzanite_trillion,
            91 => Gem::tigers_eye,
            92 => Gem::tourmaline_laser_cut,
            93 => Gem::tourmaline,
            94 => Gem::unakite,
            95 => Gem::white_square,
            96 => Gem::yellow_baguette,
            97 => Gem::yellow_beryl_oval,
            98 => Gem::yellow_heart,
            99 => Gem::yellow_jasper,
            100 => Gem::zircon,
            101 => Gem::zoisite,
            _ => unreachable!("% NUM_GEMS never produces number > NUM_GEMS"),
        }
    }
}

macro_rules! gem_insert {
    ($map: ident, $name: ident) => {
        $map.insert(
            Gem::$name,
            egui_extras::RetainedImage::from_image_bytes(
                format!("{}.png", stringify!($name).replace("_", "-")),
                GEM_RESOURCE_DIR
                    .get_file(format!("{}.png", stringify!($name).replace("_", "-")))
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
        gem_insert!(m, alexandrite_pear_shape);
        gem_insert!(m, alexandrite);
        gem_insert!(m, almandine_garnet);
        gem_insert!(m, amethyst);
        gem_insert!(m, ametrine);
        gem_insert!(m, ammolite);
        gem_insert!(m, apatite);
        gem_insert!(m, aplite);
        gem_insert!(m, apricot_square_radiant);
        gem_insert!(m, aquamarine);
        gem_insert!(m, australian_marquise);
        gem_insert!(m, aventurine);
        gem_insert!(m, azurite);
        gem_insert!(m, beryl);
        gem_insert!(m, black_obsidian);
        gem_insert!(m, black_onyx);
        gem_insert!(m, black_spinel_cushion);
        gem_insert!(m, blue_ceylon_sapphire);
        gem_insert!(m, blue_cushion);
        gem_insert!(m, blue_pear_shape);
        gem_insert!(m, blue_spinel_heart);
        gem_insert!(m, bulls_eye);
        gem_insert!(m, carnelian);
        gem_insert!(m, chrome_diopside);
        gem_insert!(m, chrysoberyl_cushion);
        gem_insert!(m, chrysolite);
        gem_insert!(m, citrine_checkerboard);
        gem_insert!(m, citrine);
        gem_insert!(m, clinohumite);
        gem_insert!(m, color_change_oval);
        gem_insert!(m, cordierite);
        gem_insert!(m, diamond);
        gem_insert!(m, dumortierite);
        gem_insert!(m, emerald);
        gem_insert!(m, fancy_spinel_marquise);
        gem_insert!(m, garnet);
        gem_insert!(m, golden_diamond_cut);
        gem_insert!(m, goldstone);
        gem_insert!(m, grandidierite);
        gem_insert!(m, gray_agate);
        gem_insert!(m, green_aventurine);
        gem_insert!(m, green_beryl_antique);
        gem_insert!(m, green_beryl);
        gem_insert!(m, green_princess_cut);
        gem_insert!(m, grossular_garnet);
        gem_insert!(m, hackmanite);
        gem_insert!(m, heliotrope);
        gem_insert!(m, hematite);
        gem_insert!(m, iolite_emerald_cut);
        gem_insert!(m, jasper);
        gem_insert!(m, jaspilite);
        gem_insert!(m, kunzite_oval);
        gem_insert!(m, kunzite);
        gem_insert!(m, labradorite);
        gem_insert!(m, lapis_lazuli);
        gem_insert!(m, lemon_quartz_briolette);
        gem_insert!(m, magnesite);
        gem_insert!(m, mexican_opal);
        gem_insert!(m, moonstone);
        gem_insert!(m, morganite_oval);
        gem_insert!(m, moss_agate);
        gem_insert!(m, orange_radiant);
        gem_insert!(m, padparadscha_oval);
        gem_insert!(m, padparadscha_sapphire);
        gem_insert!(m, peridot);
        gem_insert!(m, pink_emerald_cut);
        gem_insert!(m, pink_opal);
        gem_insert!(m, pink_round);
        gem_insert!(m, pink_spinel_cushion);
        gem_insert!(m, prasiolite);
        gem_insert!(m, prehnite);
        gem_insert!(m, purple_cabochon);
        gem_insert!(m, purple_oval);
        gem_insert!(m, purple_spinel_trillion);
        gem_insert!(m, purple_square_cushion);
        gem_insert!(m, raw_beryl);
        gem_insert!(m, raw_citrine);
        gem_insert!(m, red_diamond);
        gem_insert!(m, red_spinel_square_emerald_cut);
        gem_insert!(m, rhodonite);
        gem_insert!(m, rock_quartz);
        gem_insert!(m, rose_quartz);
        gem_insert!(m, ruby_diamond_profile);
        gem_insert!(m, ruby);
        gem_insert!(m, sphalerite);
        gem_insert!(m, spinel);
        gem_insert!(m, star_cabochon);
        gem_insert!(m, stilbite);
        gem_insert!(m, sunstone);
        gem_insert!(m, super_seven);
        gem_insert!(m, tanzanite_trillion);
        gem_insert!(m, tigers_eye);
        gem_insert!(m, tourmaline_laser_cut);
        gem_insert!(m, tourmaline);
        gem_insert!(m, unakite);
        gem_insert!(m, white_square);
        gem_insert!(m, yellow_baguette);
        gem_insert!(m, yellow_beryl_oval);
        gem_insert!(m, yellow_heart);
        gem_insert!(m, yellow_jasper);
        gem_insert!(m, zircon);
        gem_insert!(m, zoisite);
        m
    };
}
