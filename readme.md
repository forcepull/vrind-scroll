# Vrind-scroll
## Installing rust
See https://www.rust-lang.org/tools/install
## How to build
Building requires cargo-web
> cargo install cargo-web

Building for desktop and web:
> cargo build && cargo web build
## How to run
### Desktop
Execute the following command in the root directory:
> RUST_BACKTRACE=1 cargo run
### Web
Execute the following command in the root directory:
> cargo web start
Go to http://localhost:8000