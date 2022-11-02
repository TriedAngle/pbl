use sqlx::{PgPool, Row};
use tokio::time::Instant;
use framework::Result;

#[derive(sqlx::FromRow, serde::Serialize)]
struct Sequence {
    id: i32,
    sequence: String
}

#[tokio::main]
async fn main() -> Result<()> {
    let instance = Instant::now();

    let pool = PgPool::connect("postgresql://admin:admin@localhost/pbl").await?;

    let max: i32 = sqlx::query("SELECT COUNT(*) as count FROM sequences").fetch_one(&pool).await?.get::<i64, &str>("count") as i32;
    let limit = 10000;
    let mut offset = 0;
    loop {
        let res: Vec<Sequence> = sqlx::query_as(r#"
            SELECT s.id, s.sequence
            FROM sequences s
            OFFSET $1 LIMIT $2
        "#)
            .bind(offset)
            .bind(limit)
            .fetch_all(&pool)
            .await?;

        offset += res.len() as i32;
        println!("done: [{:?}] {}%", instance.elapsed(), (offset as f32) / (max as f32) * 100.0);
        if res.is_empty() { break; }
    }

    println!("Hello, world!");
    Ok(())
}

async fn redo_extras_sql(pool: &PgPool) -> Result<()> {
    sqlx::query(r#"DROP TABLE IF EXISTS extras;"#).execute(pool).await?;
    sqlx::query(
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
        .execute(pool)
        .await?;

    Ok(())
}