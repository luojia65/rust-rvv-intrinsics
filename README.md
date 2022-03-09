build example with:

```
cargo build --example vadd --target riscv64gcv-unknown-none-elf.json -Z build-std=core,panic_abort
```

todo: following command won't work

```
cargo build --example vadd --target riscv64gcv-unknown-linux-gnu.json -Z build-std=std,panic_abort
```
