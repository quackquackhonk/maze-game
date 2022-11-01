#![allow(non_camel_case_types)]
use std::collections::HashMap;

use egui_extras::RetainedImage;
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;
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
