<!--
 * @Descripttion: 
 * @version: 
 * @Author: Wynters
 * @Date: 2024-05-21 17:01:41
 * @LastEditTime: 2024-05-21 17:05:49
 * @FilePath: \RustPanel\README.cn-zh.md
-->
# RustPanel

_语言: 中文, [English](README.md)

## 🤔 这是什么?
![image](https://github.com/WyntersN/RustPanel/assets/27792650/5080e1e5-24f5-4fdc-a85e-be45061626b5)

这是基于Rust开发的新一代服务器面板。它能够监控服务器CPU、内存、流量流入和流出，以及管理数据库、网站、Docker，支持第三方插件开发等操作

**当项目能够正常运行时将会发布一个构建版本(Linux和Win,我相信这很快)***

## 项目计划

|服务 |描述|
|----------------|--------------------------------|
|支持系统|`Linux、Windows、Mac`
|站点管理|`支持的HTTP 服务器(nignx、apache、caddy) 支持的开发语言 (PHP、Node、Python、Java、Go)`
|服务器管理|`MySql、PostgreSQL、MongoDB、redis`
|Docker|`这是一个关键点，极有可能在本项目后期增加K8s，以实现K8s管理的可视化操作`
|文件管理|`您可以在线编辑代码并可视化文件管理。这是一个突出的安全问题，因此我们将在传输和保存过程中对文件进行加密`
|防火墙|`端口的开放和拒绝，SSH管理`
|资产管理|`使用一个面板管理多个面板，只需将主服务器连接地址和API密钥添加到配置文件中即可管理多个板`
|开放API|`您可以使用API自行管理`
|插件|`将实现支持Rust、Go、C和C++开发的插件`
|国际化|`中文、英语`
