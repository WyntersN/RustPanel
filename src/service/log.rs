/*
 * @Descripttion: 
 * @version: 
 * @Author: Wynters
 * @Date: 2024-05-07 15:57:30
 * @LastEditTime: 2024-05-09 18:27:51
 * @FilePath: \rust_panel\src\server\log.rs
 */


#[macro_export]
macro_rules! log_info {

    (target: $target:expr, $($arg:tt)+) => (log::info!(target: $target, $($arg)+));

    ($($arg:tt)+) => (log::info!( $($arg)+))

}
#[macro_export]
macro_rules! log_warn {

    (target: $target:expr, $($arg:tt)+) => (log::warn!(target: $target, $($arg)+));

    ($($arg:tt)+) => (log::warn!( $($arg)+))

}
#[macro_export]
macro_rules! log_error {

    (target: $target:expr, $($arg:tt)+) => (log::error!(target: $target, $($arg)+));

    ($($arg:tt)+) => (log::error!( $($arg)+))
    
}
