# rust_os

Add file in .cargo/config.toml

[unstable]
build-std = ["core", "compiler_builtins"]
build-std-features = ["compiler-builtins-mem"]

[build]
target = "x86_64-blog_os.json" # Optional

rustup component add llvm-tools-preview

cargo install bootimage

Building Kernel
cargo build --target x86_64-rust_os.json

Creating bootable image
cargo bootimage

