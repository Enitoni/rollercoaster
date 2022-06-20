# Rollercoaster

[![Build](https://github.com/Enitoni/rollercoaster/actions/workflows/test.yml/badge.svg)](https://github.com/Enitoni/rollercoaster/actions/workflows/test.yml)
[![docs.rs](https://img.shields.io/docsrs/rollercoaster)](https://docs.rs/rollercoaster/)
[![Crate](https://img.shields.io/crates/v/rollercoaster.svg)](https://crates.io/crates/rollercoaster)

This crate adds extra iterators, with extension methods so you can access them from any type implementing [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html).

## Usage

Add `Rollercoaster` to your `Cargo.toml`

```toml
[dependencies]
rollercoaster = "0.0.2"
```

Import the trait into your module

```rust
use rollercoaster::Rollercoaster;

fn main() {
  let iter = vec![1, 2, 3].into_iter();

  // The methods are now available for us to use
  iter.memory();
}
```

## Contributing

Please create a github issue if you run into any problems.
