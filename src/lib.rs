pub mod uri;

use std::error::Error;
use std::fmt::Display;

use serde::Deserialize;
use serde_json;

use gdal::Dataset;

// Core object of the module, Configuration holds validated fields.
#[derive(Deserialize, Debug, PartialEq)]
struct Configuration {
    operation: Operation,
    src_dataset: SourceURI,
    dst_dataset: String,
}

impl Configuration {
    fn from_str(s: &str) -> Result<Configuration, serde_json::Error> {
        let conf: Configuration = serde_json::from_str::<Configuration>(s)?;
        Ok(conf)
    }
}

// Error type for the module.
#[derive(Debug)]
enum ConfigurationError {
    OperationError,
    ConnectionError,
}

impl Display for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ConfigurationError::OperationError => write!(
                f,
                "Given operation is invalid, must be one of 'overwrite', 'update' or 'upsert'"
            ),
            ConfigurationError::ConnectionError => {
                write!(f, "Dataset not reachable, please check the given URI")
            }
        }
    }
}

impl Error for ConfigurationError {}

// Operations are the valid GDAL/OGR operation possible on a dataset (update, overwrite etc.)
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Operation {
    Overwrite,
    Update,
    Upsert,
}

impl Operation {
    fn from_str(s: &str) -> Result<Operation, ConfigurationError> {
        match s {
            "overwrite" => Ok(Operation::Overwrite),
            "update" => Ok(Operation::Update),
            "upsert" => Ok(Operation::Upsert),
            _ => Err(ConfigurationError::OperationError),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
struct SourceURI {
    path: String,
}

impl SourceURI {
    fn from_str(p: &str) -> Result<SourceURI, ConfigurationError> {
        let opt = gdal::DatasetOptions {
            open_flags: gdal::GdalOpenFlags::GDAL_OF_ALL,
            allowed_drivers: Some(&["PostgreSQL"]),
            ..Default::default()
        };
        match gdal::Dataset::open_ex(p, opt) {
            Ok(d) => Ok(SourceURI {
                path: p.to_string(),
            }),
            Err(e) => Err(ConfigurationError::ConnectionError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection() {
        let path = "PG:dbname=osm_suisse tables=planet_osm_line";
        let uri = SourceURI::from_str(&path).unwrap();
        assert_eq!(uri.path, path);
    }

    #[test]
    fn configuration_from_str() {
        let input = r#"
            {
                "operation": "update",
                "src_dataset": "PG:dbname=osm_suisse tables=planet_osm_line",
                "dst_dataset": "localhost::dst_dataset"
            }"#;
        let uri = SourceURI::from_str("PG:dbname=osm_suisse tables=planet_osm_line").unwrap();
        assert_eq!(
            Configuration::from_str(&input).unwrap(),
            Configuration {
                operation: Operation::Update,
                src_dataset: uri,
                dst_dataset: String::from("localhost:dst_dataset"),
            }
        );
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
        Configuration::from_str(&input).unwrap();
    }
}
