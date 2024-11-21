use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Imputation {
    pub(crate) tempo_id: Option<i64>,
    pub(crate) user: String,
    pub(crate) day: String,
    pub(crate) issue: String,
    pub(crate) description: String,
    pub(crate) time: String,
}

impl Imputation {
    pub fn compare(&self, other: &Imputation) -> bool {
        self.tempo_id == other.tempo_id
            && self.user.to_lowercase().trim() == other.user.to_lowercase().trim()
            && self.day.to_lowercase().trim() == other.day.to_lowercase().trim()
            && self.issue.to_lowercase().trim() == other.issue.to_lowercase().trim()
            && self.description.to_lowercase().trim() == other.description.to_lowercase().trim()
            && self.time.to_lowercase().trim() == other.time.to_lowercase().trim()
    }

    pub fn default() -> Imputation {
        Imputation {
            tempo_id: None,
            user: String::new(),
            day: String::new(),
            issue: String::new(),
            description: String::new(),
            time: String::new(),
        }
    }
}
