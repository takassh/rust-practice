use flutter_rust_bridge::frb;

pub struct CustomType {
    pub name: String,
}

impl CustomType {
    #[frb(sync)]
    pub fn new(name: String) -> CustomType {
        CustomType { name }
    }
}

#[frb(sync)]
pub fn create() -> CustomType {
    CustomType {
        name: "a".to_string(),
    }
}
