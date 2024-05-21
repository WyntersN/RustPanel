/*
 * @Descripttion:
 * @version:
 * @Author: Wynters
 * @Date: 2024-05-08 18:11:45
 * @LastEditTime: 2024-05-20 18:57:58
 * @FilePath: \RustPanel\src\server\db.rs
 */
use std::fs;
use diesel::{connection::SimpleConnection, r2d2::ConnectionManager, ExpressionMethods, RunQueryDsl, SqliteConnection};
use rand::Rng;
use chrono::{NaiveDateTime, Utc};

use crate::common::fun::sha1_salt;





pub type DBPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn install(pool: &DBPool){


    if let Ok(metadata) = fs::metadata("./panel.lock") {
        if metadata.is_file() {
           return;
        } 
    } 
    let mut conn = pool.get().expect("Failed to get a connection from the pool");

  


    conn.batch_execute(
        "CREATE TABLE if not exists rp_users (
            id INTEGER PRIMARY KEY,
            username TEXT(16) UNIQUE NOT NULL,
            password TEXT(40) NOT NULL,
            salt TEXT(16) NOT NULL,
            initial_password TEXT(18) NOT NULL,
            authority TEXT(20) NOT NULL,
            error_count INTEGER(5) DEFAULT 0,
            status INTEGER(5) DEFAULT 0 NOT NULL,
            api_key TEXT(40)  NULL,
            created_at datetime NOT NULL,
            updated_at datetime NOT NULL,
            deleted_at datetime
         )"
    ).unwrap();
    use crate::{common::fun::generate_random_string, models::structure::schema};
    // 插入一条示例数据
    let now: NaiveDateTime = Utc::now().naive_utc().clone();

    let mut rng = rand::thread_rng();

    let password = generate_random_string(rng.gen_range(8..18));
    let salt = generate_random_string(rng.gen_range(8..16));
    let password_clone = password.clone();
    let salt_clone = salt.clone();

 
 diesel::insert_into(schema::rp_users::table)
 .values((
    schema::rp_users::username.eq(generate_random_string(rng.gen_range(6..12))),
    schema::rp_users::password.eq(sha1_salt(password,salt)),
    schema::rp_users::salt.eq(salt_clone),
    schema::rp_users::initial_password.eq(password_clone),
    schema::rp_users::authority.eq("admin"),
    schema::rp_users::error_count.eq(0),
    schema::rp_users::status.eq(0),
    schema::rp_users::created_at.eq(now),
    schema::rp_users::updated_at.eq(now),
 )).execute(&mut *conn).unwrap();

    fs::File::create("./panel.lock").unwrap();
}



pub fn init(){
    
  
   


}