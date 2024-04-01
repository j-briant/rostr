pub mod process;
use process::Process;

use serde_json;

#[derive(Debug)]
struct Configuration(Vec<Process>);

impl Configuration {
    pub fn load_str(s: &str) -> Result<Vec<Process>, serde_json::Error> {
        let conf: Vec<Process> = serde_json::from_str::<Vec<Process>>(s)?;
        Ok(conf)
    }

    pub fn get_process_by_id(&self, id: i32) -> Vec<Process> {
        self.0
            .iter()
            .map(|p| *p)
            .filter(|proc| proc.id == id)
            .collect()
    }

    pub fn print_process(&self) -> () {
        println!("{:?}", self)
    }

    pub fn get_id_list(&self) -> Vec<i32> { vec![0] }
}
