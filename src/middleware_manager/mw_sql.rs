// use super::*;
use crate::{midware_generator, midware_method_generator, query_submit, DB};

use crate::query_collect;

midware_generator!(Sql);
midware_method_generator!(Sql, Priority::P3, |req| {
    tokio::spawn(sql_init());
    println!("Sql");
    req
});

async fn sql_init() {
    println!("in the middleware_sql: {:#?}", DB);
    query_submit!("INSERT INTO users (id, name, age) VALUES (1, 'John', 25)");
    query_submit!("SELECT id, name, age FROM users");
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

    (@process_fields $name: ident, $($element: ident),+) => {
        #[derive(Default, Debug, sqlx::FromRow)]
        struct $name {
            $($element: String),+
        }
    };
}
#[macro_export()]
macro_rules! query_submit {
    ($command: expr) => {{
        let rows = sqlx::query($command)
            .fetch_all($crate::DB.get().expect("fail to get DB pool"))
            .await
            .expect("fail to submit query");
        println!("{:#?}", rows);

        if !rows.is_empty() {
            let column_names: Vec<_> = rows[0].columns().iter().map(|col| col.name()).collect();

            let struct_fields: Vec<(&str, Value)> = column_names
                .iter()
                .map(|name| (*name, Value::Null))
                .collect();
            let mut struct_map: HashMap<&str, Value> = HashMap::new();
            for (name, value) in struct_fields {
                struct_map.insert(name, value);
            }

            for row in &rows {
                let mut dynamic_struct: HashMap<&str, Value> = struct_map.clone();
                for col in row.columns() {
                    let col_name = col.name();
                    let col_value: Value = match row.try_get::<String, _>(&col.name()) {
                        Ok(v) => json!(v),
                        Err(_) => Value::Null,
                    };
                    dynamic_struct.insert(col_name, col_value);
                }
                println!("{:#?}", dynamic_struct);
            }
        }

        println!("success");
    }};
}
// #[macro_export()]
// macro_rules! query_submit {
//     () => {};
//     ($command: expr) => {
//         {
//             let rows = sqlx::query($command)
//                 .fetch_all($crate::DB.get().expect("fail to get DB pool"))
//                 .await
//                 .expect("fail to submit query");
//             println!("{:#?}", rows);
//             let mut name_vec: Vec<String> = Vec::new();
//             if rows.len() > 0 {
//                 for col in rows[0].columns() {
//                     let col_name = col.name();
//                     name_vec.push(col_name.to_string());
//                 }
//                 let name = $command.replace(" ", "");
//                 query_collect!(@process_fields name, name_vec);
//                 let ret: Vec<name> = sqlx::query_as($command).fetch_all($crate::DB.get().expect("fail to get DB pool")).await.expect("fail to submit query");
//                 println!("succeed creating: {:#?}", ret);
//             }
//             println!("success");
//         }
//     };
// }
