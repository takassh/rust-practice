pub struct RepositoryImpl {}

impl RepositoryImpl {
    pub fn new() -> Self {
        Self {}
    }
}

pub mod static_implementation {
    use crate::static_dispatch::Repository;
    use super::RepositoryImpl;
    use anyhow::{Ok, Result};

    impl Repository for RepositoryImpl {
        fn find(&self, id: i32) -> Result<()> {
            println!("find user id:{}", id);
            Ok(())
        }

        fn create(&self, name: &str, age: u32, arg: &str) -> Result<()> {
            println!("created user name:{} age:{} with arg:{}", name, age, arg);
            Ok(())
        }
    }
}

pub mod dynamic_implementation {
    use crate::dynamic_dispatch::Repository;
    use super::RepositoryImpl;
    use anyhow::{Ok, Result};

    impl Repository for RepositoryImpl {
        fn find(&self, id: i32) -> Result<()> {
            println!("find user id:{}", id);
            Ok(())
        }

        fn create(&self, name: &str, age: u32, arg: &str) -> Result<()> {
            println!("created user name:{} age:{} with arg:{}", name, age, arg);
            Ok(())
        }
    }
}