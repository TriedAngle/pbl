use color_eyre::Result;
use framework::consts::HydroScale;
use framework::hdf::Dataset;

fn main() -> Result<()> {
    let dataset = Dataset::default();
    let mut counter = 0;
    let len = dataset.len;
    let split_amount = 10000;
    let split = (len + split_amount - 1) / split_amount;
    for slice in dataset.iter_chunked(split, Some(split_amount), 0) {
        for entry in &slice {
            if entry.is_unimplemented_sequence() {
                println!("id: {}, sequence: {}", counter, entry.sequence.as_str())
            }
            counter += 1;
        }
    }
    Ok(())
}
