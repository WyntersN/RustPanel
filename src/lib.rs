/*
 * @Descripttion: 
 * @version: 
 * @Author: Wynters
 * @Date: 2024-05-05 20:56:38
 * @LastEditTime: 2024-05-19 20:59:30
 * @FilePath: \RustPanel\src\lib.rs
 */
extern crate actix_web;
extern crate diesel;
extern crate crypto;
rust_i18n::i18n!();
pub mod server;
pub mod common;
pub mod errors;
pub mod models;
pub mod api;
pub mod test;
