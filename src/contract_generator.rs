pub fn generate_contract(contract_name: &str, functions: Vec<&str>, structs: Vec<&str>) -> String {
    let mut contract = String::new();

    contract.push_str("use ink_lang as ink;\n\n");
    contract.push_str("#[ink::contract]\n");
    contract.push_str(&format!("mod {}_contract {{\n", contract_name));
    contract.push_str("    #[ink(storage)]\n");
    contract.push_str(&format!("    pub struct {}Contract {{\n", capitalize_first_letter(contract_name)));

    for struct_def in &structs {
        let struct_name = struct_def.split(':').next().unwrap().trim();
        contract.push_str(&format!("        {}: {},\n", to_snake_case(struct_name), struct_name));
    }

    contract.push_str("    }\n\n");

    contract.push_str(&format!("    impl {}Contract {{\n", capitalize_first_letter(contract_name)));
    contract.push_str("        #[ink(constructor)]\n");
    contract.push_str("        pub fn new() -> Self {\n");
    contract.push_str("            Self {\n");
    for struct_def in &structs {
        let struct_name = struct_def.split(':').next().unwrap().trim();
        contract.push_str(&format!("                {}: Default::default(),\n", to_snake_case(struct_name)));
    }
    contract.push_str("            }\n");
    contract.push_str("        }\n\n");

    for function in functions {
        contract.push_str(&format_function(function));
    }

    contract.push_str("    }\n\n");

    for struct_def in structs {
        contract.push_str(&format_struct(struct_def));
    }

    contract.push_str("}\n");

    contract
}

pub fn modify_contract(mut contract: String, functions: Vec<&str>, structs: Vec<&str>) -> String {
    for function in functions {
        contract = insert_at_end_of_impl(&contract, &format_function(function));
    }

    for struct_def in structs {
        contract = insert_before_last_brace(&contract, &format_struct(struct_def));
    }

    contract
}

fn format_function(function: &str) -> String {
    let parts: Vec<&str> = function.split("->").collect();
    let signature = parts[0].trim();
    let return_type = parts.get(1).map(|&s| s.trim()).unwrap_or("()");

    let sig_parts: Vec<&str> = signature.split(':').collect();
    let name = sig_parts[0].trim();
    let parameters = sig_parts.get(1).map(|&s| s.trim()).unwrap_or("");

    format!(
        "        #[ink(message)]\n        pub fn {name}({parameters}) -> {return_type} {{\n            // TODO: implement function\n        }}\n\n",
        name = name,
        parameters = parameters,
        return_type = return_type,
    )
}

fn format_struct(struct_def: &str) -> String {
    let parts: Vec<&str> = struct_def.split(':').collect();
    let name = parts[0].trim();
    let fields_str = parts.get(1).map(|&s| s.trim()).unwrap_or("");

    let mut struct_def = format!("    pub struct {} {{\n", name);
    let mut impl_def = format!("    impl {} {{\n", name);

    for field in fields_str.split(';') {
        let field_parts: Vec<&str> = field.split(',').collect();
        if field_parts.len() == 2 {
            let field_name = field_parts[0].trim();
            let field_type = field_parts[1].trim();
            struct_def.push_str(&format!("        {}: {},\n", field_name, field_type));

            impl_def.push_str(&format!(
                "        pub fn get_{}(&self) -> {} {{\n            self.{}\n        }}\n\n",
                field_name, field_type, field_name,
            ));

            impl_def.push_str(&format!(
                "        pub fn set_{}(&mut self, value: {}) {{\n            self.{} = value;\n        }}\n\n",
                field_name, field_type, field_name,
            ));
        }
    }

    struct_def.push_str("    }\n\n");
    impl_def.push_str("    }\n\n");

    struct_def + &impl_def
}

fn insert_at_end_of_impl(contract: &str, addition: &str) -> String {
    let impl_pos = contract.rfind("    }\n\n").unwrap_or(contract.len());
    let (start, end) = contract.split_at(impl_pos);
    format!("{}{}\n{}", start, addition, end)
}

fn insert_before_last_brace(contract: &str, addition: &str) -> String {
    let brace_pos = contract.rfind("}\n").unwrap_or(contract.len());
    let (start, end) = contract.split_at(brace_pos);
    format!("{}{}\n{}", start, addition, end)
}

fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn to_snake_case(s: &str) -> String {
    let mut snake_case = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            snake_case.push('_');
        }
        snake_case.push(ch.to_lowercase().next().unwrap());
    }
    snake_case
}
