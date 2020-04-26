- `cargo build` to run and check compilation.
  [cargo](https://github.com/rust-lang/cargo)
- `wasm-pack build --release --no-typescript` to create a `./pkg` directory. [wasm-pack](https://github.com/rustwasm/wasm-pack) that will contain:
```shell
.rw-r--r--    1 lrlna 26 avr 13:14 .gitignore
.rw-r--r--  243 lrlna 26 avr 13:14 package.json
.rw-r--r--  508 lrlna 26 avr 13:14 README.md
.rw-r--r-- 1,4k lrlna 26 avr 13:14 wasm_string.js
.rw-r--r--  891 lrlna 26 avr 13:14 wasm_string_bg.wasm
```
