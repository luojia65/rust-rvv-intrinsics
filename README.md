build example with:

```
cargo build --example vadd --target riscv64gcv-unknown-none-elf.json -Z build-std=core,panic_abort
```
```
cargo build --example vadd --target riscv64gcv-unknown-linux-gnu.json -Z build-std=std,panic_abort
```

you must have riscv-binutils >= 2.38 installed. if you are building the toolchain from source, modify the .gitmodules:

```diff
-	branch = riscv-binutils-2.36.1
+	branch = riscv-binutils-2.38
```

dump instrucions:

```
riscv64-unknown-linux-gnu-objdump -d target/riscv64gcv-unknown-linux-gnu/debug/examples/vadd
```
