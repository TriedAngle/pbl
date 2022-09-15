use crate::consts::hydro::{Cornette, HoppWoods, HydroScale};
use crate::consts::AMINO_ACIDS;
use crate::consts::{KyteDoolittle, Weight};

pub fn calc_gravy(sequence: &str, scale: HydroScale) -> f32 {
    let scale_fn = match scale {
        HydroScale::KyteDoolittle => KyteDoolittle::convert,
        HydroScale::HoppWoods => HoppWoods::convert,
        HydroScale::Cornette => Cornette::convert,
        _ => unimplemented!(),
    };

    sequence.chars().map(scale_fn).sum::<f32>() / sequence.len() as f32
}

pub fn calc_weight(sequence: &str) -> f32 {
    sequence.chars().map(Weight::convert).sum::<f32>() + Weight::NTERM_H + Weight::CTERM_OH
}

pub fn is_unimplemented_sequence(sequence: &str) -> bool {
    for c in sequence.chars() {
        if !AMINO_ACIDS.contains(&c) {
            return true;
        }
    }
    return false;
}
