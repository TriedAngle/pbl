//! Values taken from:
//! https://pepcalc.com/notes.php?mw

/// in [Da]
#[non_exhaustive]
pub struct Weight;

impl Weight {
    pub const A: f32 = 71.07793;
    pub const C: f32 = 103.1454;
    pub const D: f32 = 115.0873;
    pub const E: f32 = 129.1139;
    pub const F: f32 = 147.1734;
    pub const G: f32 = 57.05138;
    pub const H: f32 = 137.1394;
    pub const I: f32 = 113.1576;
    pub const K: f32 = 128.1724;
    pub const L: f32 = 113.1576;
    pub const M: f32 = 131.1985;
    pub const N: f32 = 114.1028;
    pub const P: f32 = 97.11508;
    pub const Q: f32 = 128.1293;
    pub const R: f32 = 156.1861;
    pub const S: f32 = 87.07733;
    pub const T: f32 = 101.1039;
    pub const V: f32 = 99.13103;
    pub const W: f32 = 186.2095;
    pub const Y: f32 = 163.1728;

    pub const NTERM_H: f32 = 1.00797;
    pub const CTERM_OH: f32 = 17.00738;

    pub fn convert(c: char) -> f32 {
        match c {
            'A' => Weight::A,
            'C' => Weight::C,
            'D' => Weight::D,
            'E' => Weight::E,
            'F' => Weight::F,
            'G' => Weight::G,
            'H' => Weight::H,
            'I' => Weight::I,
            'K' => Weight::K,
            'L' => Weight::L,
            'M' => Weight::M,
            'N' => Weight::N,
            'P' => Weight::P,
            'Q' => Weight::Q,
            'R' => Weight::R,
            'S' => Weight::S,
            'T' => Weight::T,
            'V' => Weight::V,
            'W' => Weight::W,
            'Y' => Weight::Y,
            _ => unreachable!(),
        }
    }
}
