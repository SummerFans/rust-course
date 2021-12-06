use async_trait::async_trait;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{error::Error, thread::sleep, time::Duration};
use thiserror::Error;


#[derive(Error, Debug)]
#[non_exhaustive]
pub enum LinkError {
    #[error("database error")]
    DatabaseError(#[from] sqlx::Error),
}

async fn connect_db() -> Result<Pool<Postgres>, LinkError> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:X5ax5a62ang@summers.myds.me:31000/erp")
        .await?;
    Ok(pool)
}

struct Repository {
    db: Pool<Postgres>,
}

impl Repository {
    fn new(db: Pool<Postgres>) -> Repository {
        Repository { db }
    }
}

#[async_trait]
pub trait RepositoryOperate {
    async fn get_info(&self) -> Result<i64, sqlx::Error>;

}

#[async_trait]
impl RepositoryOperate for Repository {
    async fn get_info(&self) -> Result<i64, sqlx::Error> {
        let row: (i64,) = sqlx::query_as("SELECT * from table;")
            .bind(150_i64)
            .fetch_one(&self.db)
            .await?;

        println!("end");
        Ok(row.0)
    }
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = match connect_db().await {
        Ok(v) => v,
        Err(err) => {
            panic!("error connected: {}", err)
        }
    };

    let repository = Repository::new(pool.clone());

    let data = repository.get_info().await?;
    
    println!("{}", data);
    sleep(Duration::from_millis(10000));
    Ok(())
}
