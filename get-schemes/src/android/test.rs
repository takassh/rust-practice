use std::fs;


#[test]
pub fn get_schemes() {
    let contents = fs::read_to_string("./data/test/gradlew.txt").unwrap();
    let schemes = super::get_schemes(&contents);
    assert!(schemes.contains(&"develop".to_string()));
    assert!(schemes.contains(&"staging".to_string()));
    assert!(schemes.contains(&"production".to_string()));
    assert!(!schemes.contains(&"undefined".to_string()));
}