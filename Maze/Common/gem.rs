#![allow(non_camel_case_types)]
use serde::Deserialize;
/// Describes the gems a tile can have
#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize)]
pub enum Gem {
    #[serde(rename(deserialize = "alexandrite-pear-shape"))]
    alexandrite_pear_shape,
    alexandrite,
    #[serde(rename(deserialize = "almandine-garnet"))]
    almandine_garnet,
    amethyst,
    ametrine,
    ammolite,
    apatite,
    aplite,
    #[serde(rename(deserialize = "apricot-square-radiant"))]
    apricot_square_radiant,
    aquamarine,
    #[serde(rename(deserialize = "australian-marquise"))]
    australian_marquise,
    aventurine,
    azurite,
    beryl,
    #[serde(rename(deserialize = "black-obsidian"))]
    black_obsidian,
    #[serde(rename(deserialize = "black-onyx"))]
    black_onyx,
    #[serde(rename(deserialize = "black-spinel-cushion"))]
    black_spinel_cushion,
    #[serde(rename(deserialize = "blue-ceylon-sapphire"))]
    blue_ceylon_sapphire,
    #[serde(rename(deserialize = "blue-cushion"))]
    blue_cushion,
    #[serde(rename(deserialize = "blue-pear-shape"))]
    blue_pear_shape,
    #[serde(rename(deserialize = "blue-spinel-heart"))]
    blue_spinel_heart,
    #[serde(rename(deserialize = "bulls-eye"))]
    bulls_eye,
    carnelian,
    #[serde(rename(deserialize = "chrome-diopside"))]
    chrome_diopside,
    #[serde(rename(deserialize = "chrysoberyl-cushion"))]
    chrysoberyl_cushion,
    chrysolite,
    #[serde(rename(deserialize = "citrine-checkerboard"))]
    citrine_checkerboard,
    citrine,
    clinohumite,
    #[serde(rename(deserialize = "color-change-oval"))]
    color_change_oval,
    cordierite,
    diamond,
    dumortierite,
    emerald,
    #[serde(rename(deserialize = "fancy-spinel-marquise"))]
    fancy_spinel_marquise,
    garnet,
    #[serde(rename(deserialize = "golden-diamond-cut"))]
    golden_diamond_cut,
    goldstone,
    grandidierite,
    #[serde(rename(deserialize = "gray-agate"))]
    gray_agate,
    #[serde(rename(deserialize = "green-aventurine"))]
    green_aventurine,
    #[serde(rename(deserialize = "green-beryl-antique"))]
    green_beryl_antique,
    #[serde(rename(deserialize = "green-beryl"))]
    green_beryl,
    #[serde(rename(deserialize = "green-princess-cut"))]
    green_princess_cut,
    #[serde(rename(deserialize = "grossular-garnet"))]
    grossular_garnet,
    hackmanite,
    heliotrope,
    hematite,
    #[serde(rename(deserialize = "iolite-emerald-cut"))]
    iolite_emerald_cut,
    jasper,
    jaspilite,
    #[serde(rename(deserialize = "kunzite-oval"))]
    kunzite_oval,
    kunzite,
    labradorite,
    #[serde(rename(deserialize = "lapis-lazuli"))]
    lapis_lazuli,
    #[serde(rename(deserialize = "lemon-quartz-briolette"))]
    lemon_quartz_briolette,
    magnesite,
    #[serde(rename(deserialize = "mexican-opal"))]
    mexican_opal,
    moonstone,
    #[serde(rename(deserialize = "morganite-oval"))]
    morganite_oval,
    #[serde(rename(deserialize = "moss-agate"))]
    moss_agate,
    #[serde(rename(deserialize = "orange-radiant"))]
    orange_radiant,
    #[serde(rename(deserialize = "padparadscha-oval"))]
    padparadscha_oval,
    #[serde(rename(deserialize = "padparadscha-sapphire"))]
    padparadscha_sapphire,
    peridot,
    #[serde(rename(deserialize = "pink-emerald-cut"))]
    pink_emerald_cut,
    #[serde(rename(deserialize = "pink-opal"))]
    pink_opal,
    #[serde(rename(deserialize = "pink-round"))]
    pink_round,
    #[serde(rename(deserialize = "pink-spinel-cushion"))]
    pink_spinel_cushion,
    prasiolite,
    prehnite,
    #[serde(rename(deserialize = "purple-cabochon"))]
    purple_cabochon,
    #[serde(rename(deserialize = "purple-oval"))]
    purple_oval,
    #[serde(rename(deserialize = "purple-spinel-trillion"))]
    purple_spinel_trillion,
    #[serde(rename(deserialize = "purple-square-cushion"))]
    purple_square_cushion,
    #[serde(rename(deserialize = "raw-beryl"))]
    raw_beryl,
    #[serde(rename(deserialize = "raw-citrine"))]
    raw_citrine,
    #[serde(rename(deserialize = "red-diamond"))]
    red_diamond,
    #[serde(rename(deserialize = "red-spinel-square-emerald-cut"))]
    red_spinel_square_emerald_cut,
    rhodonite,
    #[serde(rename(deserialize = "rock-quartz"))]
    rock_quartz,
    #[serde(rename(deserialize = "rose-quartz"))]
    rose_quartz,
    #[serde(rename(deserialize = "ruby-diamond-profile"))]
    ruby_diamond_profile,
    ruby,
    sphalerite,
    spinel,
    #[serde(rename(deserialize = "star-cabochon"))]
    star_cabochon,
    stilbite,
    sunstone,
    #[serde(rename(deserialize = "super-seven"))]
    super_seven,
    #[serde(rename(deserialize = "tanzanite-trillion"))]
    tanzanite_trillion,
    #[serde(rename(deserialize = "tigers-eye"))]
    tigers_eye,
    #[serde(rename(deserialize = "tourmaline-laser-cut"))]
    tourmaline_laser_cut,
    tourmaline,
    unakite,
    #[serde(rename(deserialize = "white-square"))]
    white_square,
    #[serde(rename(deserialize = "yellow-baguette"))]
    yellow_baguette,
    #[serde(rename(deserialize = "yellow-beryl-oval"))]
    yellow_beryl_oval,
    #[serde(rename(deserialize = "yellow-heart"))]
    yellow_heart,
    #[serde(rename(deserialize = "yellow-jasper"))]
    yellow_jasper,
    zircon,
    zoisite,
}
