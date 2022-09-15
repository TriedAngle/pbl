mod calc;
mod hydro;
mod weight;

pub use calc::calc_gravy;
pub use calc::calc_weight;
pub use calc::is_unimplemented_sequence;
pub use hydro::HydroScale;
pub use hydro::KyteDoolittle;
pub use weight::Weight;

pub const AMINO_ACIDS: &[char] = &[
    'A', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W',
    'Y',
];
