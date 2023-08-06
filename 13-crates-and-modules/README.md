# 13. Creates and Modules

The Rust compiler is called `rustc`.

Rust code is build into a crate.

The Rust package manager is called `cargo`.

## Modules

Rust code is organized into modules.

A module can be defined via the `mod` keyword:

```rust
mod geo {
    pub fn do_something() {}
}
```

or by putting code in its own file, in which case the filename is the module name.

## Cargo.toml

The metadata file used by the Cargo package manager.

## What Does Cargo Do?

- Building the code
- Project configuration
- Dependency management
- Publishing to repository

- `cargo build` - builds and compiles the code
- `cargo clean` - cleans the `target` directory, which is where the build artifacts are stored
- `cargo check` - checks that the needed dependencies are there
- `cargo rustdoc` - generates documentation from the code comments
- `cargo test` - runs our tests
- `cargo new` - creates a new project
- `cargo init` - creates a new project in the same directory
- `cargo package` - generate the crate for the current version of the program

## Publishing Crates

Crate - a compressed archive that holds the source files of the program.

The `cargo publish` command published the crate (to [crates.io](https://crates.io) by default).

The `cargo yank --vers <version_to_remove> <package_name>` command deletes a crate from the registry.
