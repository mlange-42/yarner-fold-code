# yarner-fold-code

[![Test status](https://github.com/mlange-42/yarner-fold-code/actions/workflows/tests.yml/badge.svg)](https://github.com/mlange-42/yarner-fold-code/actions/workflows/tests.yml)
[![GitHub](https://img.shields.io/badge/github-repo-blue?logo=github)](https://github.com/mlange-42/yarner-fold-code)
[![Crate](https://img.shields.io/crates/v/yarner-fold-code.svg)](https://crates.io/crates/yarner-fold-code)
[![MIT license](https://img.shields.io/github/license/mlange-42/yarner-fold-code)](https://github.com/mlange-42/yarner-fold-code/blob/main/LICENSE)

A [Yarner](https://github.com/mlange-42/yarner) plugin that puts all code blocks into collapsed `<details>` tags.

Example:

<table><tr><td>

All code blocks are collapsed in details tags.

<details><summary>Main code block</summary>

```rust
//- Main code block
fn main() {
    println!("Hello world!");
}
```
</details>
</td></tr></table>

## Installation

**Binaries**

1. Download the [latest binaries](https://github.com/mlange-42/yarner-fold-code/releases) for your platform
2. Unzip somewhere
3. Add the parent directory of the executable to your `PATH` environmental variable

**Using `cargo`**

```
> cargo install yarner-fold-code
```

## Usage

Add a section `plugin.fold-code` to your `Yarner.toml`:

```toml
[plugin.fold-code]
```

## Options

The plugin allows for different options, which are all optional:

```toml
[plugin.fold-code]
min-lines = "10"
languages = ["java", "rust"]
ignore-languages = ["c", "python"]
```

| Option             | Details                                               | Default |
|--------------------|-------------------------------------------------------|---------|
| `min-lines`        | Do not fold code blocks shorter than that             | `0`     |
| `languages`        | Only fold code blocks in these languages (if present) | none    |
| `ignore-languages` | Do not fold code blocks in these languages            | none    |
