#![deny(clippy::all, clippy::cargo)]

mod sources;

use clap::Clap;

fn main() {
    sources::Subcommand::parse().run();
}
