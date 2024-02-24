use std::i32;

use anyhow::Result;

pub trait Repository {
    fn find(&self, id: i32) -> Result<()>;
    fn create(&self, name: &str, age: u32, arg: &str) -> Result<()>;
}

pub struct Service<R: Repository> {
    repository: R,
}

impl<R: Repository> Service<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn find_user(&self, id: i32) -> Result<()> {
        self.repository.find(id)
    }

    pub fn create_hello_user(&self, name: &str, age: u32) -> Result<()> {
        self.repository.create(name, age, "static hello")
    }
}
