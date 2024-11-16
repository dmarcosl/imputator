use crate::{CREDENTIALS_FILE, IMPUTATIONS_FILE};
use csv::{Reader, ReaderBuilder, WriterBuilder};
use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Imputation {
    pub(crate) user: String,
    pub(crate) day: String,
    pub(crate) issue: String,
    pub(crate) description: String,
    pub(crate) time: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Credential {
    pub(crate) user: String,
    pub(crate) pass: String,
}

async fn read_csv(file_path: &str) -> Result<Reader<File>, Box<dyn Error>> {
    let file = File::open(file_path)?;

    let rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    Ok(rdr)
}

pub(crate) async fn read_imputations() -> Result<Vec<Imputation>, Box<dyn Error>> {
    let mut rdr = read_csv(IMPUTATIONS_FILE).await?;

    let mut imputations = Vec::new();
    for result in rdr.deserialize() {
        let imputation: Imputation = result?;
        imputations.push(imputation);
    }

    Ok(imputations)
}

pub(crate) async fn remove_imputation(row: i32) -> Result<(), Box<dyn Error>> {
    let imputations = read_imputations().await?;

    let file = File::create(IMPUTATIONS_FILE)?;
    let mut wtr = WriterBuilder::new().has_headers(true).from_writer(file);

    let mut count: i32 = -1;
    for imputation in imputations {
        count += 1;
        if count == row {
            continue;
        }
        wtr.serialize(&imputation)?;
    }

    // If the file is empty, write a dummy row to keep the headers
    if row == 0 && count == 0 {
        let dummy = Imputation {
            user: String::new(),
            day: String::new(),
            issue: String::new(),
            description: String::new(),
            time: String::new(),
        };
        wtr.serialize(&dummy)?;
        wtr.flush()?;
    }

    Ok(())
}

pub(crate) async fn read_credentials() -> Result<Vec<Credential>, Box<dyn Error>> {
    let mut rdr = read_csv(CREDENTIALS_FILE).await?;

    let mut credentials = Vec::new();
    for result in rdr.deserialize() {
        let credential: Credential = result?;
        credentials.push(credential);
    }

    Ok(credentials)
}
