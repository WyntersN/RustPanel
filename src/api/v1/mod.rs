/*
 * @Descripttion: 
 * @version: 
 * @Author: Wynters
 * @Date: 2024-05-09 19:00:34
 * @LastEditTime: 2024-05-18 13:50:53
 * @FilePath: \RustPanel\src\api\v1\mod.rs
 */

pub mod login;
pub mod  user;
pub mod os;
pub mod file;


use actix_web::{get, HttpResponse,Error};
use actix_session::Session;




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
