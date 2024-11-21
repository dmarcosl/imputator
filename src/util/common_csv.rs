use csv::{Reader, ReaderBuilder};
use std::error::Error;
use std::fs::File;

pub(crate) async fn read_csv(file_path: &str) -> Result<Reader<File>, Box<dyn Error>> {
    let file = File::open(file_path)?;

    let rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    Ok(rdr)
}
