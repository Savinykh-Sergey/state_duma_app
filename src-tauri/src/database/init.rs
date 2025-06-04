use std::env;
use once_cell::sync::OnceCell;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub static STATE_DUMA_POOL: OnceCell<PgPool> = OnceCell::new();
pub async fn init_pools() {
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL должна быть указана в .env файле");

    let state_duma_pool = PgPoolOptions::new()
        .max_connections(32)
        .connect(&database_url)
        .await
        .expect("Не удалось подключиться к базе данных STATE_DUMA");

    STATE_DUMA_POOL
        .set(state_duma_pool)
        .expect("Не удалось инициализировать STATE_DUMA_POOL");
}

pub async fn get_pool() -> &'static PgPool {
    STATE_DUMA_POOL.get().expect("STATE_DUMA_POOL не инициализирован")
}