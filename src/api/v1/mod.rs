/*
 * @Descripttion: 
 * @version: 
 * @Author: Wynters
 * @Date: 2024-05-09 19:00:34
 * @LastEditTime: 2024-05-25 16:23:02
 * @FilePath: \RustPanel\src\api\v1\mod.rs
 */

pub mod login;
pub mod  user;
pub mod os;
pub mod file;


use actix_web::{get, web, Error, HttpResponse};
use actix_session::Session;
use serde::Deserialize;

use crate::common::fun::process_pid_runing;




#[get("/ping")]
async fn ping(session: Session) ->  Result<HttpResponse, Error> {
    //获取Get Key参数
     if let Some(count) = session.get::<i32>("counter")? {
        session.insert("counter", count + 1)?;
    } else {
        session.insert("counter", 1)?;
    }
    Ok(HttpResponse::Ok().append_header(("Content-Disposition", "attachment; filename=\"1.txt\"")) // 设置文件名
    .append_header(("Content-Type", "application/octet-stream")).body("----------------"))

    // Ok(HttpResponse::Ok().body(format!(
    //     "Count is {:?}!",
    //     session.get::<i32>("counter")?.unwrap()
    // )))
}
#[derive(Deserialize)]
pub struct Params {
    pub name: String,
    pub pid: i32,
}


#[get("/pid_runing")]
async fn pid_runing(data: web::Query<Params>) ->  Result<HttpResponse, Error> {
 
    Ok(HttpResponse::Ok().body(format!("{:?}",process_pid_runing(data.pid,Some(data.name.clone())))))

    // Ok(HttpResponse::Ok().body(format!(
    //     "Count is {:?}!",
    //     session.get::<i32>("counter")?.unwrap()
    // )))
}
