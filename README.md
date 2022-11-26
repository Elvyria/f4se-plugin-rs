# F4SE Rust Plugin

![F4SE](https://img.shields.io/badge/F4SE-v0.6.23-pink?style=flat-square&link=https://github.com/ianpatt/f4se/releases/download/v0.6.23/f4se_0_06_23.7z)
![License](https://img.shields.io/github/license/Elvyria/f4se-plugin-rs?label=License&color=yellow&style=flat-square)

## Building

* Clone Repository
```bash
git clone --depth 1 https://github.com/Elvyria/f4se-plugin-rs
cd f4se-plugin-rs
```

* Build F4SE
```bash
git clone --depth 1 https://github.com/ianpatt/common lib/common
git clone --depth 1 --branch v0.6.23 https://github.com/ianpatt/f4se lib/f4se

cd lib

cmake -B common/build -S common -DCMAKE_INSTALL_PREFIX=extern common
cmake --build common/build --config Release --target install

cmake -B f4se/build -S f4se -DCMAKE_INSTALL_PREFIX=extern f4se
cmake --build f4se/build --config Release

cd ..
```

* Build Plugin
```bash
cargo build --release
```
