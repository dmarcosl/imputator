use crate::domain::credential::Credential;
use crate::CREDENTIALS_FILE;
use std::error::Error;
use crate::util::common_csv::read_csv;

pub(crate) async fn read_credentials() -> Result<Vec<Credential>, Box<dyn Error>> {
    let mut rdr = read_csv(CREDENTIALS_FILE).await?;

    let mut credentials = Vec::new();
    for result in rdr.deserialize() {
        let credential: Credential = result?;
        credentials.push(credential);
    }

    Ok(credentials)
}
