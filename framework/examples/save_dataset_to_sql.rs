use color_eyre::Result;
use framework::consts::HydroScale;
use framework::hdf::Dataset;
use sqlx::{Connection, PgConnection};
use std::fs;
use std::io::{BufWriter, Write};

struct FileInsert {
    file: String,
}

const FILE: &str = "../assets/datasets/retention_time.sqlite";
async fn main() -> Result<()> {
    let dataset = Dataset::default();
    let len = dataset.len;
    let split_amount = 10000;
    let split = (len + split_amount - 1) / split_amount;

    let conn = PgConnection::connect("posgresql://admin:admin@localhost/admin").await?;

    // db.execute(r"
    //     CREATE TABLE files (
    //         id INTEGER PRIMARY KEY,
    //         file TEXT UNIQUE
    //     );
    // ", ())?;
    //
    // db.execute(r"
    //     CREATE TABLE sequences (
    //         id INTEGER PRIMARY KEY,
    //         sequence TEXT UNIQUE
    //     );
    // ", ())?;
    //
    // db.execute(r"
    //     CREATE TABLE records (
    //         id INTEGER PRIMARY KEY,
    //         retention_time REAL NOT NULL,
    //         score REAL NOT NULL,
    //         sequence INTEGER NOT NULL,
    //         file INTEGER NOT NULL,
    //         FOREIGN KEY (sequence) REFERENCES sequences (id),
    //         FOREIGN KEY (file) REFERENCES files (id)
    //     );
    // ", ())?;
    //
    // db.execute(r"
    //     CREATE TABLE extra (
    //         sequence INTEGER PRIMARY KEY,
    //         weight REAL,
    //         kyte_doolittle REAL,
    //         hopp_woods REAL,
    //         cornette REAL,
    //         FOREIGN KEY (sequence) REFERENCES sequences (id)
    //     );
    // ", ())?;
    //
    // let mut counter = 1;
    // for slice in dataset.iter_chunked(split, Some(split_amount), 0) {
    //     println!("{}%", counter / len);
    //     for entry in &slice {
    //         let unimplemented = entry.is_unimplemented_sequence();
    //
    //         let sequence = entry.sequence.as_str();
    //         let len = sequence.len();
    //         let sequence = &sequence[1..len - 1];
    //         let file = entry.raw_file.as_str();
    //
    //         db.execute("INSERT OR IGNORE INTO files (file) VALUES (?1);", [file])?;
    //         db.execute("INSERT OR IGNORE INTO sequences (sequence) VALUES (?1);", [sequence])?;
    //         db.execute(r"
    //             INSERT INTO records (id, retention_time, score, sequence, file) VALUES (
    //                 ?1,
    //                 ?2,
    //                 ?3,
    //                 (SELECT s.ID FROM sequences s WHERE s.sequence = ?4 LIMIT 1),
    //                 (SELECT f.ID FROM files f WHERE f.file = ?5 LIMIT 1)
    //             );", (counter, entry.retention_time.as_f64(), entry.score.as_f64(), sequence, file))?;
    //         // files_sql.push_str(
    //         //     &format!("INSERT OR IGNORE INTO files (file) VALUES ('{}');\n", file));
    //         // sequences_sql.push_str(
    //         //     &format!(" INSERT OR IGNORE INTO sequences (sequence) VALUES ('{}');\n", sequence));
    //         // records_sql.push_str(
    //         //     &format!("\
    //         //     INSERT INTO records (retention_time, score, sequence, file) VALUES (
    //         //         {},
    //         //         {},
    //         //         (SELECT s.ID FROM sequences s WHERE s.sequence = '{}' LIMIT 1),
    //         //         (SELECT f.ID FROM files f WHERE f.file = '{}' LIMIT 1)
    //         //     );\n", entry.retention_time.as_f64(), entry.score.as_f64(), sequence, file)
    //         // );
    //         // if !unimplemented {
    //         //     let weight = entry.weight();
    //         //     let gravy1 = entry.gravy(HydroScale::KyteDoolittle);
    //         //     let gravy2 = entry.gravy(HydroScale::HoppWoods);
    //         //     let gravy3 = entry.gravy(HydroScale::Cornette);
    //         //     extra_sql.push_str(
    //         //         &format!("\
    //         //         INSERT OR IGNORE INTO extra (sequence, weight, kyte_doolittle, hopp_woods, cornette) VALUES (
    //         //             (SELECT s.ID FROM sequences s WHERE s.sequence = '{}' LIMIT 1),
    //         //             {},
    //         //             {},
    //         //             {},
    //         //             {}
    //         //         );\n", sequence, weight, gravy1, gravy2, gravy3)
    //         //     );
    //         // }
    //         counter += 1;
    //     }
    // }
    Ok(())
}
