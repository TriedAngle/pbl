use framework::polars::prelude::*;
use framework::Result;

fn main() -> Result<()> {
    let mut df = LazyCsvReader::new("../../assets/datasets/retention_time.csv")
        .finish()?
        .select([col("sequence"), col("score"), col("unimplemented")])
        .filter(col("unimplemented").eq(lit(false)))
        .filter(col("score").gt(lit(100)))
        .select([col("sequence"), col("score")])
        .collect()?
        .head(Some(10));

    println!("{:?}", df);
    Ok(())
}
