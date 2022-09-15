use color_eyre::Result;
use framework::consts::HydroScale;
use framework::hdf::Dataset;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> Result<()> {
    let dataset = Dataset::default();
    let mut counter = 0;
    let len = dataset.len;
    let split_amount = 10000;
    let split = (len + split_amount - 1) / split_amount;

    if File::open("../assets/datasets/retention_time.csv").is_ok() {
        fs::remove_file("../assets/datasets/retention_time.csv")?;
    }

    let mut file = File::options()
        .create_new(true)
        .append(true)
        .open("../assets/datasets/retention_time.csv")?;
    let mut file = BufWriter::new(file);
    file.write("\"id\",\"retention_time\",\"score\",\"sequence\",\"raw_file\",\"weight\",\"GRAVY[KyteDoolittle]\",\"GRAVY[HoppWoods]\",\"GRAVY[Cornette]\",\"unimplemented\"\n".as_bytes())?;
    file.flush()?;
    for slice in dataset.iter_chunked(split, Some(split_amount), 0) {
        // for slice in dataset.iter_chunked(1, Some(5), 0) {
        for entry in &slice {
            let unimplemented = entry.is_unimplemented_sequence();

            let (weight, gravy_kyte_doolittle, gravy_hopp_woods, gravy_cornette) = if unimplemented
            {
                (f32::NAN, f32::NAN, f32::NAN, f32::NAN)
            } else {
                (
                    entry.weight(),
                    entry.gravy(HydroScale::KyteDoolittle),
                    entry.gravy(HydroScale::HoppWoods),
                    entry.gravy(HydroScale::Cornette),
                )
            };

            let sequence = entry.sequence.as_str();
            let sequence = &sequence[1..sequence.len()];

            let line = format!(
                "{},{},{},\"{}\",\"{}\",{},{},{},{},{}\n",
                counter,
                entry.retention_time.as_f64(),
                entry.score.as_f64(),
                sequence,
                entry.raw_file.as_str(),
                weight,
                gravy_kyte_doolittle,
                gravy_hopp_woods,
                gravy_cornette,
                unimplemented,
            );
            file.write(line.as_bytes())?;
            counter += 1;
        }
        file.flush()?;
    }
    Ok(())
}
