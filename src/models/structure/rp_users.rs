/*
 * @Descripttion: 
 * @version: 
 * @Author: Wynters
 * @Date: 2024-05-08 21:47:26
 * @LastEditTime: 2024-05-13 21:30:09
 * @FilePath: \RustPanel\src\models\structure\rp_users.rs
 */

use diesel::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Queryable)]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub initial_password: String,
    pub authority: String,
    pub error_count: i32,
    pub status: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    //pub deleted_at: Option<SystemTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub authority: String,
    pub created_at: chrono::NaiveDateTime,
}

impl From<Users> for SlimUser {
    fn from(user: Users) -> Self {
        SlimUser {
             id:user.id,
             username:user.username,
             password:user.password,
             authority:user.authority,
             created_at:user.created_at,
        }
    }
}

