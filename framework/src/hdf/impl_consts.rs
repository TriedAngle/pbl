use crate::consts::{calc_gravy, calc_weight, HydroScale};
use crate::hdf::DataEntry;

impl<'a> DataEntry<'a> {
    pub fn gravy(&self, scale: HydroScale) -> f32 {
        let sequence = self.sequence.as_str();
        let len = sequence.len();
        let sequence = &self.sequence.as_str()[1..len - 1];
        calc_gravy(sequence, scale)
    }

    pub fn weight(&self) -> f32 {
        let sequence = self.sequence.as_str();
        let len = sequence.len();
        let sequence = &self.sequence.as_str()[1..len - 1];
        calc_weight(sequence)
    }
}
