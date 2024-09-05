# Better embedded

[![Crate](https://img.shields.io/crates/v/better_embedded.svg)](https://crates.io/crates/better_embedded)
[![GitHub last commit](https://img.shields.io/github/last-commit/xuxiaocheng0201/better_embedded)](https://github.com/xuxiaocheng0201/better_embedded/commits/master)
[![GitHub issues](https://img.shields.io/github/issues-raw/xuxiaocheng0201/better_embedded)](https://github.com/xuxiaocheng0201/better_embedded/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/xuxiaocheng0201/better_embedded)](https://github.com/xuxiaocheng0201/better_embedded/pulls)
[![GitHub](https://img.shields.io/github/license/xuxiaocheng0201/better_embedded)](https://github.com/xuxiaocheng0201/better_embedded/blob/master/LICENSE)


# Description

Enables you to embed file and release it at runtime.


# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
better_embedded = "~0.1"
```


# Example

```rust,no_run
fn initialize() -> std::io::Result<()> {
    better_embedded::release_file(include_bytes!("data/file.txt"), "file.txt");
    Ok(())
}
```


# License

This project is licensed under either of

Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
