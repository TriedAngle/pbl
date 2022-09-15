use crate::consts::AMINO_ACIDS;
use crate::hdf::{RawFile, RetentionTime, Score, Sequence};
use color_eyre::Result;
use ndarray::{s, Array, Ix1};
use std::ops::Range;
use std::path::Path;

#[derive(Debug)]
pub struct Dataset {
    pub file: hdf5::File,
    pub retention_time: hdf5::Dataset,
    pub score: hdf5::Dataset,
    pub sequence: hdf5::Dataset,
    pub raw_file: hdf5::Dataset,
    pub len: usize,
}

#[derive(Debug)]
pub struct DataSlice {
    pub retention_time: Array<RetentionTime, Ix1>,
    pub score: Array<Score, Ix1>,
    pub sequence: Array<Sequence, Ix1>,
    pub raw_file: Array<RawFile, Ix1>,
    pub offset: usize,
    pub len: usize,
}

#[derive(Debug)]
pub struct DataEntry<'a> {
    pub retention_time: &'a RetentionTime,
    pub score: &'a Score,
    pub sequence: &'a Sequence,
    pub raw_file: &'a RawFile,
}

#[derive(Debug)]
pub struct DataChunkIter<'a> {
    pub dataset: &'a Dataset,
    pub chunk_size: usize,
    pub current: Range<usize>,
    pub counter: usize,
    pub times: Option<usize>,
}

#[derive(Debug)]
pub struct DataSliceIter<'a> {
    pub data_slice: &'a DataSlice,
    pub counter: usize,
    pub max: usize,
}

impl Dataset {
    pub fn new<T: AsRef<Path>>(path: T) -> Result<Self> {
        let file = hdf5::File::open(path)?;
        let retention_time = file.dataset("retention_time")?;
        let score = file.dataset("score")?;
        let sequence = file.dataset("sequence")?;
        let raw_file = file.dataset("raw_file")?;
        let len = retention_time.size();
        Ok(Self {
            file,
            retention_time,
            score,
            sequence,
            raw_file,
            len,
        })
    }

    pub fn slice(&self, range: Range<usize>) -> Result<DataSlice> {
        let len = range.len();
        let (start, stop) = (range.start, range.end);
        let retention_time = data_slice(&self.retention_time, start, stop)?;
        let score = data_slice(&self.score, start, stop)?;
        let sequence = data_slice(&self.sequence, start, stop)?;
        let raw_file = data_slice(&self.raw_file, start, stop)?;

        Ok(DataSlice {
            retention_time,
            score,
            sequence,
            raw_file,
            offset: range.start,
            len,
        })
    }

    pub fn iter_chunked(
        &self,
        size: usize,
        times: Option<usize>,
        start: usize,
    ) -> DataChunkIter<'_> {
        DataChunkIter {
            dataset: &self,
            chunk_size: size,
            current: start..start + size,
            counter: 0,
            times,
        }
    }
}

impl DataSlice {
    pub fn entry(&self, idx: usize) -> Option<DataEntry> {
        let retention_time = self.retention_time.get(idx)?;
        let score = self.score.get(idx)?;
        let sequence = self.sequence.get(idx)?;
        let raw_file = self.raw_file.get(idx)?;

        Some(DataEntry {
            retention_time,
            score,
            sequence,
            raw_file,
        })
    }
}

impl<'a> DataEntry<'a> {
    pub fn is_unimplemented_sequence(&self) -> bool {
        let sequence = self.sequence.as_str();
        let len = sequence.len();
        let sequence = &self.sequence.as_str()[1..len - 1];
        for c in sequence.chars() {
            if !AMINO_ACIDS.contains(&c) {
                return true;
            }
        }
        return false;
    }

    pub fn is_valid_format(&self) -> bool {
        let sequence = self.sequence.as_str();
        let len = sequence.len();
        if sequence.chars().nth(0).unwrap() != '_' || sequence.chars().nth(len - 1).unwrap() != '_'
        {
            return false;
        }
        let sequence = &self.sequence.as_str()[1..len - 1];
        return !sequence.contains('_');
    }
}

impl<'a> Iterator for DataChunkIter<'a> {
    type Item = DataSlice;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.start > self.dataset.len {
            return None;
        }

        if let Some(times) = self.times {
            if self.counter >= times {
                return None;
            }
            self.counter += 1;
        }

        if self.current.end > self.dataset.len {
            self.current.end -= self.current.end - self.dataset.len;
        }

        let slice = self.dataset.slice(self.current.clone()).unwrap();
        self.current.start += self.chunk_size;
        self.current.end += self.chunk_size;
        Some(slice)
    }
}

impl<'a> Iterator for DataSliceIter<'a> {
    type Item = DataEntry<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter >= self.max {
            return None;
        }

        let retention_time = self.data_slice.retention_time.get(self.counter).unwrap();
        let score = self.data_slice.score.get(self.counter).unwrap();
        let sequence = self.data_slice.sequence.get(self.counter).unwrap();
        let raw_file = self.data_slice.raw_file.get(self.counter).unwrap();

        self.counter += 1;
        Some(DataEntry {
            retention_time,
            score,
            sequence,
            raw_file,
        })
    }
}

impl<'a> IntoIterator for &'a DataSlice {
    type Item = DataEntry<'a>;
    type IntoIter = DataSliceIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            data_slice: self,
            counter: 0,
            max: self.len,
        }
    }
}

impl Default for Dataset {
    fn default() -> Self {
        Self::new("../assets/datasets/retention_time.hdf5").unwrap()
    }
}

fn data_slice<T: hdf5::H5Type>(
    dataset: &hdf5::Dataset,
    start: usize,
    stop: usize,
) -> Result<Array<T, Ix1>> {
    let slice = dataset.read_slice_1d::<T, _>(s![start..stop])?;
    Ok(slice)
}
