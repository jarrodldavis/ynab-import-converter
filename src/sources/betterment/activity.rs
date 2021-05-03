use std::path::PathBuf;

use clap::Clap;

#[derive(Clap)]
#[clap(author)]
/// Convert CSV exports from Betterment's Activity tab to YNAB's CSV format
pub(crate) struct Opts {
    #[clap(long, short, parse(from_os_str), default_value = "transactions.csv")]
    /// The CSV export to convert
    ///
    /// This converter expects to be working with the `transactions.csv` file
    /// downloaded from the Activity tab in Betterment.
    input_file: PathBuf,

    #[clap(
        long,
        short,
        parse(from_os_str),
        default_value = "transactions_ynab.csv"
    )]
    /// Where to save the converted CSV document to be imported into YNAB
    output_file: PathBuf,
}

impl Opts {
    pub(crate) fn run(&self) {
        println!("Betterment -> Investment -> Activity");
        println!("input: {:?}", self.input_file);
        println!("output: {:?}", self.output_file);
    }
}
