# RustContractor

## Overview

The Smart Contract Generator is a Rust-based CLI tool designed to simplify the creation and modification of ink! smart contracts for the Polkadot ecosystem. The tool allows you to generate smart contract templates and add custom functions and structs, streamlining the development process.

## Features

- Generate ink! smart contract templates
- Add custom functions and structs via CLI
- Modify existing smart contracts
- Support for different contract types (e.g., student, token)

## Installation

To install the Smart Contract Generator, ensure you have Rust and Cargo installed. Clone the repository and build the project:

```sh
git clone https://github.com/yourusername/smart-contract-generator.git
cd smart-contract-generator
cargo build --release
```

# Usage
## Generate a New Contract
To generate a new smart contract, use the following command:

```sh
cargo run -- --type <contract_type> --structs <structs> --functions <functions> --output <output_file>
```

<contract_type>: Type of contract (e.g., student, token)
<structs>: Structs to add in the format 'name:fields'
<functions>: Functions to add in the format 'name:parameters->return_type'
<output_file>: Name of the output file

## Example
Generate a student contract:

```sh
cargo run -- --type student --structs "Student:id,u32;name,String;grade,u8" --functions "add_student:id,u32;name,String;grade,u8->()" "get_student:id,u32->Student" "update_grade:id,u32;grade,u8->()" --output student_contract.rs
```

## Modify an Existing Contract
To modify an existing contract, use the following command:

```sh
cargo run -- --name <contract_name> --functions <functions> --structs <structs> --output <output_file>
```

<contract_name>: Name of the contract to modify
<functions>: Functions to add in the format 'name:parameters->return_type'
<structs>: Structs to add in the format 'name:fields'
<output_file>: Name of the output file
Example
Modify an existing student contract to add new functions or structs:

```sh
cargo run -- --name student_contract --functions "new_function:arg1,u32->u32" --structs "NewStruct:field1,u32;field2,String" --output student_contract.rs
```

File Structure
The project structure is as follows:

```sh
smart-contract-generator/
├── src/
│   ├── main.rs
│   ├── contract_generator.rs
│   └── token_contract_generator.rs
├── .gitignore
├── Cargo.toml
└── README.md
```

main.rs: Main entry point for the CLI tool
contract_generator.rs: Contains the logic for generating and modifying contracts
token_contract_generator.rs: Contains the logic for generating token contracts
.gitignore: Specifies files and directories to be ignored by Git
Cargo.toml: Rust package configuration
README.md: Project documentation
Contributing
Contributions are welcome! Please fork the repository and submit a pull request with your changes. Ensure that your code adheres to the project's coding standards and includes appropriate tests.

License
This project is licensed under the MIT License. See the LICENSE file for details.

Contact
For questions or feedback, please reach out to dubesushil99@gmail.com.

