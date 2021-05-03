mod activity;
mod market_changes;

use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(author, setting = AppSettings::SubcommandRequiredElseHelp)]
/// Convert CSV exports from Betterment investment accounts to YNAB's CSV format
pub(crate) enum Subcommand {
    Activity(activity::Opts),
    MarketChanges(market_changes::Opts),
}

impl Subcommand {
    pub(crate) fn run(&self) {
        match self {
            Subcommand::Activity(cmd) => cmd.run(),
            Subcommand::MarketChanges(cmd) => cmd.run(),
        }
    }
}
