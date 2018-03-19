extern crate clap as _clap;
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
    fn from_clap_with_toml<'a>(toml_str: &'a str, args: &_clap::ArgMatches) -> Self
    where
        Self: Sized,
        Self: _structopt::StructOpt,
        Self: _serde::de::Deserialize<'a>,
    {
        let from_args: Self = _structopt::StructOpt::from_clap(&args);
        let from_toml: Self = _toml::from_str(toml_str).unwrap();
        Self::merge(from_toml, from_args, &args)
    }

    /// Creates the struct from command line arguments with initial values from TOML.
    fn from_args_with_toml<'a>(toml_str: &'a str) -> Self
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
    fn from_iter_with_toml<'a, I>(toml_str: &'a str, iter: I) -> Self
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
