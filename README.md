# Example: Rust Runtime Package System

This is a straightforward example illustrating a runtime Rust-to-Rust FFI system that relies on dynamic libraries (dylibs).

> This package aims to provide an example of achieving dynamism through FFI (Foreign Function Interface).
> It does not cover aspects like package metadata, path resolution, compression, mutability, etc.

## Usage

### 1. Building the Test Package

This system assumes that you can create your packages just like regular cdylib crates.
Since it follows the usual crate structure, you can build it using Cargo as follows:

```bash
cd core-package
cargo build
cd ..
```

This command will generate the `libcore_package.dylib` file inside the `core-package/target/debug/` directory, which we can later use for FFI.

The example exposes two functions:
- `add_numbers(args: *mut FFIValue, len: size_t) -> FFIValue`
- `print(args: *mut FFIValue, len: size_t) -> FFIValue`

To verify if these functions are accessible, you can use the `nm` utility:

```bash
nm core-package/target/debug/libcore_package.dylib | grep "print"
# 0000000000004840 T _print
```

This output indicates that the `print` function is accessible at runtime.

### 2. Consuming The Package

Once we have built the example package, we can run the consumer as follows:

```bash
cargo run
# hey
```

This command will invoke our package's `print` function.

## How it Works

Creating numerous function signatures and mapping them at runtime is not practical.
This system takes a more dynamic approach by sending an array of enums (`FFIValue`).
The `FFIValue` enum contains basic data types like `f64`, `bool`, and `char`.
The library implementer can consume the array of values to obtain arguments.

> Note: The `FFIValue` enum must be consistent in both the library and the consumer. Optionally, these two crates can obtain `FFIValue` from a separate crate.

> Author: Yunus Emre Dilber ([@yunusemredilber](https://github.com/yunusemredilber))
