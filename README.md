# Smart Contract Generator

![Rust](https://img.shields.io/badge/Rust-1.55.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Contributions](https://img.shields.io/badge/contributions-welcome-brightgreen)

## Overview

Smart Contract Generator is a command-line tool that assists in generating templates for Ink! smart contracts written in Rust. This tool helps you quickly scaffold smart contracts for different purposes such as student management or token handling.

## Features

- **Student Management Contracts**: Generate smart contracts for managing students, including custom structs and functions.
- **Token Contracts**: Generate token-based smart contracts with basic functionality like balance checks and transfers.
- **Customization**: Specify the number of structs and functions to include in the generated contract.
- **Automated Code Generation**: Automatically generates getter and setter methods for struct fields.

## Installation

To install Smart Contract Generator, you'll need to have Rust installed on your machine. You can install Rust by following the instructions [here](https://www.rust-lang.org/tools/install).

Clone the repository and navigate into the project directory:

```sh
git clone https://github.com/yourusername/smart-contract-generator.git
cd smart-contract-generator
```

## Build the project:
```sh
cargo build --release
```

## Usage
### Generating a Student Management Contract
To generate a student management smart contract, run the following command and follow the prompts:
```sh
cargo run -- --type student -f <NUM_FUNCTIONS> -s <NUM_STRUCTS>
```
Replace <NUM_FUNCTIONS> and <NUM_STRUCTS> with the desired number of functions and structs, respectively.

### Generating a Token Contract
To generate a token-based smart contract, run:

```sh
cargo run -- --type token
```

The generated contract files will be created in the project directory.

## Examples
### Student Management Contract
After running:
```sh
cargo run -- --type student -f 2 -s 1
```

You might be prompted to enter details like this:

```sh
Enter name for struct 1:
Student
Enter fields for struct 1 (e.g., id: u32, name: String):
id: u32, name: String, grade: u8
Enter name for function 1:
add_student
Enter parameters and return type for function 1 (e.g., id: u32, name: String -> ()):
id: u32, name: String, grade: u8 -> ()
Enter name for function 2:
get_student
Enter parameters and return type for function 2 (e.g., id: u32 -> Student):
id: u32 -> Student
```

### Token Contract
Simply run:
```sh
cargo run -- --type token
```
Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

Fork the repository.
Create a new branch (git checkout -b feature-branch).
Commit your changes (git commit -am 'Add new feature').
Push to the branch (git push origin feature-branch).
Create a new Pull Request.
License
This project is licensed under the MIT License - see the LICENSE file for details.

Contact
If you have any questions or feedback, feel free to reach out via GitHub Issues or contact me directly at dubesushil99@gmail.com.

Acknowledgements
ink! - eDSL to write smart contracts in Rust for Substrate-based blockchains.
