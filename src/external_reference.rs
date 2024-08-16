use serde::{Serialize, Deserialize};

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalReference {
    #[serde(alias = "id")]
    code: String,
    origin: String,
}

impl ExternalReference {
    pub fn code(&self) -> String {
        self.code.clone()
    }

    pub fn origin(&self) -> String {
        self.origin.clone()
    }
}
