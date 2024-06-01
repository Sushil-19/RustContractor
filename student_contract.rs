use ink_lang as ink;

#[ink::contract]
mod my_contract_contract {
    #[ink(storage)]
    pub struct My_contractContract {
        student: Student,
    }

    impl My_contractContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                student: Default::default(),
            }
        }

        #[ink(message)]
        pub fn add_student(id,u32;name,String;grade,u8) -> () {
            // TODO: implement function
        }

        #[ink(message)]
        pub fn get_student(id,u32) -> Student {
            // TODO: implement function
        }

        #[ink(message)]
        pub fn update_grade(id,u32;grade,u8) -> () {
            // TODO: implement function
        }

    }

    pub struct Student {
        id: u32,
        name: String,
        grade: u8,
    }

    impl Student {
        pub fn get_id(&self) -> u32 {
            self.id
        }

        pub fn set_id(&mut self, value: u32) {
            self.id = value;
        }

        pub fn get_name(&self) -> String {
            self.name
        }

        pub fn set_name(&mut self, value: String) {
            self.name = value;
        }

        pub fn get_grade(&self) -> u8 {
            self.grade
        }

        pub fn set_grade(&mut self, value: u8) {
            self.grade = value;
        }

        #[ink(message)]
        pub fn new_function(arg1,u32) -> u32 {
            // TODO: implement function
        }


    }

    pub struct NewStruct {
        field1: u32,
        field2: String,
    }

    impl NewStruct {
        pub fn get_field1(&self) -> u32 {
            self.field1
        }

        pub fn set_field1(&mut self, value: u32) {
            self.field1 = value;
        }

        pub fn get_field2(&self) -> String {
            self.field2
        }

        pub fn set_field2(&mut self, value: String) {
            self.field2 = value;
        }

    }


}
