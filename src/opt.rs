use crate::common::*;

#[derive(StructOpt)]
#[structopt(
    name = "qc",
    global_settings = &[AppSettings::NoBinaryName]
)]
pub(crate) struct Opt {
    #[structopt(long)]
    pub(crate) verbose: bool,
}
