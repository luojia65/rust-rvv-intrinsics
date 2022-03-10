build example with:

```
cargo build --example vadd --target riscv64gcv-unknown-none-elf.json -Z build-std=core,panic_abort
```
```
cargo build --example vadd --target riscv64gcv-unknown-linux-gnu.json -Z build-std=std,panic_abort
```

dump instrucions:

```
riscv64-unknown-linux-gnu-objdump -d target/riscv64gcv-unknown-linux-gnu/debug/examples/vadd
```
