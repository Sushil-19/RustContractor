use ink_lang as ink;

#[ink::contract]
mod generic_contract {
    #[ink(storage)]
    pub struct GenericContract {
        my_struct: MyStruct,
    }

    impl GenericContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                my_struct: Default::default(),
            }
        }

        #[ink(message)]
        pub fn foo(u32) -> bool {
            // TODO: implement function
        }

        #[ink(message)]
        pub fn bar(u64) -> () {
            // TODO: implement function
        }

    }

    pub struct MyStruct {
        field1: u32,
        field2: bool,
    }

    impl MyStruct {
        pub fn get_field1(&self) -> u32 {
            self.field1
        }

        pub fn set_field1(&mut self, value: u32) {
            self.field1 = value;
        }

        pub fn get_field2(&self) -> bool {
            self.field2
        }

        pub fn set_field2(&mut self, value: bool) {
            self.field2 = value;
        }

    }

}
