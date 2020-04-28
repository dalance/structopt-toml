# structopt-toml
An default value loader from TOML for structopt.
It combinates with [structopt](https://github.com/TeXitoi/structopt).

[![Build Status](https://travis-ci.org/dalance/structopt-toml.svg?branch=master)](https://travis-ci.org/dalance/structopt-toml)
[![Crates.io](https://img.shields.io/crates/v/structopt-toml.svg)](https://crates.io/crates/structopt-toml)
[![Docs.rs](https://docs.rs/structopt-toml/badge.svg)](https://docs.rs/structopt-toml)
[![codecov](https://codecov.io/gh/dalance/structopt-toml/branch/master/graph/badge.svg)](https://codecov.io/gh/dalance/structopt-toml)

## Usage

This crate must be used with `serde`, `serde_derive`, `structopt`, and `toml` explicitly.

```Cargo.toml
[dependencies]
serde          = "1.0.104"
serde_derive   = "1.0.104"
structopt      = "0.3.11"
structopt-toml = "0.4.3"
toml           = "0.5.6"
```

## Example

If `derive(Deserialize)`, `derive(StructOptToml)` and `serde(default)` are added to the struct with `derive(StructOpt)`, some functions like `from_args_with_toml` can be used.

```rust
use serde_derive::Deserialize;
use structopt::StructOpt;
use structopt_toml::StructOptToml;

#[derive(Debug, Deserialize, StructOpt, StructOptToml)]
#[serde(default)]
struct Opt {
    #[structopt(default_value = "0", short = "a")] a: i32,
    #[structopt(default_value = "0", short = "b")] b: i32,
}

fn main() {
    let toml_str = r#"
        a = 10
    "#;
    let opt = Opt::from_args_with_toml(toml_str).expect("toml parse failed");
    println!("a:{}", opt.a);
    println!("b:{}", opt.b);
}
```

The execution result is below.

```console
$ ./example
a:10        // value from TOML string
b:0         // value from default_value of structopt

$ ./example -a 20
a:20        // value from command line argument
b:0
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
