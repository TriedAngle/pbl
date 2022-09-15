//! Values taken from:
//! https://resources.qiagenbioinformatics.com/manuals/clcgenomicsworkbench/650/Hydrophobicity_scales.html

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum HydroScale {
    KyteDoolittle,
    HoppWoods,
    Cornette,
    Eisenberg,
    Rose,
    Janin,
    EngelmanGES,
}

impl Default for HydroScale {
    fn default() -> Self {
        HydroScale::KyteDoolittle
    }
}

/// positive = hydrophobic
#[non_exhaustive]
pub struct KyteDoolittle;

#[non_exhaustive]
pub struct HoppWoods;

#[non_exhaustive]
pub struct Cornette;

impl KyteDoolittle {
    pub const A: f32 = 1.80;
    pub const C: f32 = 2.50;
    pub const D: f32 = -3.50;
    pub const E: f32 = -3.50;
    pub const F: f32 = 2.80;
    pub const G: f32 = -0.40;
    pub const H: f32 = -3.20;
    pub const I: f32 = 4.50;
    pub const K: f32 = -3.90;
    pub const L: f32 = 3.80;
    pub const M: f32 = 1.90;
    pub const N: f32 = -3.50;
    pub const P: f32 = -1.60;
    pub const Q: f32 = -3.50;
    pub const R: f32 = -4.50;
    pub const S: f32 = -0.80;
    pub const T: f32 = -0.70;
    pub const V: f32 = 4.20;
    pub const W: f32 = -0.90;
    pub const Y: f32 = -1.30;

    pub fn convert(c: char) -> f32 {
        use KyteDoolittle as Scale;
        match c {
            'A' => Scale::A,
            'C' => Scale::C,
            'D' => Scale::D,
            'E' => Scale::E,
            'F' => Scale::F,
            'G' => Scale::G,
            'H' => Scale::H,
            'I' => Scale::I,
            'K' => Scale::K,
            'L' => Scale::L,
            'M' => Scale::M,
            'N' => Scale::N,
            'P' => Scale::P,
            'Q' => Scale::Q,
            'R' => Scale::R,
            'S' => Scale::S,
            'T' => Scale::T,
            'V' => Scale::V,
            'W' => Scale::W,
            'Y' => Scale::Y,
            _ => unreachable!(),
        }
    }
}

impl HoppWoods {
    pub const A: f32 = -0.50;
    pub const C: f32 = -1.00;
    pub const D: f32 = 3.00;
    pub const E: f32 = 3.00;
    pub const F: f32 = -2.50;
    pub const G: f32 = 0.00;
    pub const H: f32 = -0.50;
    pub const I: f32 = -1.80;
    pub const K: f32 = 3.00;
    pub const L: f32 = -1.80;
    pub const M: f32 = -1.30;
    pub const N: f32 = 0.20;
    pub const P: f32 = 0.00;
    pub const Q: f32 = 0.20;
    pub const R: f32 = 3.00;
    pub const S: f32 = 0.30;
    pub const T: f32 = -0.40;
    pub const V: f32 = -1.50;
    pub const W: f32 = -3.40;
    pub const Y: f32 = -2.30;

    pub fn convert(c: char) -> f32 {
        use HoppWoods as Scale;
        match c {
            'A' => Scale::A,
            'C' => Scale::C,
            'D' => Scale::D,
            'E' => Scale::E,
            'F' => Scale::F,
            'G' => Scale::G,
            'H' => Scale::H,
            'I' => Scale::I,
            'K' => Scale::K,
            'L' => Scale::L,
            'M' => Scale::M,
            'N' => Scale::N,
            'P' => Scale::P,
            'Q' => Scale::Q,
            'R' => Scale::R,
            'S' => Scale::S,
            'T' => Scale::T,
            'V' => Scale::V,
            'W' => Scale::W,
            'Y' => Scale::Y,
            _ => unreachable!(),
        }
    }
}

impl Cornette {
    pub const A: f32 = 0.20;
    pub const C: f32 = 4.10;
    pub const D: f32 = -3.10;
    pub const E: f32 = -1.80;
    pub const F: f32 = 4.40;
    pub const G: f32 = 0.00;
    pub const H: f32 = 0.50;
    pub const I: f32 = 4.80;
    pub const K: f32 = -3.10;
    pub const L: f32 = 5.70;
    pub const M: f32 = 4.20;
    pub const N: f32 = -0.50;
    pub const P: f32 = -2.20;
    pub const Q: f32 = -2.80;
    pub const R: f32 = 1.40;
    pub const S: f32 = -0.50;
    pub const T: f32 = -1.90;
    pub const V: f32 = 4.70;
    pub const W: f32 = 1.00;
    pub const Y: f32 = 3.20;

    pub fn convert(c: char) -> f32 {
        use Cornette as Scale;
        match c {
            'A' => Scale::A,
            'C' => Scale::C,
            'D' => Scale::D,
            'E' => Scale::E,
            'F' => Scale::F,
            'G' => Scale::G,
            'H' => Scale::H,
            'I' => Scale::I,
            'K' => Scale::K,
            'L' => Scale::L,
            'M' => Scale::M,
            'N' => Scale::N,
            'P' => Scale::P,
            'Q' => Scale::Q,
            'R' => Scale::R,
            'S' => Scale::S,
            'T' => Scale::T,
            'V' => Scale::V,
            'W' => Scale::W,
            'Y' => Scale::Y,
            _ => unreachable!(),
        }
    }
}
