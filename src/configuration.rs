use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use thiserror::Error;

/// A data Source passed to GDAL to read the data. Can be vector or raster. Corresponds to the configuration structure.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Source {
    pub path: String,
    pub option: Vec<String>,
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Source ({}, {:?})", self.path, self.option)
    }
}

/// A data Destination passed to GDAL to send the data. Can be vector or raster.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Destination {
    pub path: String,
    pub option: Vec<String>,
}

impl Display for Destination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Destination ({}, {:?})", self.path, self.option)
    }
}

/// A list of operations supported. Mainly used to validate the configuration.
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Operation {
    Overwrite,
    Update,
    Upsert,
}

/// The central objet. A Transfer represents a single table transfer between a Source and a Destination.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Transfer {
    pub id: u32,
    pub operation: Operation,
    pub src: Source,
    pub dst: Destination,
}

impl Display for Transfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Transfer ({}, {:?}, {}, {})",
            self.id, self.operation, self.src, self.dst
        )
    }
}

impl TryFrom<&str> for Transfer {
    type Error = serde_json::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let t: Transfer = serde_json::from_str::<Transfer>(&value)?;
        Ok(t)
    }
}

/// Represents a higher level of abstraction above Transfers. A Process groups multiple Transfers and can be configured to multithread the execution with the `parallel` field.
/// It's up to the user to decide what should be in a Process and when to divide between two.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Process {
    pub id: u32,
    pub parallel: bool,
    pub transfer: Vec<Transfer>,
}

/// Errors concerning a Process instance manipulation. As a Process is aware of its fields and not other Processes, these errors will often be linked to a Transfer.
#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("the transfer `{0}` can't be pushed")]
    PushError(Transfer),
}

impl Display for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Process ({}, {}, {:#?})",
            self.id, self.parallel, self.transfer
        )
    }
}

impl TryFrom<&str> for Process {
    type Error = serde_json::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let proc: Process = serde_json::from_str::<Process>(&value)?;
        Ok(proc)
    }
}

impl Process {
    /// Create a new instance of a Process from a `&str`. The preferred creation method should be through the `TryFrom` trait `try_into` function.
    ///
    /// # Example
    /// ```
    /// use rostr::configuration::Process;
    /// let ps = r#"
    ///{
    ///    "id": 2308,
    ///    "parallel": true,
    ///    "transfer": [
    ///        {
    ///            "id": 2432,
    ///            "operation": "overwrite",
    ///            "src": {
    ///                "path": "my/path/",
    ///                "option": ["TRUNCATE=1", "option3"]
    ///            },
    ///            "dst": {
    ///                "path": "my/destination",
    ///                "option": ["option2", "option3"]
    ///            }
    ///        }
    ///    ]
    ///}"#;
    ///
    /// let p = Process::new(&ps);
    /// ```
    pub fn new(s: &str) -> Result<Self, serde_json::Error> {
        let proc: Process = serde_json::from_str::<Process>(s)?;
        Ok(proc)
    }

    /// Push a Transfer to the transfer list of a Process instance.
    ///
    /// # Example
    ///
    /// ```
    /// use rostr::configuration::{Process, Transfer};
    ///
    /// let ts = r#"
    /// {
    ///     "id": 2432,
    ///     "operation": "overwrite",
    ///     "src": {
    ///         "path": "my/path/",
    ///         "option": ["TRUNCATE=1", "option3"]
    ///     },
    ///     "dst": {
    ///         "path": "my/destination",
    ///         "option": ["option2", "option3"]
    ///     }
    ///}
    /// "#;
    ///
    /// let t: Transfer = ts.try_into().unwrap();
    ///
    /// let ps = r#"
    ///{
    ///    "id": 2308,
    ///    "parallel": true,
    ///    "transfer": [
    ///        {
    ///            "id": 2432,
    ///            "operation": "overwrite",
    ///            "src": {
    ///                "path": "my/path/",
    ///                "option": ["TRUNCATE=1", "option3"]
    ///            },
    ///            "dst": {
    ///                "path": "my/destination",
    ///                "option": ["option2", "option3"]
    ///            }
    ///        }
    ///    ]
    ///}"#;
    ///
    /// let mut p = Process::new(&ps).unwrap();
    ///
    /// p.push_transfer(t);
    ///
    ///
    /// ```
    pub fn push_transfer(&mut self, p: Transfer) -> Result<(), ProcessError> {
        self.transfer.push(p);
        Ok(())
    }

    /// Get a Transfer from the current Process using its id field and returning a immutable reference.
    ///
    /// # Example
    /// ```
    /// use rostr::configuration::{Process, Transfer};
    /// 
    /// let ps = r#"
    /// {
    ///   "id": 2308,
    ///  "parallel": true,
    ///  "transfer": [
    ///      {
    ///          "id": 2432,
    ///          "operation": "overwrite",
    ///          "src": {
    ///              "path": "my/path/",
    ///              "option": ["TRUNCATE=1", "option3"]
    ///          },
    ///          "dst": {
    ///              "path": "my/destination",
    ///              "option": ["option2", "option3"]
    ///          }
    ///      }
    ///  ]
    /// }"#;
    /// 
    /// let p = Process::new(&ps).unwrap();
    ///
    /// let t = p.get_transfer(2432).unwrap();
    ///
    /// assert_eq!(t.id, 2432);
    /// ```
    pub fn get_transfer(&self, id: u32) -> Option<&Transfer> {
        self.transfer.iter().find(|&p| p.id == id)
    }

    /// Get a Transfer from the current Process using its id field and returning a mutable reference.
    ///
    /// # Example
    /// ```
    /// use rostr::configuration::{Process, Transfer};
    /// 
    /// let ps = r#"
    /// {
    ///   "id": 2308,
    ///  "parallel": true,
    ///  "transfer": [
    ///      {
    ///          "id": 2432,
    ///          "operation": "overwrite",
    ///          "src": {
    ///              "path": "my/path/",
    ///              "option": ["TRUNCATE=1", "option3"]
    ///          },
    ///          "dst": {
    ///              "path": "my/destination",
    ///              "option": ["option2", "option3"]
    ///          }
    ///      }
    ///  ]
    /// }"#;
    /// 
    /// let mut p = Process::new(&ps).unwrap();
    ///
    /// let mut t = p.get_transfer_as_mut(2432).unwrap();
    ///
    /// assert_eq!(t.id, 2432);
    /// 
    /// t.id = 1;
    /// 
    /// assert_eq!(t.id, 1);
    /// 
    /// ```
    pub fn get_transfer_as_mut(&mut self, id: u32) -> Option<&mut Transfer> {
        self.transfer.iter_mut().find(|p| p.id == id)
    }

    pub fn get_transfer_by_list(&self, id_list: Vec<u32>) -> Option<Vec<&Transfer>> {
        Some(
            self.transfer
                .iter()
                .filter(|&p| id_list.contains(&p.id))
                .collect(),
        )
    }

    pub fn get_transfer_by_list_as_mut(&mut self, id_list: Vec<u32>) -> Option<Vec<&mut Transfer>> {
        Some(
            self.transfer
                .iter_mut()
                .filter(|p| id_list.contains(&p.id))
                .collect(),
        )
    }

    pub fn remove_transfer(&mut self, id: u32) -> Option<Transfer> {
        if let Some(idx) = self.transfer.iter().position(|p| p.id == id) {
            Some(self.transfer.swap_remove(idx))
        } else {
            None
        }
    }

    pub fn remove_transfer_by_list(&mut self, id_list: Vec<u32>) -> Option<()> {
        self.transfer.retain(|p| !id_list.contains(&p.id));
        Some(())
    }
}

#[derive(Error, Debug)]
pub enum ConfigurationError {
    #[error("the process `{0}` can't be pushed")]
    PushError(Process),
    #[error("the configuration can't be written to file")]
    WriteError,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(transparent)]
pub struct Configuration {
    pub process: Vec<Process>,
}

impl Configuration {
    pub fn new(s: &str) -> Result<Self, serde_json::Error> {
        let conf: Configuration = serde_json::from_str::<Configuration>(s)?;
        Ok(conf)
    }

    pub fn push(&mut self, p: Process) -> Result<(), ConfigurationError> {
        self.process.push(p);
        Ok(())
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        let output = serde_json::to_string_pretty(&self)?;
        write!(file, "{}", output)
    }

    pub fn get_process(&self, id: u32) -> Option<&Process> {
        self.process.iter().find(|&p| p.id == id)
    }

    pub fn get_process_as_mut(&mut self, id: u32) -> Option<&mut Process> {
        self.process.iter_mut().find(|p| p.id == id)
    }

    pub fn get_process_by_list(&self, id_list: Vec<u32>) -> Option<Vec<&Process>> {
        Some(
            self.process
                .iter()
                .filter(|&p| id_list.contains(&p.id))
                .collect(),
        )
    }

    pub fn get_process_by_list_as_mut(&mut self, id_list: Vec<u32>) -> Option<Vec<&mut Process>> {
        Some(
            self.process
                .iter_mut()
                .filter(|p| id_list.contains(&p.id))
                .collect(),
        )
    }

    pub fn remove_process(&mut self, id: u32) -> Option<Process> {
        if let Some(idx) = self.process.iter().position(|p| p.id == id) {
            Some(self.process.swap_remove(idx))
        } else {
            None
        }
    }

    pub fn remove_process_by_list(&mut self, id_list: Vec<u32>) -> Option<()> {
        self.process.retain(|p| !id_list.contains(&p.id));
        Some(())
    }
}
