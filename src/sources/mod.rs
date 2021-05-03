pub(crate) mod betterment;

use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(
    author, about, version,
    global_setting = AppSettings::ColoredHelp,
    global_setting = AppSettings::DeriveDisplayOrder,
    setting = AppSettings::SubcommandRequiredElseHelp,
)]
pub(crate) enum Subcommand {
    Betterment(betterment::Subcommand),
}

impl Subcommand {
    pub(crate) fn run(&self) {
        match self {
            Subcommand::Betterment(cmd) => cmd.run(),
        }
    }
}
