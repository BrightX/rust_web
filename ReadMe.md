# Rust Web

创建应用

```bash
cargo new ruse_web
```

运行

```bash
cargo run
```

构建

```bash
# 默认构建 dev
cargo build

# 构建生产环境
cargo build --release

```

跨平台交叉编译
```bash
# 查看所有平台列表
rustup target list

rustup target list --installed

# Linux arm64
aarch64-unknown-linux-gnu
aarch64-unknown-linux-musl
# Linux x86_64
x86_64-unknown-linux-gnu
x86_64-unknown-linux-gnux32
x86_64-unknown-linux-musl

# 添加指定目标平台
rustup target add <target>
rustup target add x86_64-unknown-linux-musl

# 构建指定目标平台
#cargo build --release --target <target>
#cargo build --release --target x86_64-unknown-linux-musl

cargo install cross
cross build --target=x86_64-unknown-linux-musl
```



问题：

```bash
 Is `x86_64-linux-gnu-gcc` installed?
 Is `musl-gcc` installed? (see https://github.com/rust-lang/cc-rs#compile-time-requirements for help)
```

参考：

https://doc.rust-lang.org/cargo/reference/config.html#targettriplelinker 

https://rust-lang.github.io/rustup/cross-compilation.html 



