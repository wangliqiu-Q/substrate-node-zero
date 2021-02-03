Analyze 
======


runtime
-----
```sh
cargo expand --lib > runtime.rs
```
因为 bin/node/runtime/src/lib.rs:

    #[cfg(feature = "std")]
    include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

所以对于 runtime.rs 要删除含有 WASM_BINARY 和 WASM_BINARY_BLOATY 两行。













