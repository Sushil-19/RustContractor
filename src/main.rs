mod contract_generator;
mod token_contract_generator;

use clap::{Arg, Command};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn main() {
    let matches = Command::new("Smart Contract Generator")
        .version("1.0")
        .author("Your Name <you@example.com>")
        .about("Generates and modifies ink! smart contract templates")
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .value_name("TYPE")
                .help("Type of contract (e.g., student, token)")
                .num_args(1),
        )
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .value_name("NAME")
                .help("Name of the contract to modify")
                .num_args(1),
        )
        .arg(
            Arg::new("functions")
                .short('f')
                .long("functions")
                .value_name("FUNCTIONS")
                .help("Functions to add in the format 'name:parameters->return_type'")
                .num_args(1..),
        )
        .arg(
            Arg::new("structs")
                .short('s')
                .long("structs")
                .value_name("STRUCTS")
                .help("Structs to add in the format 'name:fields'")
                .num_args(1..),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT")
                .help("Output file name")
                .required(true)
                .num_args(1),
        )
        .get_matches();

    let contract_type = matches.get_one::<String>("type").map(String::as_str).unwrap_or("generic");
    let contract_name = matches.get_one::<String>("name").map(String::as_str).unwrap_or("my_contract");
    let functions = matches.get_many::<String>("functions").unwrap_or_default().map(|s| s.as_str()).collect::<Vec<&str>>();
    let structs = matches.get_many::<String>("structs").unwrap_or_default().map(|s| s.as_str()).collect::<Vec<&str>>();
    let output_file = matches.get_one::<String>("output").expect("Output file is required");

    let mut contract = read_contract(output_file);

    if contract.is_empty() {
        contract = match contract_type {
            "token" => token_contract_generator::generate_token_contract("Token", "TKN", 18),
            _ => contract_generator::generate_contract(contract_name, functions.clone(), structs.clone()),
        };
    } else {
        contract = contract_generator::modify_contract(contract, functions, structs);
    }

    // Write the contract to the output file
    let mut file = File::create(output_file).expect("Could not create file");
    file.write_all(contract.as_bytes()).expect("Could not write to file");

    println!("Contract written to {}", output_file);
}

fn read_contract(file_path: &str) -> String {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => return String::new(),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read file");
    contents
}
