# structopt-toml
An initial value loader from TOML for structopt.
It combinates with [structopt](https://github.com/TeXitoi/structopt).

[![Build Status](https://travis-ci.org/dalance/structopt-toml.svg?branch=master)](https://travis-ci.org/dalance/structopt-toml)

## Usage

```Cargo.toml
[dependencies]
structopt-toml = "0.1.0"
```

## Example

You should add `derive(Deserialize)`, `derive(StructOptToml)` and `serde(default)` to the struct with `derive(StructOpt)`.
So, you can use functions like `from_args_with_toml` which load initial values from TOML string.

```rust:example.rs
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate structopt_toml;
extern crate toml;

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
    let opt = Opt::from_args_with_toml(toml_str);
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
