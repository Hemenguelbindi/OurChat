use crate::database::postgres::PostgresDbCfg;
use crate::database::redis::RedisCfg;
use sea_orm::DatabaseConnection;

pub mod postgres;
pub mod redis;

/// The database connection pool, redis connection pool
/// you can clone it freely without many extra cost
#[derive(Debug, Clone)]
pub struct DbPool {
    pub db_pool: DatabaseConnection,
    pub redis_pool: deadpool_redis::Pool,
}

impl DbPool {
    pub async fn close(&mut self) -> anyhow::Result<()> {
        self.db_pool.clone().close().await?;
        self.redis_pool.close();
        Ok(())
    }

    pub async fn build(postgres: &PostgresDbCfg, redis: &RedisCfg) -> anyhow::Result<Self> {
        let db_pool = postgres::connect_to_db(&postgres.url()).await?;
        let redis_pool = redis::connect_to_redis(&redis.get_redis_url()?).await?;
        Ok(Self {
            db_pool,
            redis_pool,
        })
    }

    pub async fn init(&self) -> anyhow::Result<()> {
        postgres::init_postgres(&self.db_pool).await?;
        Ok(())
    }
}
