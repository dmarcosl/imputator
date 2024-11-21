use crate::util::common_csv::read_csv;
use crate::domain::imputation::Imputation;
use crate::IMPUTATIONS_FILE;
use csv::WriterBuilder;
use std::error::Error;
use std::fs::File;

pub(crate) async fn read_imputations() -> Result<Vec<Imputation>, Box<dyn Error>> {
    let mut rdr = read_csv(IMPUTATIONS_FILE).await?;

    let mut imputations = Vec::new();
    for result in rdr.deserialize() {
        let imputation: Imputation = result?;
        imputations.push(imputation);
    }

    Ok(imputations)
}

pub(crate) async fn update_imputation(row: i32, tempo_id: i64) -> Result<(), Box<dyn Error>> {
    let imputations = read_imputations().await?;

    let file = File::create(IMPUTATIONS_FILE)?;
    let mut wtr = WriterBuilder::new().has_headers(true).from_writer(file);

    let mut count: i32 = -1;
    for mut imputation in imputations {
        count += 1;
        if count == row {
            imputation.tempo_id = Option::from(tempo_id);
        }
        wtr.serialize(&imputation)?;
    }

    Ok(())
}

#[deprecated(note = "Replaced by `update_imputation` function.")]
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
        let dummy = Imputation::default();
        wtr.serialize(&dummy)?;
        wtr.flush()?;
    }

    Ok(())
}
