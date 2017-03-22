# cl-gen-moodle

Small project. Main target is to learn Rust. This tool generates a Quiz with different question types for Moodle Quiz in XML format.
To build and run it, just run `cargo run` in the project root.

## Cross compilation for Windows and Linux

I'm developing this tool on macOS, but it is intended to run on Windows as well in the future. Rust is able to cross compile code to different targets.

### Windows

First set the rust environment to nightly.

```
rustup default nightly
```


Setup the toolchain to build the Windows target on macOS. Use Homebrew for that.

```
brew install mingw-w64
rustup target add x86_64-pc-windows-gnu
```


Configure cargo.

```
vim ~/.cargo/config

[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-gcc-ar"
```


And you are ready to build the Windows executable.

```
cargo build --target x86_64-pc-windows-gnu
```


### Linux

First set the rust environment to nightly.

```
rustup default nightly
```


Setup the linux toolchain with musl target.

```
brew install filosottile/musl-cross/musl-cross
rustup target add x86_64-unknown-linux-musl
```
Note: the Homebrew install step takes a while...


Setup cargo.

```
vim ~/.cargo/config

[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
```


Final step. Build.
```
cargo build --target x86_64-unkown-linux-musl
```
