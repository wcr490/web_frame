use super::*;
use crate::{midware_generator, midware_method_generator, query_submit, DB};

midware_generator!(Sql);
midware_method_generator!(Sql, Priority::P3, |req| {
    tokio::spawn(sql_init());
    println!("Sql");
    req
});

async fn sql_init() {
    println!("in the middleware_sql: {:#?}", DB);
    query_submit!("INSERT INTO users (id, name, age) VALUES (1, 'John', 25)");
}

pub fn db_pool<'a>() -> &'a MySqlPool {
    DB.get().expect("fail to find pool")
}
pub async fn set_db_pool(url: &str) -> &MySqlPool {
    let p = Pool::connect(url).await.expect("fail to connect");
    DB.get_or_init(|| p)
}

#[macro_export()]
macro_rules! query_collect {
    () => {};
    /* name can be command.replace(" ", "")  */
    (@process_fields $name: expr, $($element: expr),+) => {
        struct $name {
            $($element: String),+
        }
    };
}
#[macro_export()]
macro_rules! query_submit {
    () => {};
    ($command: expr) => {{
        let rows = sqlx::query($command)
            .fetch_all($crate::DB.get().expect("fail to get DB pool"))
            .await
            .expect("fail to submit query");
        // let mut name_vec: Vec<String> = Vec::new();
        // for col in rows[0].columns() {
        //     let col_name = col.name();
        //     name_vec.push(col_name.to_string());
        // }
        println!("success");
    }};
}
