use serde::{Deserialize, Serialize};

use crate::{external_reference::ExternalReference, position::Position};

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Destination {
    #[serde(default)]
    id: String,
    #[serde(alias = "displayName")]
    pub name: String,
    #[serde(alias = "location")]
    pub position: Position,
    #[serde(alias = "shortDescription", default)]
    description: String,
    #[serde(alias = "externalReferences", default)]
    external_references: Vec<ExternalReference>,
    #[serde(default)]
    platform: Option<String>,
    #[serde(default)]
    categories: Vec<Category>,
}

impl Destination {
    pub fn get_nsr_code(&self) -> String {
        for reference in self.external_references.iter() {
            if reference.origin() == "NSR" {
                return reference.code().to_owned();
            }
        }
        "NO_NSR_CODE".to_owned()
    }

    pub fn get_platform(&self) -> &str {
        match &self.platform {
            None => "No platform found",
            Some(res) => res,
        }
    }
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize)]
struct Category {
    id: String,
    #[serde(default)]
    name: String,
}
