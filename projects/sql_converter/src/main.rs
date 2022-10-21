use color_eyre::Result;
use framework::consts;
use framework::consts::HydroScale;
use framework::hdf::{Dataset, RawFile, RetentionTime, Score, Sequence};
use sqlx::{Connection, Executor, FromRow, PgConnection, PgPool};
use tokio::time::Instant;

#[derive(sqlx::FromRow)]
struct SeqInsert {
    id: i32,
    sequence: String,
}

#[derive(Debug, FromRow)]
struct Record {
    id: i32,
    retention_time: f32,
    score: f32,
    sequence: i32,
    file: i32
}

const FILE: &str = "../../assets/datasets/retention_time.hdf5";
const FIRST: bool = false;
#[tokio::main]
async fn main() -> Result<()> {
    let instance = Instant::now();
    let dataset = Dataset::new(FILE)?;
    let len = dataset.len;
    let split_amount = 10000;
    let split = (len + split_amount - 1) / split_amount;

    let mut pool = PgPool::connect("postgresql://admin:admin@localhost/pbl").await?;

    if !FIRST {
        sqlx::query(r#"DROP TABLE IF EXISTS extras;"#).execute(&pool).await?;
        sqlx::query(r#"DROP TABLE IF EXISTS records;"#).execute(&pool).await?;
        sqlx::query(r#"DROP TABLE IF EXISTS sequences;"#)
            .execute(&pool)
            .await?;
        sqlx::query(r#"DROP TABLE IF EXISTS files;"#).execute(&pool).await?;
    }
    let _ = sqlx::query(
        r#"
        CREATE TABLE files (
            id SERIAL PRIMARY KEY,
            file TEXT UNIQUE
        );"#,
    )
    .execute(&pool)
    .await;
    let _ = sqlx::query(
        r#"
        CREATE TABLE sequences (
            id SERIAL PRIMARY KEY,
            sequence TEXT UNIQUE
        );"#,
    )
    .execute(&pool)
    .await;
    let _ = sqlx::query(
        r#"
        CREATE TABLE records (
            id SERIAL PRIMARY KEY,
            retention_time REAL NOT NULL,
            score REAL NOT NULL,
            sequence INTEGER NOT NULL,
            file INTEGER NOT NULL,
            FOREIGN KEY (sequence) REFERENCES sequences (id),
            FOREIGN KEY (file) REFERENCES files (id)
        );"#,
    )
    .execute(&pool)
    .await;
    let _ = sqlx::query(
        r#"
        CREATE TABLE extras (
            sequence INTEGER PRIMARY KEY,
            weight REAL,
            kyte_doolittle REAL,
            hopp_woods REAL,
            cornette REAL,
            FOREIGN KEY (sequence) REFERENCES sequences (id)
        );"#,
    )
    .execute(&pool)
    .await;

    let mut counter = 1;
    for slice in dataset.iter_chunked(split, Some(split_amount), 0) {
        println!(
            "{}% taking: {:?}",
            (counter as f32 / len as f32) * 100.0,
            instance.elapsed()
        );

        let retention_times_raw = slice
            .retention_time
            .as_slice()
            .unwrap()
            .iter()
            .map(RetentionTime::as_f64)
            .collect::<Vec<_>>();
        let scores_raw = slice
            .score
            .as_slice()
            .unwrap()
            .iter()
            .map(Score::as_f64)
            .collect::<Vec<_>>();
        let files_raw = slice
            .raw_file
            .as_slice()
            .unwrap()
            .iter()
            .map(RawFile::as_str)
            .collect::<Vec<_>>();
        let sequences_raw = slice
            .sequence
            .as_slice()
            .unwrap()
            .iter()
            .map(|s| {
                let seq = Sequence::as_str(s);
                &seq[1..seq.len() - 1]
            })
            .collect::<Vec<_>>();

        sqlx::query(
            r#"
            WITH inserts_query (file) AS (
              SELECT inserts.file
              FROM UNNEST($1) AS inserts (file)
            )
            INSERT INTO files (file)
            SELECT DISTINCT iq.file
            FROM inserts_query iq
            WHERE NOT EXISTS (
              SELECT 1
              FROM files
              WHERE file = iq.file
            )
        "#,
        )
        .bind(&files_raw)
        .execute(&pool)
        .await?;

        let sequences: Vec<SeqInsert> = sqlx::query_as(
            r#"
            WITH inserts_query (sequence) AS (
              SELECT inserts.sequence
              FROM UNNEST($1) AS inserts (sequence)
            )
            INSERT INTO sequences (sequence)
            SELECT DISTINCT iq.sequence
            FROM inserts_query iq
            WHERE NOT EXISTS (
              SELECT 1
              FROM sequences
              WHERE sequence = iq.sequence
            )
            RETURNING id, sequence
        "#,
        )
        .bind(&sequences_raw)
        .fetch_all(&pool)
        .await?;

        let mut ids = Vec::new();
        let mut weight = Vec::new();
        let mut gravy1 = Vec::new();
        let mut gravy2 = Vec::new();
        let mut gravy3 = Vec::new();
        for sequence in sequences {
            if consts::is_unimplemented_sequence(&sequence.sequence) {
                continue;
            }
            ids.push(sequence.id);
            weight.push(consts::calc_weight(&sequence.sequence));
            gravy1.push(consts::calc_gravy(
                &sequence.sequence,
                HydroScale::KyteDoolittle,
            ));
            gravy2.push(consts::calc_gravy(
                &sequence.sequence,
                HydroScale::HoppWoods,
            ));
            gravy3.push(consts::calc_gravy(&sequence.sequence, HydroScale::Cornette));
        }

        sqlx::query(r#"
            INSERT INTO extras (sequence, weight, kyte_doolittle, hopp_woods, cornette)
            SELECT i.sequence, i.weight, i.kyte_doolittle, i.hopp_woods, i.cornette
            FROM UNNEST($1, $2, $3, $4, $5) AS i (sequence, weight, kyte_doolittle, hopp_woods, cornette)
        "#)
            .bind(&ids)
            .bind(&weight)
            .bind(&gravy1)
            .bind(&gravy2)
            .bind(&gravy3)
            .execute(&pool).await?;

        sqlx::query(
            r#"
            WITH raw_inserts (retention_time, score, sequence, file) AS (
                SELECT i.retention_time, i.score, i.sequence, i.file
                FROM UNNEST($1, $2, $3, $4) AS i (retention_time, score, sequence, file)
            )
            INSERT INTO records (retention_time, score, sequence, file)
            SELECT i.retention_time, i.score, i.sequence_id, i.file_id
            FROM (
                SELECT raw.retention_time, raw.score, s.id as sequence_id, f.id as file_id
                FROM raw_inserts raw
                JOIN sequences s ON raw.sequence = s.sequence
                JOIN files f ON raw.file = f.file
            ) as i
        "#,
        )
        .bind(&retention_times_raw)
        .bind(&scores_raw)
        .bind(&sequences_raw)
        .bind(&files_raw)
        .execute(&pool)
        .await?;

        for _entry in &slice {
            counter += 1;
        }
    }
    println!("took: {:?}", instance.elapsed());
    Ok(())
}
