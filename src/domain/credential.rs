use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Credential {
    pub(crate) user: String,
    pub(crate) pass: String,
}
