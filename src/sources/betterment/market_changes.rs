use std::path::PathBuf;

use chrono::NaiveDate;
use clap::Clap;
use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct InputRecord {
    #[serde(rename = "Date")]
    date: NaiveDate,

    #[serde(rename = "Market Change Amount")]
    amount: f32,
}

#[derive(Debug, Serialize)]
struct OutputRecord {
    #[serde(rename = "Date")]
    date: NaiveDate,

    #[serde(rename = "Payee")]
    payee: &'static str,

    #[serde(rename = "Memo")]
    memo: String,

    #[serde(rename = "Amount")]
    amount: f32,
}

impl From<&InputRecord> for OutputRecord {
    fn from(input_record: &InputRecord) -> Self {
        Self {
            date: input_record.date,
            payee: "Betterment",
            memo: format!(
                "Market Changes for {}",
                input_record.date.format("%A, %-d %B, %C%y")
            ),
            amount: input_record.amount,
        }
    }
}

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

        let mut reader = Reader::from_path(&self.input_file).expect("csv reader");
        let mut writer = Writer::from_path(&self.output_file).expect("csv writer");

        for result in reader.deserialize() {
            let input_record: InputRecord = result.expect("csv input record");
            let output_record = OutputRecord::from(&input_record);
            println!("{:?} -> {:?}", input_record, output_record);
            writer.serialize(output_record).expect("csv output record");
        }
    }
}
