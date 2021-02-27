# yarner-fold-code

A [Yarner](https://github.com/mlange-42/yarner) pre-processor that puts all code blocks into collapsed `<details>` tags.

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
> cargo install --git https://github.com/mlange-42/yarner-fold-code.git --branch main
```

## Usage

Add a section `preprocessor.fold-code` to your `Yarner.toml`:

```toml
[preprocessor.fold-code]
```
