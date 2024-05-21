/*
 * @Descripttion: 
 * @version: 
 * @Author: Wynters
 * @Date: 2024-05-09 19:10:36
 * @LastEditTime: 2024-05-20 17:41:04
 * @FilePath: \RustPanel\src\api\route.rs
 */
use actix_web::web;

use super::v1;


pub fn v1() -> actix_web::Scope {
    web::scope("/api")
        .service(web::scope("/v1")
        //.wrap(middleware::Auth)
            .service(
                web::scope("/login")
                .route("/sign/{security_dir}",web::post().to(v1::login::sign))
                .route("/security_dir",web::get().to(v1::login::get_security_dir))
            )
            .service(
                web::resource("/file/list")
                    .route(web::get().to(v1::file::list))
            )
            .service(
                web::resource("/file/content")
                    .route(web::get().to(v1::file::content))
            ).service(
                web::resource("/file/save")
                    .route(web::post().to(v1::file::save))
            )
            .service(
                web::resource("/user/me")
                    .route(web::get().to(v1::user::get_me))
            )
            .service(
                web::resource("/user/menus")
                    .route(web::get().to(v1::user::get_menus))
            )
            .service(
                web::resource("/os_info")
                    .route(web::get().to(v1::os::os_info))
            )
            .service(v1::ping)
)
}