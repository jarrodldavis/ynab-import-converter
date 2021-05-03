use std::path::PathBuf;

use clap::Clap;

#[derive(Clap)]
#[clap(author)]
/// Convert Market Changes CSV exports from Betterment's Performance tab to
/// YNAB's CSV format
pub(crate) struct Opts {
    #[clap(long, short, parse(from_os_str), default_value = "market-changes.csv")]
    /// The CSV export to convert
    ///
    /// This converter expects to be working with the `market-changes.csv` file
    /// downloaded from the Balance section of the Performance tab
    /// in Betterment.
    input_file: PathBuf,

    #[clap(
        long,
        short,
        parse(from_os_str),
        default_value = "market-changes_ynab.csv"
    )]
    /// Where to save the converted CSV document to be imported into YNAB
    output_file: PathBuf,
}

impl Opts {
    pub(crate) fn run(&self) {
        println!("Betterment -> Investment -> Market Changes");
        println!("input: {:?}", self.input_file);
        println!("output: {:?}", self.output_file);
    }
}
