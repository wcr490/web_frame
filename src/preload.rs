use super::*;

#[macro_export()]
macro_rules! sql_pool_generator {
    () => {
        use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
        use sqlx::Pool;
        pub static DB: OnceCell<MySqlPool> = OnceCell::new();
        pub fn db_pool<'a>() -> &'a MySqlPool {
            DB.get().expect("fail to find pool")
        }
        pub async fn set_db_pool(url: &str) -> &MySqlPool {
            let p = Pool::connect(url).await.expect("fail to connect");
            DB.get_or_init(|| p)
        }
    };
}
