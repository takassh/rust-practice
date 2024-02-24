use constructor_di::{
    dynamic_dispatch::Service as DynamicService, implementation::RepositoryImpl as DynamicImpl,
    implementation::RepositoryImpl as StaticImpl, static_dispatch::Service as StaticService,
};

struct Module {
    static_service: StaticService<StaticImpl>,
    dynamic_service: DynamicService,
}

// In this impl, resolve dependency
impl Module {
    pub fn new() -> Self {
        let repository = StaticImpl::new();
        let static_service = StaticService::new(repository);

        let repository = DynamicImpl::new();
        let dynamic_service = DynamicService::new(Box::new(repository));
        Self {
            static_service,
            dynamic_service,
        }
    }
}

fn main() {
    let module = Module::new();

    // Static DI (resolve dependency in compile time)
    let static_service = module.static_service;
    let _ = static_service.find_user(10);
    let _ = static_service.create_hello_user("john", 20);

    // Dynamic DI (resolve dependency in run time)
    let dynamic_service = module.dynamic_service;
    let _ = dynamic_service.find_user(10);
    let _ = dynamic_service.create_hello_user("john", 20);
}
