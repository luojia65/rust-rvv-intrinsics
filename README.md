build example with:

```
cargo build --example vadd --target riscv64gcv-unknown-none-elf.json -Z build-std=core,panic_abort
```
```
cargo build --example vadd --target riscv64gcv-unknown-linux-gnu.json -Z build-std=std,panic_abort
```


todo: following json only work on linux

```
  "linker": "riscv64-unknown-linux-gnu-gcc",
  "linker-flavor": "gcc",
```
