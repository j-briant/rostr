use rostr::Configuration;

struct Test {
    field1: String,
}

struct OtherTest {
    my_field: String,
}

fn run();

fn add();

fn remove();

fn list();

fn validate();

fn main() {
    let params;
    let configuration = Configuration::from(params);
    configuration.execute();
}
