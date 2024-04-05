pub mod process;
pub mod transfer;
pub mod uri;

use process::Process;

use serde_json;

#[derive(Debug)]
pub struct Configuration(Vec<Process>);

impl Configuration {
    pub fn load_str(s: &str) -> Result<Vec<Process>, serde_json::Error> {
        let conf: Vec<Process> = serde_json::from_str::<Vec<Process>>(s)?;
        Ok(conf)
    }

    pub fn get_process_by_id(&self, id: i32) -> Option<&Process> {
        self.0.iter().find(|p| p.id == id)
    }

    pub fn print_process(&self) -> () {
        println!("{:?}", self)
    }

    pub fn get_process_id_list(&self) -> Vec<i32> {
        self.0.iter().map(|p| p.id).collect()
    }
}
