pub fn generate_token_contract(name: &str, symbol: &str, decimals: u8) -> String {
    let mut contract = String::new();

    contract.push_str("use ink_lang as ink;\n\n");
    contract.push_str("#[ink::contract]\n");
    contract.push_str("mod token_contract {\n");
    contract.push_str("    #[ink(storage)]\n");
    contract.push_str(&format!("    pub struct {}Contract {{\n", capitalize_first_letter(name)));
    contract.push_str("        total_supply: Balance,\n");
    contract.push_str("        balances: ink_storage::collections::HashMap<AccountId, Balance>,\n");
    contract.push_str("        allowances: ink_storage::collections::HashMap<(AccountId, AccountId), Balance>,\n");
    contract.push_str("    }\n\n");

    contract.push_str("    impl TokenContract {\n");
    contract.push_str("        #[ink(constructor)]\n");
    contract.push_str(&format!(
        "        pub fn new(initial_supply: Balance) -> Self {{\n"
    ));
    contract.push_str("            let caller = Self::env().caller();\n");
    contract.push_str("            let mut balances = ink_storage::collections::HashMap::new();\n");
    contract.push_str("            balances.insert(caller, initial_supply);\n");
    contract.push_str("            Self {\n");
    contract.push_str("                total_supply: initial_supply,\n");
    contract.push_str("                balances,\n");
    contract.push_str("                allowances: Default::default(),\n");
    contract.push_str("            }\n");
    contract.push_str("        }\n\n");

    contract.push_str("        #[ink(message)]\n");
    contract.push_str("        pub fn total_supply(&self) -> Balance {\n");
    contract.push_str("            self.total_supply\n");
    contract.push_str("        }\n\n");

    contract.push_str("        #[ink(message)]\n");
    contract.push_str("        pub fn balance_of(&self, owner: AccountId) -> Balance {\n");
    contract.push_str("            *self.balances.get(&owner).unwrap_or(&0)\n");
    contract.push_str("        }\n\n");

    contract.push_str("        #[ink(message)]\n");
    contract.push_str("        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<(), &'static str> {\n");
    contract.push_str("            let from = self.env().caller();\n");
    contract.push_str("            let from_balance = self.balance_of(from);\n");
    contract.push_str("            if from_balance < value {\n");
    contract.push_str("                return Err(\"Insufficient balance\");\n");
    contract.push_str("            }\n");
    contract.push_str("            self.balances.insert(from, from_balance - value);\n");
    contract.push_str("            let to_balance = self.balance_of(to);\n");
    contract.push_str("            self.balances.insert(to, to_balance + value);\n");
    contract.push_str("            Ok(())\n");
    contract.push_str("        }\n");
    contract.push_str("    }\n\n");

    contract.push_str("}\n");

    contract
}

fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
