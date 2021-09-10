# Windows 下安装和使用 diesel 库



在被 rust-postgresql 折磨之后，选择了 diesel 库，这才了解到**对象关系映射**（Object Relational Mapping, ORM） ，可以像使用本地变量和函数一样操作关系型数据库。这有点像非关系型数据库（如`MongoDB`）操作方式的意味，不过不知道孰先孰后。

### 初步认识

diesel 提供了一个叫 diesel_cli 的工具，用来将 MySQL、PostgreSQL 或 Sqlite 上的数据表结构转换成代码，类似于 (schema.rs)，或代为执行一些 SQL 语句：

```
// A simple schema.rs
table! {
    verifications (id) {
        id -> Int4,
        uid -> Int4,
        login_type -> Int4,
        account -> Varchar,
        credential -> Nullable<Varchar>,
    }
}
```



也可以使用 `diesel print-schema` 命令将应该生成于 `schema.rs` 的内容直接输出。

之后配套相关的 `struct` 就可以与 table 关联起来。

类似于 Django 的 ORM，diesel_cli 也有一个 `migration` ，可以快速执行给定的 sql 语句进行创建或删除表的操作（对应于 `up.sql` 和 `down.sql`。可以通过 `diesel migration list` 查看 diesel_cli 所认为的表的状态，必要时可以用 DataGrip 或 PgAdmin 手动连接数据库刷新。

在初始化之后，就不常用到 `diesel_cli` 了。

### 安装

找了找 [Diesel的文档](http://diesel.rs/guides/getting-started/)（[中文译版](https://blog.csdn.net/m0_37696990/article/details/84304876)），可以在命令行执行如下：

```
cargo install diesel_cli
```

由于现在只需要使用 postgres，官方文档在后面提示说可以只安装支持 postgres 的部分：

```
cargo install diesel_cli --no-default-features --features postgres
```

### 解决一个问题

等了一会儿，报错：

```
error: linking with `link.exe` failed: exit code: 1181
  |
  = note: "/* 省略一些链接参数 */"
  = note: LINK : fatal error LNK1181: cannot open input file 'libpq.lib'

error: aborting due to previous error

error: failed to compile `diesel_cli v1.4.0`, intermediate artifacts can be found at `C:\Users\sunnysab\AppData\Local\Temp\cargo-installhOpejX`

Caused by:
  could not compile `diesel_cli`.

To learn more, run the command again with --verbose.
```

提示缺少 libpq.lib。 libpq 是 postgresql 的 C 接口的二进制库，有 libpq.lib 和 libpq.dll 两个文件，前者导入函数声明，后者是函数实现（用C++的说法），配套使用。去 [PostgreSQL 官网](https://www.postgresql.org/download/windows/)下载 Windows 版安装包（Installer和zip archive均可），然后把 .lib 文件放到 `VC\Tools\MSVC\版本\lib\x64`目录下，即可重新编译。再进入 PostgreSQL 里的 bin 文件夹，将 `lib*.dll`拷贝到项目的目录下。

### 开始使用

和许多 Rust 程序一样，diesel 默认通过环境变量获取数据库路径，也可以通过 `--database-url` 参数指定。打开命令提示符，进入项目目录，设置一个环境变量用于指示 PostgreSQL 数据库的路径：

```
set DATABASE_URL=postgresql://USERNAME:PASSWORD@HOST.DOMAIN:PORT/DATABASE_NAME
```

然后执行 setup。 setup 会将本地 `schema.rs` 同步到最新。不过更多的时候，手动同步更新数据库和 `schema.rs` 即可。

```
D:\Workspace\Projects\Rust\pegions>diesel setup
no schema has been selected to create in
```

剩下的内容就可以参考官方文档，在 `Cargo.toml` 中依赖 `diesel` 并进行后续编码，这里赘述意义不大。前文中提到了中英文的链接。

因为 Diesel 是个人项目，在文档上有很多不足，中文资料更是稀少。不过日后随着 rust 使用者越来越多，这些问题会有解决吧。

### 参考

1. rgwu. [解决rust 在使用diesel r2d2 编写postgresql数据库连接池时编译不通过报错](https://blog.csdn.net/m0_37696990/article/details/84262678)
2. https://dba.stackexchange.com/questions/106057/error-no-schema-has-been-selected-to-create-in