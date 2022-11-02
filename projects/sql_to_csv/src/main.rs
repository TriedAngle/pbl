use std::time::Instant;
use framework::Result;
use sqlx::PgPool;
use csv::{Writer, WriterBuilder};

#[derive(sqlx::FromRow, serde::Serialize)]
struct Entry1 {
    seq_id: i32,
    seq_full: String,
    seq_full_ox: String,
    retention_time: f32,
    score: f32,
    is_ox: bool,
    mass: f32,
    hs_kd: f32,
    hs_hw: f32,
    hs_c: f32,
}

#[derive(sqlx::FromRow, serde::Serialize)]
struct Entry2 {
    rec_id: i32,
    seq_id: i32,
    file_id: i32,
    retention_time: f32,
    score: f32,
    is_ox: bool,
    mass: f32,
    hs_kd: f32,
    hs_hw: f32,
    hs_c: f32,
}

#[derive(sqlx::FromRow, serde::Serialize)]
struct Entry3 {
    seq_id: i32,
    file_id: i32,
    retention_time: f32,
    score: f32,
    count: i32,
    is_ox: bool,
    mass: f32,
    hs_kd: f32,
    hs_hw: f32,
    hs_c: f32,
}

enum Option {
    FULL,
    PerFile,
    Unique
}

const OPT: Option = Option::Unique;

#[tokio::main]
async fn main() -> Result<()> {
    let instance = Instant::now();
    let mut pool = PgPool::connect("postgresql://admin:admin@localhost/pbl").await?;
    let mut writer = WriterBuilder::new()
            .double_quote(true)
            .from_path(match OPT {
                Option::FULL => {"../../assets/datasets/complete.csv" }
                Option::PerFile => {"../../assets/datasets/per_file.csv"}
                Option::Unique => {"../../assets/datasets/unique.csv"}
            })?;

    let limit: i32 = 10000;
    let mut offset: i32 = 0;
    let max = match OPT {
        Option::FULL => {27372470}
        Option::PerFile => {1233527}
        Option::Unique => {531989}
    };
    let query = match OPT {
        Option::Unique =>
        r#"
            SELECT s.id                                                          as seq_id,
                   s.sequence                                                    as seq_full,
                   REPLACE(s.sequence, '(ox)', 'X')                              as seq_full_ox,
                   a.weighted2_retention                                         as retention_time,
                   a.weighted2_score                                             as score,
                   CASE WHEN e.sequence IS NULL THEN true ELSE false END         as is_ox,
                   CASE WHEN e.sequence IS NULL THEN 0 ELSE e.weight END         as mass,
                   CASE WHEN e.sequence IS NULL THEN 0 ELSE e.kyte_doolittle END as hs_kd,
                   CASE WHEN e.sequence IS NULL THEN 0 ELSE e.hopp_woods END     as hs_hw,
                   CASE WHEN e.sequence IS NULL THEN 0 ELSE e.cornette END       as hs_c
            FROM average_retention_times a
                     JOIN sequences s on a.sequence = s.id
                     LEFT JOIN extras e on a.sequence = e.sequence
            OFFSET $1 LIMIT $2
        "#,
        Option::FULL =>
        r#"
            SELECT
                   r.id                                                          as rec_id,
                   r.sequence                                                    as seq_id,
                   r.file                                                        as file_id,
                   r.retention_time                                              as retention_time,
                   r.score                                                       as score,
                   CASE WHEN e.sequence IS NULL THEN true ELSE false END         as is_ox,
                   CASE WHEN e.sequence IS NULL THEN 0 ELSE e.weight END         as mass,
                   CASE WHEN e.sequence IS NULL THEN 0 ELSE e.kyte_doolittle END as hs_kd,
                   CASE WHEN e.sequence IS NULL THEN 0 ELSE e.hopp_woods END     as hs_hw,
                   CASE WHEN e.sequence IS NULL THEN 0 ELSE e.cornette END       as hs_c
            FROM records r
                     LEFT JOIN extras e on r.sequence = e.sequence
            ORDER BY r.id
            OFFSET $1 LIMIT $2
        "#,
        Option::PerFile => r#"
            SELECT a.sequence                                                    as seq_id,
                   a.file                                                        as file_id,
                   a.weighted_retention                                          as retention_time,
                   a.weighted_score                                              as score,
                   a.count                                                       as count,
                   CASE WHEN e.sequence IS NULL THEN true ELSE false END         as is_ox,
                   CASE WHEN e.sequence IS NULL THEN 0 ELSE e.weight END         as mass,
                   CASE WHEN e.sequence IS NULL THEN 0 ELSE e.kyte_doolittle END as hs_kd,
                   CASE WHEN e.sequence IS NULL THEN 0 ELSE e.hopp_woods END     as hs_hw,
                   CASE WHEN e.sequence IS NULL THEN 0 ELSE e.cornette END       as hs_c
            FROM average_retention_times_per_file a
                     LEFT JOIN extras e on a.sequence = e.sequence
            ORDER BY a.sequence, a.file
            OFFSET $1 LIMIT $2
        "#
    };
    loop {
        let res: Vec<Entry1> = sqlx::query_as(query)
            .bind(offset)
            .bind(limit)
            .fetch_all(&pool)
            .await?;

        // let res: Vec<Entry2> = sqlx::query_as(query)
        //     .bind(offset)
        //     .bind(limit)
        //     .fetch_all(&pool)
        //     .await?;
        // let res: Vec<Entry3> = sqlx::query_as(query)
        //     .bind(offset)
        //     .bind(limit)
        //     .fetch_all(&pool)
        //     .await?;

        offset += res.len() as i32;
        for entry in &res {
            writer.serialize(entry)?;
        }
        writer.flush()?;
        println!("done: {}%", (offset as f32) / (max as f32) * 100.0);
        if res.is_empty() { break; }
    }
    println!("done, took: {:?}", instance.elapsed());
    Ok(())
}
