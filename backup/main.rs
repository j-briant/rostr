use std::path::PathBuf;
use clap::{arg, Command};


fn run() {
    todo!()
}

fn add(){
    todo!()
}

fn remove(){
    todo!()
}

fn list(){
    todo!()
}

fn validate(){
    todo!()
}

fn cli() -> Command {
    Command::new("rostr")
        .about("CLI for moving geospatialized data around")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("list")
                .about("List processes")
                .arg(arg!(<REMOTE> "The remote to clone"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("add")
                .about("Add a new process")
                .arg_required_else_help(true)
                .arg(arg!(<PATH> ... "Stuff to add").value_parser(clap::value_parser!(PathBuf))),
        )
        .subcommand(
            Command::new("remove")
                .args_conflicts_with_subcommands(true)
                .flatten_help(true)
                .args(push_args())
                .subcommand(Command::new("push").args(push_args()))
                .subcommand(Command::new("pop").arg(arg!([STASH])))
                .subcommand(Command::new("apply").arg(arg!([STASH]))),
        )
}

fn push_args() -> Vec<clap::Arg> {
    vec![arg!(-m --message <MESSAGE>)]
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let paths = sub_matches
                .get_many::<PathBuf>("PATH")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Adding {paths:?}");
        }
        Some(("stash", sub_matches)) => {
            let stash_command = sub_matches.subcommand().unwrap_or(("push", sub_matches));
            match stash_command {
                ("apply", sub_matches) => {
                    let stash = sub_matches.get_one::<String>("STASH");
                    println!("Applying {stash:?}");
                }
                ("pop", sub_matches) => {
                    let stash = sub_matches.get_one::<String>("STASH");
                    println!("Popping {stash:?}");
                }
                ("push", sub_matches) => {
                    let message = sub_matches.get_one::<String>("message");
                    println!("Pushing {message:?}");
                }
                (name, _) => {
                    unreachable!("Unsupported subcommand `{name}`")
                }
            }
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}