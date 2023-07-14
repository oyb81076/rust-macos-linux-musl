## 大陆镜像设置以及交叉编译设置

```sh
echo "export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static" >> ~/.zprofile
echo "export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup" >> ~/.zprofile
mkdir -p ~/.cargo
cat >> ~/.cargo/config <<EOF
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
EOF
```

## 交叉编译设置

```
cat >> ~/.cargo/config <<EOF
[target.x86_64-unknown-linux-musl]
linker="x86_64-linux-musl-gcc"
EOF
rustup update
rustup target add x86_64-unknown-linux-musl
cargo install cross
brew install filosottile/musl-cross/musl-cross
```
