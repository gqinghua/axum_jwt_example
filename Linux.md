# 		diesel_cli安装问题解决

置顶 郭大侠行走江湖 2020-11-26 12:56:55  232  收藏
分类专栏： diesel diesel_cli 文章标签： rust
编辑 版权
diesel_cli does not build:
在github上issuess上搜索此问题可得到以下问题,此页面由谷歌自行翻译,大致知道意思就行
安装不了diesel_cli ,可能缺少数据库客户端,各系统安装方法示例,
地址:
https://github.com/diesel-rs/diesel/issues/1592,
https://github.com/diesel-rs/diesel/blob/master/guide_drafts/backend_installation.md
解决方案
安装后端客户端库
Diesel支持SQLite，PostgreSQL和MySQL作为数据库后端。默认情况下，diesel_cli 要求安装所有三个后端的客户端库。如果缺少一个，那么cargo install diesel_cli将抛出类似以下的错误：

note: ld: library not found for -lmysqlclient
clang: error: linker command failed with exit code 1 (use -v to see invocation)
要在diesel_cli没有所有后端的情况下进行安装，请指定–no-default-features。用货的–features选项来指定postgres，sqlite和/或mysql。例如，要仅使用sqlite进行安装，请运行：

cargo install diesel_cli --no-default-features --features sqlite
对于依赖柴油的项目，您可以在中指定所需的后端Cargo.toml。例如：

[dependencies]
diesel = { version = “X.X.X”, features = [“sqlite”] }
下面是运行命令以使用各种程序包管理器安装适当的数据库客户端。

Debian / Ubuntu
SQLite的
sudo apt-get install libsqlite3-dev

PostgreSQL的
sudo apt-get install libpq-dev

的MySQL
安装以下内容以添加MySQL APT存储库。

wget https://dev.mysql.com/get/mysql-apt-config_0.8.15-1_all.deb
sudo dpkg -i mysql-apt-config_0.8.15-1_all.deb
选择。

检索软件包的新列表

sudo apt-get update
安装客户端库

sudo apt-get install libmysqlclient-dev
有关更多详细信息，请参见MySQL文档。

CentOS / Fedora
SQLite的
sudo yum install sqlite-devel

PostgreSQL的
sudo yum install postgresql-devel

的MySQL
sudo yum install mysql-devel

拱
SQLite的
sudo pacman -Su sqlite

PostgreSQL的
sudo pacman -Su postgresql

的MySQL
sudo pacman -Su mysql

Mac OSX
SQLite的
默认情况下已安装。

PostgreSQL的
brew install postgresql

的MySQL
brew install mysql

视窗
PostgreSQL的
在Windows上安装PostgreSQL最简单的方法是使用EnterpriseDB公司的图形安装程序：https://www.postgresql.org/download/windows/，但你也可以只是二进制安装：HTTPS：//www.enterprisedb。 com / download-postgresql-binaries。

最后，您可以pg_env.bat在PostgreSQL\10系统中安装PostgreSQL的地方运行，这将完成您需要的所有设置。如果使用EnterpriseDB的图形安装程序，则应位于中C:\Program Files\。

或者，您可以将bin/PostgreSQL目录添加到PATH环境变量中。
