<h1 align="center">
    Game of Life using webgpu
</h1>

## Setup

You need:

- [Rust](https://www.rust-lang.org/) compiler and [Cargo package manager](https://doc.rust-lang.org/cargo/)
- One of the [supported backends](https://github.com/gfx-rs/wgpu#supported-platforms) by wgpu crate.

## 🚀 Getting started

```
$ git clone https://github.com/pythops/golrs
$ cd golrs/
$ cargo run
```

## ⚙️ Configuration

You can specify the grid size with `size` argument (the default value is `32`)

```
$ cargo run -- --size <grid size: u16>

```

## License

AGPLv3
