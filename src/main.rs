use configuration::{Configuration, Process, Transfer};
use std::fs;

mod configuration;

fn main(){
    let path = "data/configuration.json";
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    let mut my_struct = Configuration::new(&contents).unwrap();

    println!("My complete configuration: {:#?}", my_struct);

    let process = my_struct.get_process(6);
    println!("My first process: {:#?}", process);

    let process_list = my_struct.get_process_by_list(vec![4,5,6,7,8,9]);
    //println!("My first 2 process:{:#?}", process_list);

    let removed_process = my_struct.remove_process(9);

    match removed_process {
        Some(p) =>  println!("REMOVED THE PROCESS: {:#?}", p),
        None => ()
    }

    println!("REMAINING: {:#?}", my_struct);

    for k in my_struct.get_process_by_list_as_mut(vec![1]).unwrap().iter_mut() {
        k.remove_transfer_by_list(vec![1,2,3]).unwrap();
    }
    //let tra = my_struct.get_process_by_list_as_mut(vec![1,2]).unwrap().iter().remove_transfer(1).unwrap();

    //println!("My removed transfer: {tra}");
    println!("My remaining transfers: {:#?}", my_struct);

    let save_path = "data/saved_config.json";

    my_struct.save(save_path).unwrap();

    let reload_contents = fs::read_to_string(save_path)
        .expect("Should have been able to read the file");
    let mut my_new_struct = Configuration::new(&reload_contents).unwrap();

    println!("MY RELOADED FILE CONTAINS: {:#?}", my_new_struct);

    let my_custom_string = r#"
    {
        "id": 2308947234,
        "parallel": true,
        "transfer": [
            {
                "id": 243562,
                "operation": "overwrite",
                "src": {
                    "path": "my/path/",
                    "option": ["TRUNCATE=1", "option3"]
                },
                "dst": {
                    "path": "my/destination",
                    "option": ["option2", "option3"]
                }
            }
        ]
    }
    "#;

    let custom_process: Process = my_custom_string.try_into().unwrap();

    println!("MY CUSTOM PROCESS: {:#?}", custom_process);

    my_struct.push(custom_process).unwrap();

    println!("MY LOADED CONFIGURATION: {:#?}", my_struct);

    let transfer_string = r#"{
        "id": 1,
        "operation": "overwrite",
        "src": {
            "path": "my/path/",
            "option": ["TRUNCATE=1", "option3"]
        },
        "dst": {
            "path": "my/destination",
            "option": ["option2", "option3"]
        }
    }"#;

    let custom_transfer: Transfer = transfer_string.try_into().unwrap();

    my_struct.get_process_as_mut(1).unwrap().push_transfer(custom_transfer).unwrap();


    println!("MY LOADED TRANSFER: {:#?}", my_struct);

}