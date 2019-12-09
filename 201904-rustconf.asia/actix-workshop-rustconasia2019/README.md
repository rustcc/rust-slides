# RustConAsia 2019大会workshop

### 环境准备

- Rust 最新稳定版
- Docker 

windows可以装docker for win

对于Mac环境，使用

- Docker Desktop for Mac 

或者 

需要使用docker-machine配置好docker 

```
// 确保已安装VirtualBox
$ brew cask install virtualbox;
$ docker-machine create --driver virtualbox default
$ docker-machine env default
$ eval $(docker-machine env default)
```



安装好postgresql的docker镜像,,或者使用Sqlite/Mysql

```
$ docker pull postgres
$ docker run --name postgres -e POSTGRES_PASSWORD=123456 -d postgres 
```

配置Cargo 镜像

```
$  sudo vi ~/.cargo/config
```

```rust
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "http://mirrors.ustc.edu.cn/crates.io-index"
```

安装好diesel-cli

```rust
$ cargo install diesel_cli --no-default-features --features postgres
```

### 内容导读

- [概念导读](./concept_to_guide/RustConAsia2019WorkShop.pdf)
- [现场代码](./workshop-todo)

Workshop流程说明：

- actor模型介绍
- actix重点概念介绍
- actix-web重点概念介绍
- Diesel重点概念介绍
- 现场todo 接口实现，跑通接口逻辑
- 增加Dockerfile方便打包为一个独立的镜像