use anyhow::Result;
pub trait Repository {}

pub trait UsesRepository {
    fn find(&self, id: i32) -> Result<()>;
    fn create(&self, name: &str, age: u32, arg: &str) -> Result<()>;
}

impl<Repository> UsesRepository for Repository {
    fn find(&self, id: i32) -> Result<()> {
        println!("find user id:{}", id);
        Ok(())
    }

    fn create(&self, name: &str, age: u32, arg: &str) -> Result<()> {
        println!("created user name:{} age:{} with arg:{}", name, age, arg);
        Ok(())
    }
}

pub trait ProvidesRepository {
    type T: UsesRepository;
    fn user_repository(&self) -> &Self::T;
}

pub trait Service: ProvidesRepository {}

pub trait UsesService {
    fn find_user(&self, id: i32) -> Result<()>;
    fn create_hello_user(&self, name: &str, age: u32) -> Result<()>;
}

impl<T: Service> UsesService for T {
    fn find_user(&self, id: i32) -> Result<()> {
        self.user_repository().find(id)
    }

    fn create_hello_user(&self, name: &str, age: u32) -> Result<()> {
        self.user_repository().create(name, age, "hello")
    }
}

pub trait ProvidesService {
    type T: UsesService;
    fn user_service(&self) -> &Self::T;
}
