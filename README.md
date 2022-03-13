build example with:

```
cargo build --example vadd --target riscv64gcv-unknown-none-elf.json -Z build-std=core,panic_abort
```
```
cargo build --example vadd --target riscv64gcv-unknown-linux-gnu.json -Z build-std=std,panic_abort
```

you must have riscv-binutils >= 2.38 installed, if latest riscv-gnu-toolchain does not include it, you should
build it from source. ref: a [blog post](https://jia.je/software/2022/01/25/rvv-1.0-toolchain/) from @jiegec

dump instrucions:

```
riscv64-unknown-linux-gnu-objdump -d target/riscv64gcv-unknown-linux-gnu/debug/examples/vadd
```
