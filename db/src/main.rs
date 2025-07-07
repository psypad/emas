use std::error::Error;
use::sqlx::Row;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://postgres:admin@localhost:5432/serv1";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let res = sqlx::query("select 1 + 1 as sum")
        .fetch_one(&pool)
        .await?; 

    let sum: i32 = res.get("sum");

    println!("1 + 1 = {}", sum);

    Ok(())
}
