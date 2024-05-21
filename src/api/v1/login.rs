/*
 * @Descripttion:
 * @version:
 * @Author: Wynters
 * @Date: 2024-05-10 01:54:06
 * @LastEditTime: 2024-05-20 18:53:14
 * @FilePath: \RustPanel\src\api\v1\login.rs
 */

use actix_web::{web, Error, HttpResponse};
use rust_i18n::t;
use std::{collections::HashMap, time::{Duration, SystemTime, UNIX_EPOCH}};

use crate::{
    api::{
        auth::{generate_jwt, verify_password_sha1, AuthUser},
        ResponseStructure, ResponseStructureError,
    },
    common::fun::sm4_decrypt_login,
    errors::common::CommonError,
    models::users::find_user_update_error_count,
    server::{db::DBPool, global::CONF},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginAuthData {
    pub username: String,
    pub password: String,
    pub timestamp: u64,
}
#[derive(Serialize)]
struct LoginSuccessData {
    token: String,
    user: serde_json::Value,
}

pub async fn sign(
    pool: web::Data<DBPool>,
    auth_data: web::Json<LoginAuthData>,
    security_dir: web::Path<String>,
) -> Result<HttpResponse, Error> {
    if CONF
        .app
        .security_dir
        .as_str()
        .eq(&security_dir.into_inner())
    {


    let system_time = UNIX_EPOCH + Duration::from_millis(auth_data.timestamp);
    let now = SystemTime::now();
    if let Ok(duration) = now.duration_since(system_time) {

        if duration > Duration::from_secs(60) {
            return Ok(HttpResponse::InternalServerError().json(ResponseStructureError {
                success: false,
                code: 101,
                message: String::from("Invalid timestamp.-1"),
            }));
        } 
    } /*else {
        return Ok(HttpResponse::InternalServerError().json(ResponseStructureError {
            success: false,
            code: 101,
            message: String::from("Invalid timestamp.-2"),
        }));
    }*/
        

     
       let user_data :LoginAuthData = LoginAuthData{
           username:sm4_decrypt_login(auth_data.username.to_string(), CONF.app.security_dir.to_string(),auth_data.timestamp.to_string()),
           password:sm4_decrypt_login(auth_data.password.to_string(), CONF.app.security_dir.to_string(),auth_data.timestamp.to_string()),
           timestamp:0,
       };
 

        let user = web::block(move || query(user_data, pool)).await??;

        //   Identity::login(&req.extensions(), serde_json::to_string(&user).unwrap()).unwrap();
        //返回Cookies
        return Ok(HttpResponse::Ok()
            .cookie(
                actix_web::cookie::Cookie::build(
                    "token",
                    generate_jwt(&serde_json::to_string(&user).unwrap()).unwrap(),
                )
                .finish(),
            )
            .json(ResponseStructure {
                success: true,
                code: 200,
                message: String::from("success"),
                data: Some(LoginSuccessData {
                    token: generate_jwt(&serde_json::to_string(&user).unwrap()).unwrap(),
                    user: serde_json::json!({"id":user.id,"username":user.username,"authority":user.authority})
                }),
              
            }));

        //return Ok(HttpResponse::Ok().body("Login Success"))
    }
    Ok(HttpResponse::NotFound().finish())
}

pub async fn get_security_dir(
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    // 尝试从查询参数中获取 "key"，如果 "key" 参数不存在，返回 400 错误
    let security_dir = match query.get("v") {
        Some(key) => key,
        None => {
            return Ok(
                HttpResponse::InternalServerError().json(ResponseStructureError {
                    success: false,
                    code: 101,
                    message: String::from("Security directory does not match the query parameter"),
                }),
            )
        }
    };

    if security_dir == &CONF.app.security_dir {
        // 如果匹配，返回成功的响应
        Ok(HttpResponse::Ok().json(ResponseStructureError {
            success: true,
            code: 200,
            message: String::from("success"),
        }))
    } else {
        // 如果不匹配，返回 404 错误，表示未找到
        Ok(
            HttpResponse::InternalServerError().json(ResponseStructureError {
                success: false,
                code: 101,
                message: String::from("Security directory does not match the query parameter"),
            }),
        )
    }
}

fn query(auth_data: LoginAuthData, pool: web::Data<DBPool>) -> Result<AuthUser, CommonError> {
    use crate::models::structure::{
        rp_users::Users,
        schema::rp_users::dsl::{rp_users, status, username},
    };
    use diesel::{prelude::*, ExpressionMethods, QueryDsl};

    let mut conn = pool.get().unwrap();
    let mut items = rp_users
        .filter(username.eq(&auth_data.username))
        .load::<Users>(&mut conn)?;

    // let items = rp_users::table
    //     .filter(rp_users::username.eq(auth_data.username))
    //     .first::<Users>(&mut *conn);

    if let Some(user) = items.pop() {
        if user.error_count >= 5 {
            diesel::update(rp_users.find(user.id))
                .set(status.eq(-1))
                .execute(&mut conn)?;
            return Err(
                CommonError::BadRequest(String::from(t!("auth.login.username_band"))).into(),
            );
        }
        if let Ok(matching) = verify_password_sha1(
            &user.password,
            &user.salt,
            &auth_data.password,
        ) {
            if matching {
                find_user_update_error_count(user.id, 0, &mut conn).unwrap();
                return Ok(user.into());
            } else {
                find_user_update_error_count(user.id, user.error_count + 1, &mut conn).unwrap();
                return Err(CommonError::BadRequest(String::from(
                    t!("auth.login.password_error",count=> 5 - (user.error_count+1)),
                ))
                .into());
            }
        } else {
            return Err(CommonError::BadRequest(String::from(
                t!("auth.login.password_error",count=> 5 - (user.error_count+1)),
            ))
            .into());
        }
    }
    Err(CommonError::BadRequest(String::from(t!("auth.login.username_error"))).into())
}
