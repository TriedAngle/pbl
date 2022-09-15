mod data;
mod impl_consts;
pub use data::*;

#[derive(Debug, hdf5::H5Type, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct RawFile(hdf5::types::FixedAscii<120>);

#[derive(Debug, hdf5::H5Type, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct RetentionTime(f64);

#[derive(Debug, hdf5::H5Type, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct Score(f64);

#[derive(Debug, hdf5::H5Type, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct Sequence(hdf5::types::FixedAscii<120>);

impl RetentionTime {
    #[inline]
    pub fn as_f64(&self) -> f64 {
        self.0
    }
}

impl Score {
    #[inline]
    pub fn as_f64(&self) -> f64 {
        self.0
    }
}

impl RawFile {
    pub fn null() -> Self {
        RawFile(hdf5::types::FixedAscii::from_ascii("".as_bytes()).unwrap())
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Sequence {
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
