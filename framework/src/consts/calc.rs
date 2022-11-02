use crate::consts;
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

pub fn calc_extinction(sequence: &str) -> f32 {
    let mut count_w_y_c = [0.0, 0.0, 0.0];
    for c in sequence.chars() {
        match c {
            'W' => count_w_y_c[0] += 1.0,
            'Y' => count_w_y_c[1] += 1.0,
            'C' => count_w_y_c[2] += 1.0,
            _ => continue,
        }
    }

    consts::E_W * count_w_y_c[0] + consts::E_Y * count_w_y_c[1] + consts::E_C * count_w_y_c[2]
}

pub fn is_unimplemented_sequence(sequence: &str) -> bool {
    for c in sequence.chars() {
        if !AMINO_ACIDS.contains(&c) {
            return true;
        }
    }
    return false;
}
