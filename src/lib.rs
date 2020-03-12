//! `structopt-toml` is an default value loader from TOML for structopt.
//!
//! ## Examples
//!
//! The following example show a quick example of `structopt-toml`.
//!
//! If `derive(Deserialize)`, `derive(StructOptToml)` and `serde(default)` are added to the struct
//! with `derive(StructOpt)`, some functions like `from_args_with_toml` can be used.
//!
//! ```
//! #[macro_use]
//! extern crate serde_derive;
//! #[macro_use]
//! extern crate structopt;
//! #[macro_use]
//! extern crate structopt_toml;
//! extern crate toml;
//!
//! use structopt::StructOpt;
//! use structopt_toml::StructOptToml;
//!
//! #[derive(Debug, Deserialize, StructOpt, StructOptToml)]
//! #[serde(default)]
//! struct Opt {
//!     #[structopt(default_value = "0", short = "a")] a: i32,
//!     #[structopt(default_value = "0", short = "b")] b: i32,
//! }
//!
//! fn main() {
//!     let toml_str = r#"
//!         a = 10
//!     "#;
//!     let opt = Opt::from_args_with_toml(toml_str).expect("toml parse failed");
//!     println!("a:{}", opt.a);
//!     println!("b:{}", opt.b);
//! }
//! ```
//!
//! The execution result of the above example is below.
//!
//! ```ignore
//! $ ./example
//! a:10        // value from TOML string
//! b:0         // value from default_value of structopt
//!
//! $ ./example -a 20
//! a:20        // value from command line argument
//! b:0
//! ```

extern crate clap as _clap;
extern crate failure;
extern crate serde as _serde;
extern crate structopt as _structopt;
extern crate toml as _toml;

#[allow(unused_imports)]
#[macro_use]
extern crate structopt_toml_derive;

#[doc(hidden)]
pub use structopt_toml_derive::*;

use std::ffi::OsString;

/// Re-export of clap
pub mod clap {
    pub use _clap::*;
}
/// Re-export of serde
pub mod serde {
    pub use _serde::*;
}
/// Re-export of structopt
pub mod structopt {
    pub use _structopt::*;
}

pub trait StructOptToml {
    /// Merge the struct from TOML and the struct from args
    fn merge<'a>(from_toml: Self, from_args: Self, args: &_clap::ArgMatches) -> Self
    where
        Self: Sized,
        Self: _structopt::StructOpt,
        Self: _serde::de::Deserialize<'a>;

    /// Creates the struct from `clap::ArgMatches` with initial values from TOML.
    fn from_clap_with_toml<'a>(
        toml_str: &'a str,
        args: &_clap::ArgMatches,
    ) -> Result<Self, failure::Error>
    where
        Self: Sized,
        Self: _structopt::StructOpt,
        Self: _serde::de::Deserialize<'a>,
    {
        let from_args: Self = _structopt::StructOpt::from_clap(&args);
        let from_toml: Self = _toml::from_str(toml_str)?;
        Ok(Self::merge(from_toml, from_args, &args))
    }

    /// Creates the struct from command line arguments with initial values from TOML.
    fn from_args_with_toml<'a>(toml_str: &'a str) -> Result<Self, failure::Error>
    where
        Self: Sized,
        Self: _structopt::StructOpt,
        Self: _serde::de::Deserialize<'a>,
    {
        let clap = Self::clap();
        let args = clap.get_matches();
        Self::from_clap_with_toml(toml_str, &args)
    }

    /// Creates the struct from iterator with initial values from TOML.
    fn from_iter_with_toml<'a, I>(toml_str: &'a str, iter: I) -> Result<Self, failure::Error>
    where
        Self: Sized,
        Self: _structopt::StructOpt,
        Self: _serde::de::Deserialize<'a>,
        I: IntoIterator,
        I::Item: Into<OsString> + Clone,
    {
        let clap = Self::clap();
        let args = clap.get_matches_from(iter);
        Self::from_clap_with_toml(toml_str, &args)
    }
}
