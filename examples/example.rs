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
    let opt = Opt::from_args_with_toml(toml_str).expect("toml parse failed");
    println!("a:{}", opt.a);
    println!("b:{}", opt.b);
}
