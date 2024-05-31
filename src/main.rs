mod token_contract_generator;

use clap::{Arg, Command};
use std::io::{self, Write};

fn main() {
    let matches = Command::new("Smart Contract Generator")
        .version("1.0")
        .author("Your Name <you@example.com>")
        .about("Generates an ink! smart contract template")
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .value_name("TYPE")
                .help("Type of contract (student or token)")
                .num_args(1),
        )
        .arg(
            Arg::new("functions")
                .short('f')
                .long("functions")
                .value_name("NUM")
                .help("Number of functions")
                .num_args(1),
        )
        .arg(
            Arg::new("structs")
                .short('s')
                .long("structs")
                .value_name("NUM")
                .help("Number of structs")
                .num_args(1),
        )
        .get_matches();

    let contract_type = matches
        .get_one::<String>("type")
        .unwrap_or(&"student".to_string())
        .to_lowercase();

    if contract_type == "token" {
        let contract = token_contract_generator::generate_token_contract();
        let mut file = std::fs::File::create("token_contract.rs").expect("Could not create file");
        file.write_all(contract.as_bytes()).expect("Could not write to file");
        println!("Token smart contract generated successfully in token_contract.rs");
    } else {
        let num_structs: usize = matches
            .get_one::<String>("structs")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let num_functions: usize = matches
            .get_one::<String>("functions")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let mut structs = Vec::new();
        for i in 1..=num_structs {
            println!("Enter name for struct {}:", i);
            let mut struct_name = String::new();
            io::stdin().read_line(&mut struct_name).expect("Failed to read line");
            let struct_name = struct_name.trim().to_string();

            println!("Enter fields for struct {} (e.g., x: u32, y: String):", i);
            let mut struct_fields = String::new();
            io::stdin().read_line(&mut struct_fields).expect("Failed to read line");
            let struct_fields = struct_fields.trim().to_string();

            structs.push((struct_name, struct_fields));
        }

        let mut functions = Vec::new();
        for i in 1..=num_functions {
            println!("Enter name for function {}:", i);
            let mut func_name = String::new();
            io::stdin().read_line(&mut func_name).expect("Failed to read line");
            let func_name = func_name.trim().to_string();

            println!("Enter parameters and return type for function {} (e.g., x: i32, y: String -> ReturnType):", i);
            let mut func_signature = String::new();
            io::stdin().read_line(&mut func_signature).expect("Failed to read line");
            let func_signature = func_signature.trim().to_string();

            functions.push((func_name, func_signature));
        }

        let contract = generate_student_contract(structs, functions);
        let mut file = std::fs::File::create("student_contract.rs").expect("Could not create file");
        file.write_all(contract.as_bytes()).expect("Could not write to file");

        println!("Student smart contract generated successfully in student_contract.rs");
    }
}

fn generate_student_contract(structs: Vec<(String, String)>, functions: Vec<(String, String)>) -> String {
    let mut contract = String::new();

    contract.push_str("use ink_lang as ink;\n\n");
    contract.push_str("#[ink::contract]\n");
    contract.push_str("mod student_contract {\n");

    for (struct_name, struct_fields) in structs {
        contract.push_str("    #[ink(storage)]\n");
        contract.push_str(&format!("    pub struct {} {{\n", struct_name));
        for field in struct_fields.split(",") {
            let field = field.trim();
            contract.push_str(&format!("        {},\n", field));
        }
        contract.push_str("    }\n\n");
    }

    contract.push_str("    impl MyContract {\n");
    contract.push_str("        #[ink(constructor)]\n");
    contract.push_str("        pub fn new(init_value: bool) -> Self {\n");
    contract.push_str("            Self { value: init_value }\n");
    contract.push_str("        }\n\n");

    contract.push_str("        #[ink(message)]\n");
    contract.push_str("        pub fn get(&self) -> bool {\n");
    contract.push_str("            self.value\n");
    contract.push_str("        }\n\n");

    contract.push_str("        #[ink(message)]\n");
    contract.push_str("        pub fn flip(&mut self) {\n");
    contract.push_str("            self.value = !self.value;\n");
    contract.push_str("        }\n\n");

    for (func_name, func_signature) in functions {
        let parts: Vec<&str> = func_signature.split("->").collect();
        let params = parts[0].trim();
        let return_type = if parts.len() > 1 {
            parts[1].trim().to_string()
        } else {
            "()".to_string()
        };

        contract.push_str(&format!("        #[ink(message)]\n"));
        contract.push_str(&format!("        pub fn {}({}) -> {} {{\n", func_name, params, return_type));
        contract.push_str("            // TODO: Implement function logic\n");
        contract.push_str("        }\n\n");
    }

    contract.push_str("    }\n\n");

    contract.push_str("    #[cfg(test)]\n");
    contract.push_str("    mod tests {\n");
    contract.push_str("        use super::*;\n\n");

    contract.push_str("        #[ink::test]\n");
    contract.push_str("        fn default_works() {\n");
    contract.push_str("            let contract = MyContract::new(false);\n");
    contract.push_str("            assert_eq!(contract.get(), false);\n");
    contract.push_str("        }\n\n");

    contract.push_str("        #[ink::test]\n");
    contract.push_str("        fn it_works() {\n");
    contract.push_str("            let mut contract = MyContract::new(false);\n");
    contract.push_str("            assert_eq!(contract.get(), false);\n");
    contract.push_str("            contract.flip();\n");
    contract.push_str("            assert_eq!(contract.get(), true);\n");
    contract.push_str("        }\n");

    contract.push_str("    }\n");
    contract.push_str("}\n");

    contract
}
