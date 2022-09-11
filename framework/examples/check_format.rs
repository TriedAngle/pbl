use framework::hdf::Dataset;
use color_eyre::Result;
use framework::consts::HydroScale;

fn main() -> Result<()>{
    let dataset = Dataset::default();
    let mut counter = 0;
    for slice in dataset.iter_chunked(10, Some(3), 0) {
        for entry in &slice {
            entry.gravy(HydroScale::default());
            counter += 1;
        }
    }
    assert_eq!(counter, 30);
    Ok(())
}