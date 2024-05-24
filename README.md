<!--
 * @Descripttion: 
 * @version: 
 * @Author: Wynters
 * @Date: 2024-05-07 21:19:37
 * @LastEditTime: 2024-05-21 17:01:23
 * @FilePath: \RustPanel\README.md
-->
# RustPanel

_language: English, [中文](README.cn-zh.md)

## 🤔 What is this?
![image](https://github.com/WyntersN/RustPanel/assets/27792650/a7e0cadb-2c44-44c5-9384-63f815edbee8)

This is a new generation server panel developed based on Rust.It is capable of monitoring server CPU, memory, traffic inflow and outflow, as well as managing databases, websites, dockers, support third-party plugin development and other operations

## Development plan
|service |description|
|----------------|--------------------------------|
|Support OS|`Linux、Windows、Mac`
|Website Management|`HTTP SERVER(nignx、apache、caddy) development language (PHP、Node、Python、Java、Go)`
|Database Management|`MySql、PostgreSQL、MongoDB、redis`
|Docker|`This is a key point, and it is highly likely that K8s will be added in the later stages of this project to achieve visual operation of K8s management`|
|File Management|`You can edit code online and visualize file management.This is a prominent security issue, so we will encrypt the files during transmission and saveing`
|Firewall|`Port allow and deny, SSH management`
|Asset Management|`Manage multiple panels with one panel, simply add the main server connection address and API key to the configuration file to manage multiple panels`
|OpenApi|`You can manage it yourself by using APIs`
|Plugins|`Plugins that support Rust, Go, C, and C++development will be implemented`
|Multilingual|`English、Chinese`
