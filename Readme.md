# 使用RISC-V固件的内核网络部署和维护方案

## 参赛信息

本项目参加2023全国大学生计算机系统能力大赛操作系统设计赛-功能挑战赛，选题为[proj140-RV-firmware-for-net](https://github.com/oscomp/proj140-RV-firmware-for-net)。

参赛小组：

- 北方工业大学 于新程 
- 北方工业大学 林达华

指导老师：

- 北方工业大学 宋丽华
- 北方工业大学 吴爱燕

## 目标描述

- 选择一块RISC-V开发板。编写或选用裸机的网络协议栈和网卡驱动，使其能在此开发板的M态收或发网络包。
- 选用合适的通讯协议，从文件服务器上下载内核文件。
- 部署下载好的内核到内存并启动此内核。 本题要求至少能下载和启动rCore-Tutorial级别的简单内核。

## 项目进度
本项目选用M1W dock开发板，其搭载了k210芯片和esp8285 wifi模块，符合项目的需求。  
目前，本项目已成功使用uart向wifi模块发送AT命令，使其连接上网络，并且初步搭建好了tcp传输协议栈
。正在学习如何使用smoltcp完成sftp协议，以便向服务器发送请求，来接收rCore内核。  
在配置好项目相关环境后，修改`src/conncection.rs`中的wifi账号和密码，即可通过`make run BOARD=k210`命令来测试。


## 仓库文件简介

- src/net/connection.rs:设置gpio、uart等，与wifi模块进行通信并联网
- src/net/dvc.rs:用于接收、发送数据包的接口
- src/net/tcp_connect.rs:通过smoltcp建立tcp的socket连接
- src/net/mod.rs:网络模块的主函数
