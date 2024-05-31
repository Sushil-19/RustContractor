// token_contract_generator.rs

use std::io::{self, Write};

pub fn generate_token_contract() -> String {
    let mut contract = String::new();

    contract.push_str("use ink_lang as ink;\n\n");
    contract.push_str("#[ink::contract]\n");
    contract.push_str("mod token_contract {\n");
    contract.push_str("    #[ink(storage)]\n");
    contract.push_str("    pub struct TokenContract {\n");
    contract.push_str("        total_supply: Balance,\n");
    contract.push_str("        balances: ink::storage::Mapping<AccountId, Balance>,\n");
    contract.push_str("    }\n\n");

    contract.push_str("    impl TokenContract {\n");
    contract.push_str("        #[ink(constructor)]\n");
    contract.push_str("        pub fn new(initial_supply: Balance) -> Self {\n");
    contract.push_str("            let caller = Self::env().caller();\n");
    contract.push_str("            let mut balances = ink::storage::Mapping::new();\n");
    contract.push_str("            balances.insert(&caller, &initial_supply);\n");
    contract.push_str("            Self { total_supply: initial_supply, balances }\n");
    contract.push_str("        }\n\n");

    contract.push_str("        #[ink(message)]\n");
    contract.push_str("        pub fn total_supply(&self) -> Balance {\n");
    contract.push_str("            self.total_supply\n");
    contract.push_str("        }\n\n");

    contract.push_str("        #[ink(message)]\n");
    contract.push_str("        pub fn balance_of(&self, owner: AccountId) -> Balance {\n");
    contract.push_str("            self.balances.get(&owner).unwrap_or_default()\n");
    contract.push_str("        }\n\n");

    contract.push_str("        #[ink(message)]\n");
    contract.push_str("        pub fn transfer(&mut self, to: AccountId, value: Balance) -> bool {\n");
    contract.push_str("            let from = self.env().caller();\n");
    contract.push_str("            let from_balance = self.balances.get(&from).unwrap_or_default();\n");
    contract.push_str("            if from_balance < value {\n");
    contract.push_str("                return false;\n");
    contract.push_str("            }\n");
    contract.push_str("            self.balances.insert(&from, &(from_balance - value));\n");
    contract.push_str("            let to_balance = self.balances.get(&to).unwrap_or_default();\n");
    contract.push_str("            self.balances.insert(&to, &(to_balance + value));\n");
    contract.push_str("            true\n");
    contract.push_str("        }\n");
    contract.push_str("    }\n\n");

    contract.push_str("    #[cfg(test)]\n");
    contract.push_str("    mod tests {\n");
    contract.push_str("        use super::*;\n\n");

    contract.push_str("        #[ink::test]\n");
    contract.push_str("        fn new_works() {\n");
    contract.push_str("            let initial_supply = 1000;\n");
    contract.push_str("            let contract = TokenContract::new(initial_supply);\n");
    contract.push_str("            assert_eq!(contract.total_supply(), initial_supply);\n");
    contract.push_str("            assert_eq!(contract.balance_of(contract.env().caller()), initial_supply);\n");
    contract.push_str("        }\n\n");

    contract.push_str("        #[ink::test]\n");
    contract.push_str("        fn transfer_works() {\n");
    contract.push_str("            let initial_supply = 1000;\n");
    contract.push_str("            let mut contract = TokenContract::new(initial_supply);\n");
    contract.push_str("            let receiver = AccountId::from([0x1; 32]);\n");
    contract.push_str("            assert!(contract.transfer(receiver, 500));\n");
    contract.push_str("            assert_eq!(contract.balance_of(contract.env().caller()), 500);\n");
    contract.push_str("            assert_eq!(contract.balance_of(receiver), 500);\n");
    contract.push_str("        }\n");
    contract.push_str("    }\n");
    contract.push_str("}\n");

    contract
}
