use super::*;
use crate::{midware_generator, midware_method_generator};
use crate::{query_collect, query_submit};

midware_generator!(Sql);
midware_method_generator!(Sql, Priority::P3, |req| {
    tokio::runtime::Runtime::new()
        .expect("sql: fail to build a new rt")
        .block_on(sql_init());
    println!("Sql");
    req
});

async fn sql_init() {}

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

#[macro_export()]
macro_rules! query_collect {
    () => {};
    ($($element: expr),+) => {
        struct $table {
            ($element):String;+
        }
    };
}
#[macro_export()]
macro_rules! query_submit {
    () => {};
    ($command: expr, $pool: expr) => {{
        let rows = sqlx::query($command)
            .fetch_all($pool)
            .await
            .expect("fail to submit query");
        for row in &rows {
            for col in row.columns() {
                let col_name = col.name();
                let col_val = row.get::<_, String>(col_name);
            }
        }
    }};
}
