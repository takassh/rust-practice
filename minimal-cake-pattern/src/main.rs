use minimal_cake_pattern::{ProvidesRepository, ProvidesService, Repository, Service, UsesService};

pub struct Module {}

impl Module{
    pub fn new() -> Self {
        Self{}
    }
}

impl Repository for Module {}
impl Service for Module {}

impl ProvidesRepository for Module {
    type T = Self;

    fn user_repository(&self) -> &Self::T {
        self
    }
}

impl ProvidesService for Module {
    type T = Self;

    fn user_service(&self) -> &Self::T {
        self
    }
}

fn main() {
    let module = Module::new();
    let service = module.user_service();
    let _ = service.find_user(10);
    let _ = service.create_hello_user("john", 20);
}
