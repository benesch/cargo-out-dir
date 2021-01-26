# cargo-out-dir

A Cargo subcommand to print the ["out directory"][out-dir], where build script output
is written.

# Installation

```
cargo install cargo-out-dir
```

# Usage

```
$ cd path/to/crate
$ cargo out-dir
/home/benesch/path/to/crate/target/debug/build/foo-9d9f15d3d084bc3a/out
```

# License

Licensed under the [Apache License, version 2.0](LICENSE).

[out-dir]: https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#out-dir
