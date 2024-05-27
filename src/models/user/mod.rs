/*
 * @Descripttion:
 * @version:
 * @Author: Wynters
 * @Date: 2024-05-08 21:50:55
 * @LastEditTime: 2024-05-27 16:24:23
 * @FilePath: \RustPanel\src\models\users\mod.rs
 */

use std::io;

use super::structure::{
    rp_users::Users,
    schema::rp_users::{self,dsl::{error_count, rp_users as ds_rp_users}},
};
use crate::{errors::common::CommonError, service::db::DBPool};
use diesel::{prelude::*, r2d2::ConnectionManager, ExpressionMethods, QueryDsl};
use r2d2::PooledConnection;

pub fn find_user_by_id(user_id: i32, pool: &DBPool) -> Result<Option<Users>, CommonError> {
    let mut conn = pool
        .get()
        .expect("Failed to get a connection from the pool");

    // 使用Diesel查询特定ID的用户
    let user_data = rp_users::table
        .filter(rp_users::id.eq(user_id))
        .first::<Users>(&mut *conn);
    match user_data {
        Ok(data) => {
            // 查询成功，返回用户
            return Ok(Some(data));
        }
        Err(err) => {
            // 查询失败，处理 Diesel 错误
            match err {
                diesel::result::Error::NotFound => {
                    // 查询失败，用户不存在
                    Ok(None)
                }
                _ => {
                    // 查询失败，其他错误
                    Err(CommonError::Error(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Database error: {:?}", err),
                    )))
                }
            }
        }
    }
}

pub fn find_user_update_error_count(
    id: i32,
    count: i32,
    conn:&mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<Option<bool>, CommonError> {


    match diesel::update(ds_rp_users.find(id))
        .set(error_count.eq(count))
        .execute(&mut *conn)
    {
        Ok(_) => {
            // 查询成功，返回用户
            return Ok(Some(true));
        }
        Err(err) => {
            // 查询失败，处理 Diesel 错误
            match err {
                diesel::result::Error::NotFound => {
                    // 查询失败，用户不存在
                    Ok(None)
                }
                _ => {
                    // 查询失败，其他错误
                    Err(CommonError::Error(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Database error: {:?}", err),
                    )))
                }
            }
        }
    }
}
