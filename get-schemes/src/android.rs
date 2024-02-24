#[cfg(test)]
pub mod test;

use regex::Regex;
use std::collections::HashSet;

pub fn get_schemes(output: &str) -> Vec<String> {
    let mut variants = HashSet::new();
    for task in output.split("\n") {
        let assemble_task_pattern = Regex::new(r"(assemble)(\S+)").unwrap();
        let caps = assemble_task_pattern.captures(task);
        let Some(caps) = caps else {
            continue;
        };
        let Some(capture) = caps.get(2) else {
            continue;
        };
        let variant = capture.as_str().to_lowercase();
        if !variant.ends_with("test") {
            variants.insert(variant);
        }
    }

    let mut product_flavors = HashSet::new();
    for variant1 in variants.iter() {
        for variant2 in variants.iter() {
            if !(variant2.starts_with(variant1) && variant1 != variant2) {
                continue;
            }
            let build_type = variant2.clone().split_off(variant1.len());
            if variants.contains(&build_type) {
                product_flavors.insert(variant1.clone());
            }
        }
    }

    product_flavors
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}
