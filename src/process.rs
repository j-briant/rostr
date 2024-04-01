pub mod transfer;

use transfer::Transfer;

use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::fmt::Display;

/// Core object of the module, Process holds validated fields.
#[derive(Deserialize, Debug, PartialEq)]
pub struct Process {
    pub id: i32,
    parallel: bool,
    transfer: Transfer,
}

impl Process {
    fn load_str(s: &str) -> Result<Process, serde_json::Error> {
        let conf: Process = serde_json::from_str::<Process>(s)?;
        Ok(conf)
    }
}

#[derive(Debug)]
enum ProcessError {
    OperationError,
    ConnectionError,
}

impl Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ProcessError::OperationError => write!(
                f,
                "Given operation is invalid, must be one of 'overwrite', 'update' or 'upsert'"
            ),
            ProcessError::ConnectionError => {
                write!(f, "Dataset not reachable, please check the given URI")
            }
        }
    }
}

impl Error for ProcessError {}
