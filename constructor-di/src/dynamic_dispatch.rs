use std::i32;

use anyhow::Result;

pub trait Repository {
    fn find(&self, id: i32) -> Result<()>;
    fn create(&self, name: &str, age: u32, arg: &str) -> Result<()>;
}

pub struct Service {
    repository: Box<dyn Repository>,
}

impl Service {
    pub fn new(repository: Box<dyn Repository>) -> Self {
        Self { repository }
    }

    pub fn find_user(&self, id: i32) -> Result<()> {
        self.repository.find(id)
    }

    pub fn create_hello_user(&self, name: &str, age: u32) -> Result<()> {
        self.repository.create(name, age, "dynamic hello")
    }
}
