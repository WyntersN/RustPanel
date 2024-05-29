/*
 * @Descripttion:
 * @version:
 * @Author: Wynters
 * @Date: 2024-05-07 15:35:18
 * @LastEditTime: 2024-05-29 18:36:53
 * @FilePath: \RustPanel\src\bin\panel.rs
 */
use actix_files as fs;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::{time::Duration, Key}, get, web, Responder};
use diesel::{r2d2::ConnectionManager, SqliteConnection};
use rust_panel::{
    api, common::sys::get_all_ip_addresses, errors::handlers, log_error, log_info, log_warn, models::user::find_user_by_id, service::{
        db::{install, DBPool},
        global::{CONF, SESSION_KEY},
    }, test
};
use std::{collections::HashMap, fs::File, process};
use std::io::Write;

#[get("/hello/{name}")]
async fn greet(
    name: web::Path<String>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    //获取Get Key参数

    // 从查询参数中获取 key 参数的值
    if let Some(key_value) = query.get("key") {
        println!("The value of 'key' parameter is aaa: {}\n", key_value);
    } else {
        println!("The 'key' parameter is not present in the URL.\n");
    }

    format!("Hello {}!", name)
}



#[tokio::main]
async fn main() -> std::io::Result<()> {
    init();
    let pool: DBPool = r2d2::Pool::builder()
        .max_size(CONF.database.max_pool)
        .build(ConnectionManager::<SqliteConnection>::new(
            CONF.database.path.as_str(),
        ))
        .expect("Failed to create pool.");
    install(&pool);
    test::demo(&pool).await;

    if !port_check::is_local_port_free(CONF.app.port) {
        log_error!("Error: The Port {} is already in use.", CONF.app.port);
        std::process::exit(1);
    }

    {
        println!("=========Listening AND Login address=========");
        match get_all_ip_addresses() {
            Ok(ip_addresses) => {
                for ip in ip_addresses {
                    println!("http://{}:{}/#/admin/login?v={}", ip, CONF.app.port, CONF.app.security_dir);
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }

        // let resp_ip_addr = reqwest::Client::builder()
        //     .build()
        //     .unwrap()
        //     .get("http://ipinfo.io/ip")
        //     .send()
        //     .await
        //     .unwrap()
        //     .text()
        //     .await
        //     .unwrap();

        let resp_ip_addr: String = ureq::get("http://ipinfo.io/ip")
        .call()
        .unwrap()
        .into_string()
        .unwrap();

        println!(
            "http://{}:{}/#/admin/login?v={}",
            resp_ip_addr, CONF.app.port, CONF.app.security_dir
        );

  

        // DB.with(|db| {
        //     let conn = db.write().unwrap();
        //     let mut stmt = conn
        //         .prepare("SELECT username,initial_password FROM rp_users WHERE id = 1")
        //         .unwrap();
        //     stmt.query_row(params![], |row| {
        //         match row.get::<usize, String>(0) {
        //             Ok(username) => println!("username:{}", username),
        //             Err(err) => println!("Error retrieving user ID: {:?}", err),
        //         }
        //         match row.get::<usize, String>(1) {
        //             Ok(password) => println!("password:{}", password),
        //             Err(err) => println!("Error retrieving user ID: {:?}", err),
        //         }
        //         Ok(())
        //     })
        //     .unwrap();
        // });

        match find_user_by_id(1, &pool) {
            Ok(Some(user)) => {
                // 用户存在，处理用户数据
                log_info!("username:{}", user.username);
                log_info!("password:{}", user.initial_password);
            }
            Ok(None) => {
                log_warn!("User not found");
            }
            Err(err) => {
                log_error!("Error retrieving user: {:?}", err);
            }
        }
        println!("=========Listening AND Login address=========");
    }
    
    use actix_web::{middleware, App, HttpServer};
    // start HTTP Server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::JsonConfig::default().error_handler(handlers::json_error_handler))
            .app_data(web::PathConfig::default().error_handler(handlers::path_error_handler))
                //.app_data(web::JsonConfig::default().limit(4096))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            //.wrap(api::middleware::Auth)  
            // .wrap(SessionMiddleware::builder(CookieSessionStore::default(),Key::generate())
            // .session_lifecycle(
            //     actix_session::config::PersistentSession::default()
            //         .session_ttl(Duration::hours(CONF.app.session_ttl)),
            // )
            // .build())
            //.wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    Key::from(SESSION_KEY),
                    //Key::generate()
                )
                .session_lifecycle(PersistentSession::default().session_ttl(Duration::hours(CONF.app.session_ttl)))
                .cookie_name(String::from("token"))
                .cookie_secure(false)
                //.cookie_domain(Some(domain.clone()))
                .cookie_path(String::from("/"))
                .build(),
            )
            .service(greet)
            .service(api::route::v1())
            .service(fs::Files::new("/", "./public").index_file("index.html"))          
    })
    .workers(CONF.app.workers)
    .bind((CONF.app.host.as_str(), CONF.app.port))?
    .run()
    .await
}

fn init() {
    log4rs::init_file("./config/log4rs.yaml", Default::default()).expect("init Error...log4rs init failed!");
    rust_i18n::set_locale("zh-CN");



    let pid = process::id();

    // 创建或打开 .pid 文件
    let mut file = match File::create("panel.pid") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error creating file: {}", err);
            return;
        }
    };

     // 将 PID 写入 .pid 文件
     match write!(file, "{}", pid) {
        Ok(_) => println!("PID {} written to panel.pid", pid),
        Err(err) => eprintln!("Error writing to file: {}", err),
    }
    // let mut aaa = false;

    // DB.with( |db| {
    //     let  conn = db.write().unwrap();

    //     aaa = conn.is_autocommit();

    //     conn.execute(
    //         "CREATE TABLE if not exists user (
    //             id INTEGER PRIMARY KEY,
    //             name TEXT NOT NULL,
    //             age INTEGER
    //          )",[]
    //     ).unwrap();

    // });
    // println!("---------{}",aaa);
}
