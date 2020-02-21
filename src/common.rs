pub(crate) use crate::{error::Error, opt::Opt};
pub(crate) use std::num::ParseIntError;
pub(crate) use structopt::{clap::AppSettings, StructOpt};

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;
