// pub mod uri;

use std::error::Error;
use std::fmt;
// use gdal::DatasetOptions;

use serde::Deserialize;

/// Operations are the valid GDAL/OGR operation possible on a dataset (update, overwrite etc.)
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Operation {
    Overwrite,
    Update,
    Upsert,
}

/// SourceURI is a GDAL validated connection to a dataset with DatasetOptions.
#[derive(Debug, Deserialize, PartialEq)]
pub struct SourceURI {
    path: String,
}

#[derive(Debug)]
enum TransferError {
    ParseError(serde_json::Error),
}

impl fmt::Display for TransferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransferError::ParseError(e) => write!(f, "{}", e),
        }
    }
}

impl Error for TransferError {
    // Implement this to return the lower level source of this Error.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TransferError::ParseError(e) => Some(e),
        }
    }
}

impl From<serde_json::Error> for TransferError {
    fn from(e: serde_json::Error) -> Self {
        TransferError::ParseError(e)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Transfer {
    id: i32,
    operation: Operation,
    src: SourceURI,
    dst: String,
}

impl Transfer {
    pub fn from_str(s: &str) -> Result<Transfer, TransferError> {
        let t: Transfer = serde_json::from_str::<Transfer>(s)?;
        Ok(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        // Missing src_dataset
        let input = r#"
            {
                "id": 0,
                "operation": "update",
                "src": "PG:dbname=osm_suisse table=planet_osm_line"
                "dst": "localhost::dst_dataset"
            }"#;
        Transfer::from_str(&input).unwrap();
    }

    #[test]
    #[should_panic]
    fn err_on_missing_field() {
        // Missing src_dataset
        let input = "
            {
                \"operation\": \"update\",
                \"dst_dataset\": \"localhost::dst_dataset\"
            }";
        Transfer::from_str(&input).unwrap();
    }
}
