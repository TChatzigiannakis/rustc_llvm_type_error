# rustc_llvm_type_error
A program that type checks in Rust but gets a type error in LLVM.

To build this program, use:

```
cargo build
```

You will get the following output:

```llvm
   Compiling rustc_llvm_type_error v0.1.0 (SomePath\rustc_llvm_type_error)
Function return type does not match operand type of return inst!
  ret %"alloc::vec::Vec<alloc::boxed::Box<core::ops::function::Fn<(&u8), Output=()>>>.0"* %6, !dbg !84
 %"alloc::vec::Vec<alloc::boxed::Box<core::ops::function::Fn<(&u8), Output=()>>>"*LLVM ERROR: Broken function found, compilation aborted!
error: Could not compile `rustc_llvm_type_error`.
```
